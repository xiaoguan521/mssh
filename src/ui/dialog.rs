use crate::app::App;
use ratatui::{prelude::*, widgets::*};

/// 渲染对话框
///
/// # 参数
/// - `f`: 绘制 Frame
/// - `area`: 绘制区域
/// - `app`: 应用状态
/// - `title`: 对话框标题
/// - `message`: 对话框消息
/// - `buttons`: 按钮列表
pub fn render_dialog(
    f: &mut Frame,
    area: Rect,
    app: &mut App,
    _title: &str,
    _message: &str,
    _buttons: &[&str],
) {
    let popup_area = centered_rect(50, 10, area);
    // 当前暂不使用 title, message, buttons 参数

    let dialog_text = if let Some(config) = app.get_selected_config() {
        format!("确定要删除配置 '{}' 吗？", config.alias)
    } else {
        "确定要删除选中的配置吗？".to_string()
    };

    // 将对话框文字加粗显示
    let dialog = Paragraph::new(Span::styled(
        dialog_text,
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("确认删除")
            .border_style(Style::default().fg(Color::LightRed)),
    )
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });

    f.render_widget(Clear, popup_area);
    f.render_widget(dialog, popup_area);
}

/// 生成居中的矩形区域
///
/// # 参数
/// - `percent_x`: 矩形宽度占父区域宽度的百分比
/// - `percent_y`: 矩形高度占父区域高度的百分比
/// - `r`: 父区域
///
/// # 返回
/// 返回计算后的矩形区域
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
