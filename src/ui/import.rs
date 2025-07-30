use crate::app::App;
use ratatui::{prelude::*, widgets::*};

/// 渲染导入界面
///
/// # 参数
/// - `f`: 绘制 Frame
/// - `area`: 绘制区域
/// - `app`: 应用状态
pub fn render_import(f: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let info_text = format!(
        "从 ~/.ssh/config 文件中找到 {} 个配置 (已选择: {})",
        app.import_candidates().len(),
        app.import_selected().iter().filter(|&&x| x).count()
    );

    let info = Paragraph::new(info_text)
        .block(Block::default().borders(Borders::ALL).title("SSH 配置导入"))
        .alignment(Alignment::Center);

    f.render_widget(info, chunks[0]);

    let items: Vec<ListItem> = app
        .import_candidates()
        .iter()
        .enumerate()
        .map(|(i, config)| {
            let checkbox = if app.import_selected().get(i).copied().unwrap_or(false) {
                "[✓]"
            } else {
                "[ ]"
            };

            let content = format!(
                "{} {} ({}@{}:{})",
                checkbox,
                config.alias,
                config.user.as_deref().unwrap_or("root"),
                config.address,
                config.port.unwrap_or(22)
            );

            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("可导入的配置"))
        .style(Style::default())
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ");

    let mut state = ListState::default();
    state.select(Some(app.import_selected_index()));
    f.render_stateful_widget(list, chunks[1], &mut state);
}
