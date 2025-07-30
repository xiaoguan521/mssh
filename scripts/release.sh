#!/bin/bash

# MSSH Release Script
# 用于创建新版本标签和触发 GitHub Actions 自动发布

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 打印彩色消息
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查是否在 git 仓库中
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    print_error "当前目录不是 Git 仓库"
    exit 1
fi

# 检查工作目录是否干净
if ! git diff-index --quiet HEAD --; then
    print_error "工作目录有未提交的更改，请先提交或暂存"
    exit 1
fi

# 获取当前版本
current_version=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
print_info "当前版本: $current_version"

# 获取新版本
if [ $# -eq 0 ]; then
    read -p "请输入新版本号 (例如: 1.0.1): " new_version
else
    new_version=$1
fi

# 验证版本号格式
if ! [[ $new_version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    print_error "版本号格式错误，应为 x.y.z 格式"
    exit 1
fi

print_info "新版本: $new_version"

# 确认操作
read -p "确认创建新版本 $new_version？(y/N): " confirm
if [[ ! $confirm =~ ^[Yy]$ ]]; then
    print_info "操作已取消"
    exit 0
fi

print_info "开始发布流程..."

# 更新 Cargo.toml 中的版本号
print_info "更新 Cargo.toml 版本号..."
sed -i.bak "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
rm -f Cargo.toml.bak

# 更新 CHANGELOG.md
print_info "更新 CHANGELOG.md..."
today=$(date +%Y-%m-%d)
sed -i.bak "s/## \[Unreleased\]/## [Unreleased]\n\n## [$new_version] - $today/" CHANGELOG.md
rm -f CHANGELOG.md.bak

# 提交更改
print_info "提交版本更新..."
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to $new_version"

# 创建标签
print_info "创建标签 v$new_version..."
git tag -a "v$new_version" -m "Release v$new_version"

# 推送到远程仓库
print_info "推送到远程仓库..."
git push origin main
git push origin "v$new_version"

print_success "版本 $new_version 发布成功！"
print_info "GitHub Actions 将自动构建和发布二进制文件"
print_info "请查看: https://github.com/Caterpolaris/mssh/actions"

# 生成发布说明
print_info "生成发布说明..."
cat << EOF

📦 发布说明模板：

## MSSH v$new_version

### 新功能
- 

### 改进
- 

### 修复
- 

### 安装方法

#### Linux x86_64
\`\`\`bash
wget https://github.com/Caterpolaris/mssh/releases/download/v$new_version/mssh-linux-x86_64.tar.gz
tar -xzf mssh-linux-x86_64.tar.gz
sudo mv mssh /usr/local/bin/
\`\`\`

#### macOS
\`\`\`bash
# Intel Mac
wget https://github.com/Caterpolaris/mssh/releases/download/v$new_version/mssh-macos-x86_64.tar.gz
tar -xzf mssh-macos-x86_64.tar.gz
sudo mv mssh /usr/local/bin/

# Apple Silicon Mac
wget https://github.com/Caterpolaris/mssh/releases/download/v$new_version/mssh-macos-aarch64.tar.gz
tar -xzf mssh-macos-aarch64.tar.gz
sudo mv mssh /usr/local/bin/
\`\`\`

EOF

print_success "发布流程完成！" 