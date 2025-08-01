# MSSH 架构设计文档

## 项目概述

MSSH 是一个现代化的 SSH 配置管理工具，采用 TUI（Terminal User Interface）界面，使用 Rust 语言开发。该项目经过模块化重构，具有清晰的架构层次和职责分离。

## 核心架构

### 整体架构图

```
┌─────────────────────────────────────────────────┐
│               main.rs                           │
│          (程序入口和终端管理)                      │
└─────────────────┬───────────────────────────────┘
                  │
┌─────────────────▼───────────────────────────────┐
│                  App                            │
│             (应用状态管理)                        │
└─────┬───────┬───────┬───────┬──────┬────────────┘
      │       │       │       │      │
      ▼       ▼       ▼       ▼      ▼
┌──────────┐ ┌─────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐
│Navigation│ │ Form    │ │ Message  │ │ Config   │ │   SSH    │
│ Manager  │ │ Manager │ │ Manager  │ │ Manager  │ │ Manager  │
└──────────┘ └─────────┘ └──────────┘ └──────────┘ └──────────┘
      │            │            │            │            │
      ▼            ▼            ▼            ▼            ▼
┌──────────┐ ┌─────────┐ ┌───────────┐ ┌──────────┐ ┌──────────┐
│  导航     │ │ 表单     │ │  消息     │ │  配置     │ │  SSH     │
│  状态     │ │ 状态     │ │  提示     │ │  文件     │ │  连接     │
└──────────┘ └─────────┘ └───────────┘ └──────────┘ └──────────┘
```

### UI 渲染层

```
┌────────────────────────────────────────────────┐
│               ui/mod.rs                        │
│              (UI 协调器)                        │
└─┬─────┬─────┬─────┬─────┬──────────────────────┘
  │     │     │     │     │
  ▼     ▼     ▼     ▼     ▼
┌────┐┌────┐┌──────┐┌──────┐┌─────┐
│list││form││dialog││import││proxy│
└────┘└────┘└──────┘└──────┘└─────┘
```

## 核心模块详解

### 1. 应用层 (App)

**文件**: `src/app.rs`

**职责**:
- 应用状态的中央管理
- 各个管理器之间的协调
- 对外提供统一的 API 接口
- 业务逻辑的封装

**主要功能**:
- 配置的增删改查操作
- SSH 连接管理
- 表单状态管理
- 导航状态控制
- 消息提示管理

### 2. 导航管理器 (NavigationManager)

**文件**: `src/navigation_manager.rs`

**职责**:
- 应用模式切换 (列表、表单、对话框等)
- 列表项选择状态管理
- 导入功能的状态管理
- 焦点状态控制

**应用模式**:
```rust
pub enum AppMode {
    List,           // 主列表界面
    AddForm,        // 添加配置表单
    EditForm,       // 编辑配置表单
    DeleteDialog,   // 删除确认对话框
    SelectImport,   // 导入选择界面
    ProxyConfig,    // 代理配置界面
}
```

### 3. 表单管理器 (FormManager)

**文件**: `src/form_manager.rs`

**职责**:
- 表单字段数据管理
- 表单验证逻辑
- 光标位置控制
- 字段导航
- 配置对象构建

**支持的表单类型**:
- SSH 配置表单
- 代理配置表单

### 4. 消息管理器 (MessageManager)

**文件**: `src/message_manager.rs`

**职责**:
- 成功/错误消息的显示
- 消息自动过期清理
- 消息状态管理

### 5. 配置管理器 (ConfigManager)

**文件**: `src/config.rs`

**职责**:
- 配置文件的读写
- SSH 配置对象管理
- 全局配置管理
- SSH config 文件解析

### 6. SSH 管理器 (SSHManager)

**文件**: `src/ssh.rs`

**职责**:
- SSH 连接建立
- 代理配置应用
- 连接参数构建

### 7. 事件处理器 (EventHandler)

**文件**: `src/events.rs`

**职责**:
- 键盘事件分发
- 不同模式下的事件处理
- 用户输入处理

### 8. UI 渲染层

**目录**: `src/ui/`

**组件结构**:
- `mod.rs`: UI 协调器和公共组件
- `list.rs`: 主列表界面渲染
- `form.rs`: 表单界面渲染
- `dialog.rs`: 对话框渲染
- `import.rs`: 导入界面渲染
- `proxy.rs`: 代理配置界面渲染

## 数据流

### 1. 用户输入流

```
用户键盘输入 → EventHandler → App 方法调用 → 状态更新 → UI 重渲染
```

### 2. 配置操作流

```
用户操作 → App → FormManager (验证) → ConfigManager (持久化) → MessageManager (反馈)
```

### 3. SSH 连接流

```
用户选择连接 → App → SSHManager → 系统 SSH 命令执行
```

## 设计原则

### 1. 单一职责原则
每个管理器都有明确的职责范围，避免功能重叠。

### 2. 依赖注入
App 作为依赖注入容器，管理所有管理器的生命周期。

### 3. 状态集中管理
所有状态通过 App 进行集中管理，避免状态分散。

### 4. 错误处理
统一的错误处理机制，通过 MessageManager 提供用户反馈。

### 5. 模块化设计
清晰的模块边界，便于维护和扩展。

## 扩展性设计

### 1. 新增 UI 模式
在 `AppMode` 枚举中添加新模式，在相应管理器中实现状态管理。

### 2. 新增配置类型
在 FormManager 中添加新的表单类型和验证逻辑。

### 3. 新增连接方式
在 SSHManager 中扩展连接方式和参数处理。

### 4. 新增 UI 组件
在 `ui/` 目录下添加新的渲染组件。

## 性能考虑

### 1. 状态更新优化
只在状态真正变化时触发 UI 重渲染。

### 2. 配置文件缓存
配置文件读取后进行内存缓存，避免频繁 IO 操作。

### 3. 消息自动清理
实现消息的自动过期机制，避免内存泄漏。

## 安全考虑

### 1. 配置文件权限
严格控制配置文件的读写权限。

### 2. SSH 密钥管理
安全地处理 SSH 密钥路径和权限。

### 3. 代理配置验证
对代理配置进行严格验证，防止恶意配置。