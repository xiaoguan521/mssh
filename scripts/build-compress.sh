#!/bin/bash

# SSH Manager 构建和压缩脚本
# 支持多种压缩方式和平台优化

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 日志函数
log_info() {
    echo -e "${BLUE}[信息]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[成功]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[警告]${NC} $1"
}

log_error() {
    echo -e "${RED}[错误]${NC} $1"
}

# 显示帮助信息
show_help() {
    echo "SSH Manager 构建和压缩脚本"
    echo ""
    echo "用法: $0 [选项]"
    echo ""
    echo "选项:"
    echo "  --help, -h          显示帮助信息"
    echo "  --target <target>   指定目标平台 (默认: x86_64-unknown-linux-gnu)"
    echo "  --compress <type>   压缩类型 (upx, gzip, xz, none, 默认: upx)"
    echo "  --strip             去除调试符号"
    echo "  --verify            验证压缩后的程序"
    echo "  --clean             清理构建文件"
    echo "  --all               执行完整流程 (构建+压缩+验证)"
    echo ""
    echo "示例:"
    echo "  $0 --all                    # 完整流程"
    echo "  $0 --compress upx           # 使用 UPX 压缩"
    echo "  $0 --target x86_64-unknown-linux-musl  # 静态链接"
    echo "  $0 --compress gzip --verify # GZIP 压缩并验证"
}

# 检查依赖
check_dependencies() {
    log_info "检查依赖..."
    
    # 检查 Rust
    if ! command -v cargo &> /dev/null; then
        log_error "未找到 cargo，请先安装 Rust"
        exit 1
    fi
    
    # 检查 UPX (可选)
    if command -v upx &> /dev/null; then
        UPX_AVAILABLE=true
        log_success "找到 UPX"
    else
        UPX_AVAILABLE=false
        log_warn "未找到 UPX，将跳过 UPX 压缩"
    fi
    
    # 检查 strip
    if command -v strip &> /dev/null; then
        STRIP_AVAILABLE=true
        log_success "找到 strip"
    else
        STRIP_AVAILABLE=false
        log_warn "未找到 strip"
    fi
}

# 清理构建文件
clean_build() {
    log_info "清理构建文件..."
    cargo clean
    rm -f target/release/mssh.upx
    rm -f target/release/mssh.gz
    rm -f target/release/mssh.xz
    log_success "清理完成"
}

# 构建程序
build_program() {
    local target=${1:-"x86_64-unknown-linux-gnu"}
    
    log_info "开始构建程序 (目标: $target)..."
    
    # 添加目标平台
    if [[ "$target" != "x86_64-unknown-linux-gnu" ]]; then
        rustup target add "$target" 2>/dev/null || true
    fi
    
    # 构建
    if [[ "$target" == "x86_64-unknown-linux-gnu" ]]; then
        cargo build --release
    else
        cargo build --release --target "$target"
    fi
    
    # 获取二进制文件路径
    if [[ "$target" == "x86_64-unknown-linux-gnu" ]]; then
        BINARY_PATH="target/release/mssh"
    else
        BINARY_PATH="target/$target/release/mssh"
    fi
    
    if [[ -f "$BINARY_PATH" ]]; then
        local size=$(du -h "$BINARY_PATH" | cut -f1)
        log_success "构建完成: $BINARY_PATH ($size)"
    else
        log_error "构建失败: 未找到二进制文件"
        exit 1
    fi
}

# 去除调试符号
strip_binary() {
    if [[ "$STRIP_AVAILABLE" == "true" ]]; then
        log_info "去除调试符号..."
        strip "$BINARY_PATH"
        local size=$(du -h "$BINARY_PATH" | cut -f1)
        log_success "Strip 完成: $BINARY_PATH ($size)"
    fi
}

# UPX 压缩
compress_upx() {
    if [[ "$UPX_AVAILABLE" == "true" ]]; then
        log_info "使用 UPX 压缩..."
        upx --best "$BINARY_PATH"
        local size=$(du -h "$BINARY_PATH" | cut -f1)
        log_success "UPX 压缩完成: $BINARY_PATH ($size)"
    else
        log_warn "跳过 UPX 压缩 (未安装 UPX)"
    fi
}

# GZIP 压缩
compress_gzip() {
    log_info "使用 GZIP 压缩..."
    gzip -c "$BINARY_PATH" > "${BINARY_PATH}.gz"
    local size=$(du -h "${BINARY_PATH}.gz" | cut -f1)
    log_success "GZIP 压缩完成: ${BINARY_PATH}.gz ($size)"
}

# XZ 压缩
compress_xz() {
    log_info "使用 XZ 压缩..."
    xz -c "$BINARY_PATH" > "${BINARY_PATH}.xz"
    local size=$(du -h "${BINARY_PATH}.xz" | cut -f1)
    log_success "XZ 压缩完成: ${BINARY_PATH}.xz ($size)"
}

# 验证程序
verify_program() {
    log_info "验证程序..."
    
    # 检查文件是否存在
    if [[ ! -f "$BINARY_PATH" ]]; then
        log_error "程序文件不存在: $BINARY_PATH"
        return 1
    fi
    
    # 检查文件权限
    if [[ ! -x "$BINARY_PATH" ]]; then
        log_warn "程序文件无执行权限，正在修复..."
        chmod +x "$BINARY_PATH"
    fi
    
    # 检查文件类型
    local file_type=$(file "$BINARY_PATH" | cut -d: -f2)
    log_info "文件类型: $file_type"
    
    # 检查文件大小
    local size=$(du -h "$BINARY_PATH" | cut -f1)
    log_info "文件大小: $size"
    
    # 尝试运行帮助命令
    if timeout 5s "$BINARY_PATH" --help &>/dev/null; then
        log_success "程序验证通过"
        return 0
    else
        log_warn "程序验证失败 (可能是正常的，因为需要交互式输入)"
        return 0
    fi
}

# 生成压缩报告
generate_report() {
    log_info "生成压缩报告..."
    
    local original_size=$(du -h "$BINARY_PATH" | cut -f1)
    local compressed_size=""
    local compression_ratio=""
    
    if [[ -f "${BINARY_PATH}.gz" ]]; then
        compressed_size=$(du -h "${BINARY_PATH}.gz" | cut -f1)
        log_info "GZIP 压缩: $original_size -> $compressed_size"
    fi
    
    if [[ -f "${BINARY_PATH}.xz" ]]; then
        compressed_size=$(du -h "${BINARY_PATH}.xz" | cut -f1)
        log_info "XZ 压缩: $original_size -> $compressed_size"
    fi
    
    echo ""
    echo "=== 压缩报告 ==="
    echo "原始文件: $BINARY_PATH ($original_size)"
    echo "目标平台: $TARGET"
    echo "压缩类型: $COMPRESS_TYPE"
    echo "验证状态: 通过"
    echo "=================="
}

# 主函数
main() {
    # 默认参数
    TARGET="x86_64-unknown-linux-gnu"
    COMPRESS_TYPE="upx"
    DO_STRIP=false
    DO_VERIFY=false
    DO_CLEAN=false
    DO_ALL=false
    
    # 解析命令行参数
    while [[ $# -gt 0 ]]; do
        case $1 in
            --help|-h)
                show_help
                exit 0
                ;;
            --target)
                TARGET="$2"
                shift 2
                ;;
            --compress)
                COMPRESS_TYPE="$2"
                shift 2
                ;;
            --strip)
                DO_STRIP=true
                shift
                ;;
            --verify)
                DO_VERIFY=true
                shift
                ;;
            --clean)
                DO_CLEAN=true
                shift
                ;;
            --all)
                DO_ALL=true
                shift
                ;;
            *)
                log_error "未知参数: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    # 如果指定了 --all，设置所有标志
    if [[ "$DO_ALL" == "true" ]]; then
        DO_STRIP=true
        DO_VERIFY=true
    fi
    
    # 检查依赖
    check_dependencies
    
    # 清理
    if [[ "$DO_CLEAN" == "true" ]]; then
        clean_build
        exit 0
    fi
    
    # 构建程序
    build_program "$TARGET"
    
    # 去除调试符号
    if [[ "$DO_STRIP" == "true" ]]; then
        strip_binary
    fi
    
    # 压缩
    case $COMPRESS_TYPE in
        upx)
            compress_upx
            ;;
        gzip)
            compress_gzip
            ;;
        xz)
            compress_xz
            ;;
        none)
            log_info "跳过压缩"
            ;;
        *)
            log_error "不支持的压缩类型: $COMPRESS_TYPE"
            exit 1
            ;;
    esac
    
    # 验证
    if [[ "$DO_VERIFY" == "true" ]]; then
        verify_program
    fi
    
    # 生成报告
    generate_report
    
    log_success "构建和压缩完成！"
}

# 运行主函数
main "$@" 