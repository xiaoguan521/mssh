use crate::app::App;
use ratatui::{prelude::*, widgets::*};

/// 渲染代理配置界面
///
/// # 参数
/// - `f`: 绘制 Frame
/// - `area`: 绘制区域
/// - `app`: 应用状态
pub fn render_proxy_config(f: &mut Frame, area: Rect, app: &mut App) {
    let form_block = Block::default()
        .title("全局代理配置")
        .borders(Borders::NONE);

    let inner_area = form_block.inner(area);
    f.render_widget(form_block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 代理类型
            Constraint::Length(3), // 主机
            Constraint::Length(3), // 端口
            Constraint::Length(3), // 用户名
            Constraint::Length(3), // 密码
            Constraint::Min(0),    // 剩余空间
        ])
        .split(inner_area);

    render_proxy_type_field(f, chunks[0], app, 0);
    render_proxy_field(f, chunks[1], "主机", "global_proxy_host", app, 1);
    render_proxy_field(f, chunks[2], "端口", "global_proxy_port", app, 2);
    render_proxy_field(f, chunks[3], "用户名", "global_proxy_username", app, 3);
    render_proxy_field(f, chunks[4], "密码", "global_proxy_password", app, 4);
}

/// 渲染代理字段
///
/// # 参数
/// - `f`: 绘制 Frame
/// - `area`: 绘制区域
/// - `label`: 字段标签
/// - `field_name`: 字段名
/// - `app`: 应用状态
/// - `field_index`: 字段索引
fn render_proxy_field(
    f: &mut Frame,
    area: Rect,
    label: &str,
    field_name: &str,
    app: &mut App,
    field_index: usize,
) {
    let is_focused = app.current_field() == field_index;

    let value = app.form_data().get(field_name).cloned().unwrap_or_default();
    let display_value = if field_name.contains("password") {
        if is_focused && !value.is_empty() {
            // 显示前面的字符为星号，最后一个字符显示明文
            value
        } else {
            // 非聚焦或空字符串时全部显示为星号
            "*".repeat(value.len())
        }
    } else {
        value
    };
    let input = Paragraph::new(display_value).block(
        Block::default()
            .borders(Borders::ALL)
            .title(label)
            .border_style(if is_focused {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            }),
    );

    f.render_widget(input, area);

    if is_focused {
        f.set_cursor(area.x + app.cursor_position() as u16 + 1, area.y + 1);
    }
}

/// 渲染代理类型字段
///
/// # 参数
/// - `f`: 绘制 Frame
/// - `area`: 绘制区域
/// - `app`: 应用状态
/// - `field_index`: 字段索引
fn render_proxy_type_field(f: &mut Frame, area: Rect, app: &mut App, field_index: usize) {
    let is_focused = app.current_field() == field_index;
    let proxy_type = app
        .form_data()
        .get("global_proxy_type")
        .map(|t| t.as_str())
        .unwrap_or("None");

    let type_text = match proxy_type {
        "Socks5" => "SOCKS5",
        "Http" => "HTTP",
        _ => "无代理",
    };

    let type_field = Paragraph::new(type_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("代理类型 (空格切换)")
            .border_style(if is_focused {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            }),
    );

    f.render_widget(type_field, area);
}
