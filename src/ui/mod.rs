mod dialog;
mod form;
mod import;
mod list;
mod proxy;
mod scrollbar;

use crate::app::App;
use crate::navigation_manager::AppMode;
use ratatui::{prelude::*, widgets::*};

pub use dialog::render_dialog;
pub use form::render_form;
pub use import::render_import;
pub use list::render_list;
pub use proxy::render_proxy_config;
pub use scrollbar::{render_scrollbar, ScrollManager};

/// 渲染主用户界面
///
/// # 参数
/// - `f`: 绘制 Frame
/// - `app`: 应用状态
pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0), // 主内容区域
            // Constraint::Length(3),   // 消息栏
            Constraint::Length(3), // 帮助栏
        ])
        .split(f.size());

    match *app.mode() {
        AppMode::List => render_list(f, chunks[0], app),
        AppMode::AddForm | AppMode::EditForm => render_form(f, chunks[0], app),
        AppMode::DeleteDialog => render_dialog(
            f,
            chunks[0],
            app,
            "确认删除",
            "确定要删除这个SSH配置吗？",
            &["确定", "取消"],
        ),
        AppMode::SelectImport => render_import(f, chunks[0], app),
        AppMode::ProxyConfig => render_proxy_config(f, chunks[0], app),
    }

    render_message_bar(f, app);
    render_help_bar(f, chunks[1], app);
}

/// 渲染帮助栏
///
/// # 参数
/// - `f`: 绘制 Frame
/// - `area`: 绘制区域
/// - `app`: 应用状态
fn render_help_bar(f: &mut Frame, area: Rect, app: &App) {
    let help_text = match *app.mode() {
        AppMode::List => vec![
            Span::raw("Enter: 连接 | "),
            Span::raw("Ctrl+N: 新增 | "),
            Span::raw("Ctrl+E: 编辑 | "),
            Span::raw("Ctrl+D: 删除 | "),
            Span::raw("Ctrl+L: 导入 | "),
            Span::raw("Ctrl+P: 全局代理 | "),
            Span::raw("Ctrl+Q: 退出"),
        ],
        AppMode::AddForm | AppMode::EditForm => vec![
            Span::raw("Enter: 保存 | "),
            Span::raw("Tab/↑↓: 切换字段 | "),
            Span::raw("Esc: 取消"),
        ],
        AppMode::DeleteDialog => vec![Span::raw("Enter: 确认删除 | "), Span::raw("Esc: 取消")],
        AppMode::SelectImport => vec![
            Span::raw("Space: 选择/取消 | "),
            Span::raw("Ctrl+A: 全选 | "),
            Span::raw("Enter: 导入 | "),
            Span::raw("Esc: 取消"),
        ],
        AppMode::ProxyConfig => vec![
            Span::raw("Enter: 保存 | "),
            Span::raw("Tab/↑↓: 切换字段 | "),
            Span::raw("Esc: 取消"),
        ],
    };

    let help = Paragraph::new(Line::from(help_text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("帮助")
                .border_style(Style::default().fg(Color::LightBlue)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(help, area);
}

/// 渲染消息栏
///
/// # 参数
/// - `f`: 绘制 Frame
/// - `app`: 应用状态
fn render_message_bar(f: &mut Frame, app: &App) {
    if let Some(message) = app.message() {
        let style = if message.is_error {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::Green)
        };

        let popup_area = centered_rect(60, 90, 3, f.size());
        let message_widget = Paragraph::new(message.content.as_str())
            .style(style)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        f.render_widget(Clear, popup_area);
        f.render_widget(message_widget, popup_area);
    }
}

/// 生成一个在水平方向居中，垂直方向可调整的矩形区域
///
/// # 参数
/// - `percent_x`: 矩形宽度占父区域宽度的百分比
/// - `percent_y`: 矩形顶部位置占父区域高度的百分比 (0为顶部，100为底部)
/// - `height`: 矩形的固定高度
/// - `r`: 父区域
///
/// # 返回
/// 返回计算后的矩形区域
fn centered_rect(percent_x: u16, percent_y: u16, height: u16, r: Rect) -> Rect {
    // 水平方向仍然保持居中
    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(r);

    // 垂直方向根据percent_y调整
    let available_height = r.height;
    let top_margin = available_height.saturating_mul(percent_y) / 100;

    // 确保有足够的空间放置矩形
    let remaining_height = available_height.saturating_sub(top_margin);
    let actual_height = std::cmp::min(height, remaining_height);

    Rect {
        x: horizontal_layout[1].x,
        y: r.y + top_margin,
        width: horizontal_layout[1].width,
        height: actual_height,
    }
}
