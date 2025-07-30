#!/bin/bash

# SSH Manager 压缩部署脚本
# 支持多种部署方式和平台

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

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
    echo "SSH Manager 压缩部署脚本"
    echo ""
    echo "用法: $0 [选项]"
    echo ""
    echo "选项:"
    echo "  --help, -h          显示帮助信息"
    echo "  --build             构建压缩程序"
    echo "  --deploy <target>   部署到目标服务器"
    echo "  --package           打包压缩文件"
    echo "  --install <path>    安装到指定路径"
    echo "  --remote <server>   远程部署"
    echo "  --all               完整流程 (构建+打包+部署)"
    echo ""
    echo "示例:"
    echo "  $0 --build                    # 构建压缩程序"
    echo "  $0 --deploy 10.20.30.30      # 部署到服务器"
    echo "  $0 --package                  # 打包压缩文件"
    echo "  $0 --all                      # 完整流程"
}

# 构建压缩程序
build_compressed() {
    log_info "开始构建压缩程序..."
    
    # 检查构建脚本
    if [[ ! -f "build-compress.sh" ]]; then
        log_error "未找到构建脚本 build-compress.sh"
        exit 1
    fi
    
    # 设置执行权限
    chmod +x build-compress.sh
    
    # 执行构建
    ./build-compress.sh --all
    
    log_success "构建完成"
}

# 打包压缩文件
package_compressed() {
    log_info "开始打包压缩文件..."
    
    # 查找二进制文件
    local binary_files=()
    
    # 查找所有可能的二进制文件
    for file in target/release/mssh* target/*/release/mssh*; do
        if [[ -f "$file" ]]; then
            binary_files+=("$file")
        fi
    done
    
    if [[ ${#binary_files[@]} -eq 0 ]]; then
        log_error "未找到二进制文件，请先构建程序"
        exit 1
    fi
    
    # 创建发布目录
    local release_dir="release"
    mkdir -p "$release_dir"
    
    # 复制文件
    for file in "${binary_files[@]}"; do
        local filename=$(basename "$file")
        local target_path="$release_dir/$filename"
        cp "$file" "$target_path"
        chmod +x "$target_path"
        log_info "复制: $file -> $target_path"
    done
    
    # 复制配置文件
    if [[ -d "config" ]]; then
        cp -r config "$release_dir/"
        log_info "复制配置文件"
    fi
    
    # 创建安装脚本
    cat > "$release_dir/install.sh" << 'EOF'
#!/bin/bash

# SSH Manager 安装脚本

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

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

# 检查是否为 root 用户
check_root() {
    if [[ $EUID -ne 0 ]]; then
        log_error "此脚本需要 root 权限运行"
        exit 1
    fi
}

# 安装程序
install_program() {
    log_info "开始安装 SSH Manager..."
    
    # 创建安装目录
    local install_dir="/usr/local/bin"
    local config_dir="/etc/mssh"
    
    mkdir -p "$install_dir"
    mkdir -p "$config_dir"
    
    # 查找二进制文件
    local binary_file=""
    for file in mssh*; do
        if [[ -f "$file" && -x "$file" ]]; then
            binary_file="$file"
            break
        fi
    done
    
    if [[ -z "$binary_file" ]]; then
        log_error "未找到可执行的二进制文件"
        exit 1
    fi
    
    # 安装二进制文件
    cp "$binary_file" "$install_dir/mssh"
    chmod +x "$install_dir/mssh"
    
    # 安装配置文件
    if [[ -d "config" ]]; then
        cp -r config/* "$config_dir/"
        log_info "配置文件已安装到 $config_dir"
    fi
    
    # 创建软链接
    ln -sf "$install_dir/mssh" /usr/bin/mssh
    
    log_success "SSH Manager 已安装到 $install_dir"
    log_info "使用方法: mssh --help"
}

# 主函数
main() {
    check_root
    install_program
    log_success "安装完成！"
}

main "$@"
EOF
    
    chmod +x "$release_dir/install.sh"
    
    # 创建压缩包
    local timestamp=$(date +%Y%m%d_%H%M%S)
    local archive_name="mssh_${timestamp}.tar.gz"
    
    tar -czf "$archive_name" -C "$release_dir" .
    
    log_success "打包完成: $archive_name"
    log_info "文件大小: $(du -h "$archive_name" | cut -f1)"
}

# 部署到目标服务器
deploy_to_server() {
    local server="$1"
    
    if [[ -z "$server" ]]; then
        log_error "请指定目标服务器"
        exit 1
    fi
    
    log_info "部署到服务器: $server"
    
    # 检查是否有压缩包
    local archive_file=""
    for file in mssh_*.tar.gz; do
        if [[ -f "$file" ]]; then
            archive_file="$file"
            break
        fi
    done
    
    if [[ -z "$archive_file" ]]; then
        log_error "未找到压缩包，请先运行 --package"
        exit 1
    fi
    
    # 上传文件
    log_info "上传文件到服务器..."
    scp "$archive_file" "root@$server:/tmp/"
    
    # 远程安装
    log_info "远程安装..."
    ssh "root@$server" << EOF
cd /tmp
tar -xzf $(basename "$archive_file")
chmod +x install.sh
./install.sh
rm -f $(basename "$archive_file")
rm -rf mssh*
EOF
    
    log_success "部署完成"
}

# 远程部署
remote_deploy() {
    local server="$1"
    
    if [[ -z "$server" ]]; then
        log_error "请指定目标服务器"
        exit 1
    fi
    
    log_info "远程部署到: $server"
    
    # 创建临时目录
    local temp_dir=$(mktemp -d)
    
    # 复制必要文件
    cp -r . "$temp_dir/"
    cd "$temp_dir"
    
    # 构建和部署
    ./build-compress.sh --all
    ./deploy-compressed.sh --package
    
    # 上传并安装
    local archive_file=""
    for file in mssh_*.tar.gz; do
        if [[ -f "$file" ]]; then
            archive_file="$file"
            break
        fi
    done
    
    if [[ -n "$archive_file" ]]; then
        scp "$archive_file" "root@$server:/tmp/"
        ssh "root@$server" << EOF
cd /tmp
tar -xzf $(basename "$archive_file")
chmod +x install.sh
./install.sh
rm -f $(basename "$archive_file")
rm -rf mssh*
EOF
        log_success "远程部署完成"
    else
        log_error "构建失败"
        exit 1
    fi
    
    # 清理临时目录
    cd - > /dev/null
    rm -rf "$temp_dir"
}

# 安装到指定路径
install_to_path() {
    local install_path="$1"
    
    if [[ -z "$install_path" ]]; then
        log_error "请指定安装路径"
        exit 1
    fi
    
    log_info "安装到路径: $install_path"
    
    # 查找二进制文件
    local binary_file=""
    for file in target/release/mssh* target/*/release/mssh*; do
        if [[ -f "$file" ]]; then
            binary_file="$file"
            break
        fi
    done
    
    if [[ -z "$binary_file" ]]; then
        log_error "未找到二进制文件，请先构建程序"
        exit 1
    fi
    
    # 创建安装目录
    mkdir -p "$install_path"
    
    # 复制文件
    cp "$binary_file" "$install_path/mssh"
    chmod +x "$install_path/mssh"
    
    # 复制配置文件
    if [[ -d "config" ]]; then
        cp -r config "$install_path/"
    fi
    
    log_success "安装完成: $install_path/mssh"
}

# 主函数
main() {
    local action=""
    local target=""
    
    # 解析命令行参数
    while [[ $# -gt 0 ]]; do
        case $1 in
            --help|-h)
                show_help
                exit 0
                ;;
            --build)
                action="build"
                shift
                ;;
            --deploy)
                action="deploy"
                target="$2"
                shift 2
                ;;
            --package)
                action="package"
                shift
                ;;
            --install)
                action="install"
                target="$2"
                shift 2
                ;;
            --remote)
                action="remote"
                target="$2"
                shift 2
                ;;
            --all)
                action="all"
                shift
                ;;
            *)
                log_error "未知参数: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    case $action in
        build)
            build_compressed
            ;;
        package)
            package_compressed
            ;;
        deploy)
            deploy_to_server "$target"
            ;;
        install)
            install_to_path "$target"
            ;;
        remote)
            remote_deploy "$target"
            ;;
        all)
            build_compressed
            package_compressed
            log_info "完整流程完成，可以运行 --deploy <server> 进行部署"
            ;;
        "")
            log_error "请指定操作"
            show_help
            exit 1
            ;;
    esac
}

# 运行主函数
main "$@" 