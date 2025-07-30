# MSSH 模块说明文档

## 概述

本文档详细说明了 MSSH 项目中各个模块的功能、接口和使用方法。

## 核心模块

### 1. 应用程序模块 (app.rs)

**功能概述**: 应用程序的主要逻辑控制器，协调各个子模块的工作。

**主要结构**:
```rust
pub struct App {
    pub config_manager: ConfigManager,
    pub ssh_manager: SSHManager,
    pub navigation: NavigationManager,
    pub form_manager: FormManager,
    pub message_manager: MessageManager,
}
```

**核心方法**:
- `new(config_path: Option<String>)` - 创建新的应用实例
- `next()` / `previous()` - 列表导航
- `show_add_form()` / `show_edit_form()` - 显示表单
- `save_config()` / `delete_config()` - 配置操作
- `connect_selected()` - 连接选中的SSH配置
- `quick_connect(target: &str)` - 快速连接

### 2. 配置管理模块 (config.rs)

**功能概述**: 负责SSH配置的读写、解析和管理。

**主要结构**:
```rust
pub struct ConfigManager {
    pub configs: Vec<SSHConfig>,
    pub global_config: GlobalConfig,
    config_path: PathBuf,
}

pub struct SSHConfig {
    pub alias: String,
    pub address: String,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub key: Option<String>,
    pub port_forward: Option<PortForward>,
    pub proxy: Option<ProxyConfig>,
    pub use_global_proxy: bool,
}
```

**核心方法**:
- `new(config_path: Option<String>)` - 创建配置管理器
- `load_configs()` - 加载配置文件
- `save_configs()` - 保存配置到文件
- `add_config(config: SSHConfig)` - 添加新配置
- `update_config(alias: &str, config: SSHConfig)` - 更新配置
- `remove_config(alias: &str)` - 删除配置
- `parse_ssh_host_config(content: &str, host_line: &str)` - 解析SSH配置

### 3. SSH管理模块 (ssh.rs)

**功能概述**: 处理SSH连接的建立和参数构建。

**主要结构**:
```rust
pub struct SSHManager {
    pub global_config: GlobalConfig,
}
```

**核心方法**:
- `new(global_config: GlobalConfig)` - 创建SSH管理器
- `connect(config: &SSHConfig)` - 建立SSH连接
- `build_ssh_command(config: &SSHConfig)` - 构建SSH命令

### 4. 导航管理模块 (navigation_manager.rs)

**功能概述**: 管理应用程序的导航状态和模式切换。

**主要结构**:
```rust
pub struct NavigationManager {
    pub mode: AppMode,
    pub selected_index: usize,
    pub focus: usize,
    pub import_manager: ImportManager,
}

pub enum AppMode {
    List,
    AddForm,
    EditForm,
    DeleteDialog,
    SelectImport,
    ProxyConfig,
}
```

**核心方法**:
- `new()` - 创建导航管理器
- `set_mode(mode: AppMode)` - 设置应用模式
- `next_item(len: usize)` / `previous_item(len: usize)` - 列表项导航
- `toggle_focus()` - 切换焦点
- `start_import(candidates: Vec<SSHConfig>)` - 开始导入流程

### 5. 表单管理模块 (form_manager.rs)

**功能概述**: 管理表单状态、验证和数据处理。

**主要结构**:
```rust
pub struct FormManager {
    form_data: Option<FormData>,
    editing_host: Option<String>,
}
```

**核心方法**:
- `new()` - 创建表单管理器
- `start_add_form()` - 开始添加表单
- `start_edit_form(config: &SSHConfig)` - 开始编辑表单
- `validate_and_create_config()` - 验证并创建配置
- `next_field()` / `previous_field()` - 字段导航
- `insert_char(c: char)` - 插入字符
- `delete_char()` - 删除字符

### 6. 表单数据模块 (forms.rs)

**功能概述**: 定义表单字段和数据结构。

**主要枚举**:
```rust
pub enum FormField {
    Alias,
    Address,
    Port,
    User,
    Key,
    PortForwardEnabled,
    PortForwardLocal,
    PortForwardRemote,
    UseGlobalProxy,
    ProxyHost,
    ProxyPort,
    ProxyUsername,
    ProxyPassword,
}
```

**主要结构**:
```rust
pub struct FormData {
    pub data: HashMap<String, String>,
    pub current_field: usize,
    pub cursor_position: usize,
}
```

**核心方法**:
- `new()` - 创建新表单数据
- `from_config(config: &SSHConfig)` - 从配置创建表单数据
- `to_ssh_config()` - 转换为SSH配置
- `validate()` - 验证表单数据

### 7. 消息管理模块 (message_manager.rs)

**功能概述**: 管理用户提示消息的显示和自动清理。

**主要结构**:
```rust
pub struct MessageManager {
    message: Option<Message>,
    timeout: Option<Duration>,
}

pub struct Message {
    pub content: String,
    pub is_error: bool,
    pub timestamp: Instant,
}
```

**核心方法**:
- `new()` - 创建消息管理器
- `set_success_message(content: String)` - 设置成功消息
- `set_error_message(content: String)` - 设置错误消息
- `check_and_clear_expired()` - 检查并清理过期消息

### 8. 事件处理模块 (events.rs)

**功能概述**: 处理键盘输入和用户交互事件。

**主要结构**:
```rust
pub struct EventHandler;
```

**核心方法**:
- `handle_key_event(app: &mut App, key: KeyEvent)` - 处理键盘事件
- `handle_list_mode(app: &mut App, key: KeyEvent)` - 处理列表模式事件
- `handle_form_mode(app: &mut App, key: KeyEvent)` - 处理表单模式事件
- `handle_import_mode(app: &mut App, key: KeyEvent)` - 处理导入模式事件

### 9. 代理配置模块 (proxy.rs)

**功能概述**: 管理代理配置和类型定义。

**主要枚举**:
```rust
pub enum ProxyType {
    None,
    Socks5,
    Http,
}
```

**主要结构**:
```rust
pub struct ProxyConfig {
    pub proxy_type: ProxyType,
    pub host: String,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub struct GlobalConfig {
    pub proxy: ProxyConfig,
}
```

## UI 模块

### 1. UI 协调器 (ui/mod.rs)

**功能概述**: 协调不同UI组件的渲染。

**核心方法**:
- `ui(f: &mut Frame, app: &App)` - 主UI渲染函数
- `render_help_bar()` - 渲染帮助栏
- `render_message_bar()` - 渲染消息栏

### 2. 列表渲染模块 (ui/list.rs)

**功能概述**: 渲染SSH配置列表界面。

**核心方法**:
- `render_list(f: &mut Frame, area: Rect, app: &App)` - 渲染列表

### 3. 表单渲染模块 (ui/form.rs)

**功能概述**: 渲染配置表单界面。

**核心方法**:
- `render_form(f: &mut Frame, area: Rect, app: &App)` - 渲染表单
- `render_form_field()` - 渲染普通输入字段
- `render_form_field_with_enabled()` - 渲染可启用/禁用的字段
- `render_checkbox_field()` - 渲染复选框字段
- `render_proxy_option_field()` - 渲染代理选项字段

### 4. 对话框渲染模块 (ui/dialog.rs)

**功能概述**: 渲染确认对话框。

**核心方法**:
- `render_dialog(f: &mut Frame, area: Rect, app: &App)` - 渲染对话框

### 5. 导入界面渲染模块 (ui/import.rs)

**功能概述**: 渲染SSH配置导入界面。

**核心方法**:
- `render_import(f: &mut Frame, area: Rect, app: &App)` - 渲染导入界面

### 6. 代理配置渲染模块 (ui/proxy.rs)

**功能概述**: 渲染代理配置界面。

**核心方法**:
- `render_proxy_config(f: &mut Frame, area: Rect, app: &App)` - 渲染代理配置

## 模块间交互

### 数据流向

```
用户输入 → EventHandler → App → 各个Manager → UI渲染
```

### 依赖关系

```
App
├── ConfigManager
├── SSHManager
├── NavigationManager
│   └── ImportManager
├── FormManager
│   └── FormData
└── MessageManager
    └── Message
```

### 配置数据流

```
配置文件 → ConfigManager → FormData → UI显示
UI输入 → FormData → ConfigManager → 配置文件
```

## 扩展指南

### 添加新的表单字段

1. 在 `forms.rs` 的 `FormField` 枚举中添加新字段
2. 实现字段的标签和验证逻辑
3. 在 `ui/form.rs` 中添加渲染逻辑
4. 更新 `FormData::to_ssh_config()` 方法

### 添加新的UI模式

1. 在 `navigation_manager.rs` 的 `AppMode` 枚举中添加新模式
2. 在相应的事件处理器中添加模式处理逻辑
3. 创建新的UI渲染组件
4. 在 `ui/mod.rs` 中添加渲染调用

### 添加新的配置类型

1. 在 `config.rs` 中定义新的配置结构
2. 实现序列化和反序列化逻辑
3. 更新表单处理逻辑
4. 添加相应的UI组件

## 最佳实践

### 错误处理

- 使用 `Result<T, E>` 类型进行错误传播
- 通过 `MessageManager` 提供用户友好的错误信息
- 记录详细的错误日志便于调试

### 状态管理

- 所有状态通过 `App` 集中管理
- 避免在UI组件中直接修改状态
- 使用消息传递模式进行组件间通信

### UI设计

- 保持组件的单一职责
- 使用一致的样式和布局
- 提供清晰的用户反馈和帮助信息