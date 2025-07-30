# MSSH 开发指南

## 项目介绍

MSSH 是一个使用 Rust 语言开发的现代化 SSH 配置管理工具，提供了直观的 TUI（Terminal User Interface）界面，帮助用户轻松管理和连接 SSH 服务器。

## 开发环境搭建

### 前置要求

- Rust 1.70 或更高版本
- Cargo 包管理器
- Git 版本控制系统

### 环境配置

1. **克隆项目**
   ```bash
   git clone <repository-url>
   cd mssh
   ```

2. **安装依赖**
   ```bash
   cargo build
   ```

3. **运行项目**
   ```bash
   cargo run
   ```

## 项目结构

```
ssh-manager/
├── src/
│   ├── main.rs              # 程序入口
│   ├── app.rs               # 应用主控制器
│   ├── config.rs            # 配置管理
│   ├── ssh.rs               # SSH连接管理
│   ├── proxy.rs             # 代理配置
│   ├── events.rs            # 事件处理
│   ├── forms.rs             # 表单数据结构
│   ├── form_manager.rs      # 表单管理器
│   ├── message_manager.rs   # 消息管理器
│   ├── navigation_manager.rs # 导航管理器
│   └── ui/                  # UI组件
│       ├── mod.rs           # UI协调器
│       ├── list.rs          # 列表界面
│       ├── form.rs          # 表单界面
│       ├── dialog.rs        # 对话框
│       ├── import.rs        # 导入界面
│       └── proxy.rs         # 代理配置界面
│       └── scrollbar.rs     # 滚动条
├── config.yaml              # 默认配置文件
├── doc/                     # 文档目录
├── scripts/                 # 构建脚本
├── Cargo.toml               # 项目配置
└── README.md                # 项目说明
```

## 开发流程

### 1. 功能开发流程

1. **需求分析**
   - 明确功能需求和用户场景
   - 设计交互流程和界面布局
   - 确定数据结构和API接口

2. **设计阶段**
   - 更新架构设计文档
   - 设计数据模型和状态管理
   - 规划模块间接口

3. **开发实现**
   - 创建或修改相关模块
   - 实现业务逻辑和数据处理
   - 添加UI组件和交互逻辑

4. **测试验证**
   - 编写单元测试
   - 进行集成测试
   - 用户体验测试

5. **文档更新**
   - 更新相关文档
   - 添加使用示例
   - 更新CHANGELOG

### 2. 代码规范

#### Rust 代码规范

1. **命名规范**
   ```rust
   // 结构体使用大驼峰命名
   struct SSHConfig { }
   
   // 变量和函数使用小写下划线命名
   let config_manager = ConfigManager::new();
   
   // 常量使用大写下划线命名
   const DEFAULT_PORT: u16 = 22;
   
   // 枚举使用大驼峰命名
   enum AppMode {
       List,
       AddForm,
   }
   ```

2. **错误处理**
   ```rust
   // 使用 Result 类型进行错误传播
   fn load_config() -> Result<Config, ConfigError> {
       // 实现逻辑
   }
   
   // 使用 ? 操作符简化错误传播
   fn process_config() -> Result<(), Box<dyn std::error::Error>> {
       let config = load_config()?;
       save_config(config)?;
       Ok(())
   }
   ```

3. **文档注释**
   ```rust
   /// 加载SSH配置文件
   /// 
   /// # Arguments
   /// 
   /// * `path` - 配置文件路径
   /// 
   /// # Returns
   /// 
   /// 返回配置对象或错误信息
   /// 
   /// # Examples
   /// 
   /// ```
   /// let config = load_config("~/.ssh/config")?;
   /// ```
   pub fn load_config(path: &str) -> Result<Config, ConfigError> {
       // 实现逻辑
   }
   ```

#### UI 开发规范

1. **组件结构**
   ```rust
   // UI组件应该是纯函数，不包含状态
   pub fn render_form(f: &mut Frame, area: Rect, app: &App) {
       // 渲染逻辑
   }
   ```

2. **样式一致性**
   ```rust
   // 使用统一的颜色和样式定义
   let focused_style = Style::default().fg(Color::Yellow);
   let normal_style = Style::default();
   ```

3. **响应式设计**
   ```rust
   // 根据终端大小调整布局
   let chunks = Layout::default()
       .direction(Direction::Vertical)
       .constraints([
           Constraint::Min(1),
           Constraint::Length(3),
       ])
       .split(area);
   ```

### 3. 调试和测试

#### 调试方法

1. **日志调试**
   ```rust
   // 使用 eprintln! 进行调试输出
   eprintln!("Debug: config = {:?}", config);
   ```

2. **条件编译**
   ```rust
   #[cfg(debug_assertions)]
   {
       eprintln!("Debug mode: {}", debug_info);
   }
   ```

#### 测试编写

1. **单元测试**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_config_parsing() {
           let config = parse_config("test data");
           assert_eq!(config.alias, "test");
       }
   }
   ```

2. **集成测试**
   ```rust
   // tests/integration_test.rs
   use mssh::*;
   
   #[test]
   fn test_full_workflow() {
       // 测试完整的工作流程
   }
   ```

### 4. 性能优化

#### 内存管理

1. **避免不必要的克隆**
   ```rust
   // 好的做法：使用引用
   fn process_config(config: &SSHConfig) { }
   
   // 避免：不必要的克隆
   fn process_config(config: SSHConfig) { }
   ```

2. **使用适当的数据结构**
   ```rust
   // 频繁查找使用 HashMap
   use std::collections::HashMap;
   let mut configs: HashMap<String, SSHConfig> = HashMap::new();
   
   // 有序存储使用 Vec
   let mut configs: Vec<SSHConfig> = Vec::new();
   ```

#### 渲染优化

1. **减少重绘**
   ```rust
   // 只在状态变化时重绘
   if app.state_changed() {
       terminal.draw(|f| ui::ui(f, app))?;
   }
   ```

2. **优化布局计算**
   ```rust
   // 缓存布局计算结果
   let layout = Layout::default()
       .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
       .split(area);
   ```

## 常见开发任务

### 1. 添加新的配置字段

1. **更新数据结构**
   ```rust
   // 在 config.rs 中更新 SSHConfig
   pub struct SSHConfig {
       // 现有字段...
       pub new_field: Option<String>,
   }
   ```

2. **更新表单字段**
   ```rust
   // 在 forms.rs 中添加新字段
   pub enum FormField {
       // 现有字段...
       NewField,
   }
   ```

3. **更新UI渲染**
   ```rust
   // 在 ui/form.rs 中添加渲染逻辑
   render_form_field(f, chunks[n], "新字段", "new_field", app, n);
   ```

### 2. 添加新的UI模式

1. **添加模式定义**
   ```rust
   // 在 navigation_manager.rs 中添加
   pub enum AppMode {
       // 现有模式...
       NewMode,
   }
   ```

2. **添加事件处理**
   ```rust
   // 在 events.rs 中添加处理逻辑
   fn handle_new_mode(app: &mut App, key: KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
       // 处理逻辑
   }
   ```

3. **添加UI组件**
   ```rust
   // 创建 ui/new_mode.rs
   pub fn render_new_mode(f: &mut Frame, area: Rect, app: &App) {
       // 渲染逻辑
   }
   ```

### 3. 修复Bug的流程

1. **问题重现**
   - 记录bug的具体表现
   - 确定重现步骤
   - 分析影响范围

2. **定位问题**
   - 使用调试输出定位问题代码
   - 检查相关的数据流
   - 分析状态变化

3. **修复实现**
   - 实现最小化的修复方案
   - 确保不引入新问题
   - 添加防御性代码

4. **测试验证**
   - 验证bug已修复
   - 回归测试确保无副作用
   - 更新相关测试用例

## 发布流程

### 1. 版本管理

```bash
# 更新版本号
# 在 Cargo.toml 中更新 version 字段

# 更新 CHANGELOG.md
# 记录本版本的主要变更

# 提交版本变更
git add -A
git commit -m "chore: bump version to x.y.z"

# 创建版本标签
git tag -a v1.0.0 -m "Release version 1.0.0"
```

### 2. 构建发布

```bash
# 构建优化版本
cargo build --release

# 运行完整测试
cargo test

# 检查代码质量
cargo clippy
cargo fmt --check

# 构建压缩版本（如果有脚本）
./build-compress.sh
```

### 3. 部署发布

```bash
# 推送代码和标签
git push origin main
git push origin --tags

# 如果有自动部署脚本
./deploy-compressed.sh
```

## 贡献指南

### 1. 提交规范

使用约定式提交格式：

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

类型包括：
- `feat`: 新功能
- `fix`: 修复bug
- `docs`: 文档更新
- `style`: 代码格式调整
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建或辅助工具变动

### 2. Pull Request 流程

1. Fork 项目到个人仓库
2. 创建功能分支
3. 开发并测试功能
4. 提交Pull Request
5. 代码审查和讨论
6. 合并到主分支

### 3. 代码审查要点

- 代码风格是否符合规范
- 功能实现是否正确
- 错误处理是否完善
- 性能是否有优化空间
- 文档是否需要更新

## 故障排除

### 1. 常见编译错误

1. **依赖版本冲突**
   ```bash
   # 更新依赖
   cargo update
   
   # 清理构建缓存
   cargo clean && cargo build
   ```

2. **生命周期错误**
   ```rust
   // 使用显式生命周期标注
   fn process<'a>(data: &'a str) -> &'a str {
       data
   }
   ```

### 2. 运行时问题

1. **终端兼容性**
   - 确保终端支持UTF-8编码
   - 检查终端大小是否足够
   - 验证颜色支持

2. **配置文件问题**
   - 检查配置文件格式
   - 验证文件权限
   - 确认路径正确性

### 3. 性能问题

1. **内存使用过高**
   - 检查是否有内存泄漏
   - 优化数据结构使用
   - 减少不必要的克隆

2. **渲染卡顿**
   - 优化UI渲染逻辑
   - 减少重复计算
   - 使用缓存机制

## 参考资源

### 1. Rust 学习资源

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Rust 程序设计语言](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)

### 2. TUI 开发资源

- [Ratatui 文档](https://docs.rs/ratatui/)
- [Crossterm 文档](https://docs.rs/crossterm/)
- [TUI 设计模式](https://github.com/fdehau/tui-rs/tree/master/examples)

### 3. 项目相关

- [SSH 配置文档](https://man.openbsd.org/ssh_config)
- [SOCKS 代理协议](https://tools.ietf.org/html/rfc1928)
- [HTTP 代理协议](https://tools.ietf.org/html/rfc7231)
