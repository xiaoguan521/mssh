use crate::app::App;
use crate::ui::render_scrollbar;
use ratatui::{prelude::*, widgets::*};

/// 渲染配置列表界面
///
/// # 参数
/// - `f`: 绘制 Frame
/// - `area`: 绘制区域
/// - `app`: 应用状态
pub fn render_list(f: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    render_config_list(f, chunks[0], app);
    render_config_details(f, chunks[1], app);
}

/// 渲染配置列表
///
/// # 参数
/// - `f`: 绘制 Frame
/// - `area`: 绘制区域
/// - `app`: 应用状态
fn render_config_list(f: &mut Frame, area: Rect, app: &mut App) {
    let configs = &app.config_manager.configs;

    // 计算可视区域大小（减去边框和标题的高度）
    let visible_height = area.height.saturating_sub(2); // 减去上下边框
    let visible_items = visible_height as usize;

    // 更新滚动位置
    app.navigation
        .update_scroll_position(configs.len(), visible_items);

    // 创建所有项目列表
    let items: Vec<ListItem> = configs
        .iter()
        .enumerate()
        .map(|(i, config)| {
            let proxy_info = if config.use_global_proxy {
                if app.config_manager.global_config.proxy.is_enabled() {
                    " [全局代理]".to_string()
                } else {
                    String::new()
                }
            } else if let Some(proxy) = &config.proxy {
                if proxy.is_enabled() {
                    format!(
                        " [{}代理]",
                        match proxy.proxy_type {
                            crate::proxy::ProxyType::Socks5 => "SOCKS5",
                            crate::proxy::ProxyType::Http => "HTTP",
                            crate::proxy::ProxyType::None => "",
                        }
                    )
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            let pf_info = if let Some(pf) = &config.port_forward {
                if pf.enabled {
                    format!(" [端口转发: {}->{}]", pf.local, pf.remote)
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            let content = format!(
                "{}. {} ({}@{}:{}){}{}",
                i + 1,
                config.alias,
                config.user.as_deref().unwrap_or("root"),
                config.address,
                config.port.unwrap_or(22),
                proxy_info,
                pf_info
            );

            ListItem::new(content)
        })
        .collect();

    // 获取滚动偏移量
    let scroll_offset = app.navigation.get_scroll_offset();

    // 创建可见项目列表（只显示当前可视区域的项目）
    let visible_items_list: Vec<ListItem> = items
        .into_iter()
        .skip(scroll_offset)
        .take(visible_items)
        .collect();

    // 创建列表，使用滚动功能
    let list = List::new(visible_items_list)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("SSH 配置列表")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .style(Style::default())
        .highlight_style(if app.focus() == 0 {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default().add_modifier(Modifier::BOLD)
        })
        .highlight_symbol(">> ");

    let mut state = ListState::default();
    // 调整选中索引以反映滚动偏移
    let adjusted_index = if app.selected_index() >= scroll_offset {
        app.selected_index() - scroll_offset
    } else {
        0
    };
    state.select(Some(adjusted_index));
    f.render_stateful_widget(list, area, &mut state);

    // 渲染滚动条（如果内容超出可视区域）
    if configs.len() > visible_items {
        render_scrollbar(f, area, &app.navigation.scroll_manager);
    }
}

/// 渲染配置详情
///
/// # 参数
/// - `f`: 绘制 Frame
/// - `area`: 绘制区域
/// - `app`: 应用状态
fn render_config_details(f: &mut Frame, area: Rect, app: &App) {
    let details = if let Some(config) = app.get_selected_config() {
        let mut lines = vec![
            Line::from(vec![
                Span::styled("别名: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&config.alias),
            ]),
            Line::from(vec![
                Span::styled("地址: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&config.address),
            ]),
            Line::from(vec![
                Span::styled("端口: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(config.port.map_or("22".to_string(), |p| p.to_string())),
            ]),
            Line::from(vec![
                Span::styled("用户: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(config.user.as_deref().unwrap_or("root")),
            ]),
        ];

        // 总是显示密钥状态
        if let Some(key) = &config.key {
            lines.push(Line::from(vec![
                Span::styled("密钥: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(key),
            ]));
        } else {
            // 如果没有设置密钥，显示无
            lines.push(Line::from(vec![
                Span::styled("密钥: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("无"),
            ]));
        }

        // 总是显示端口转发状态
        lines.push(Line::from(""));
        if let Some(pf) = &config.port_forward {
            lines.push(Line::from(vec![
                Span::styled("端口转发: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(if pf.enabled { "启用" } else { "禁用" }),
            ]));
            if pf.enabled {
                lines.push(Line::from(vec![
                    Span::styled("  本地: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(&pf.local),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("  远程: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(&pf.remote),
                ]));
            }
        } else {
            // 如果没有设置端口转发，显示禁用状态
            lines.push(Line::from(vec![
                Span::styled("端口转发: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("禁用"),
            ]));
        }

        lines.push(Line::from(""));
        if config.use_global_proxy {
            lines.push(Line::from(vec![
                Span::styled("代理: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("使用全局代理"),
            ]));
            let global_proxy = &app.config_manager.global_config.proxy;
            if global_proxy.is_enabled() {
                lines.push(Line::from(vec![
                    Span::styled("  类型: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(format!("{:?}", global_proxy.proxy_type)),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("  地址: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(format!(
                        "{}:{}",
                        global_proxy.host,
                        global_proxy.port.unwrap_or(match global_proxy.proxy_type {
                            crate::proxy::ProxyType::Socks5 => 1080,
                            crate::proxy::ProxyType::Http => 8080,
                            crate::proxy::ProxyType::None => 0,
                        })
                    )),
                ]));
            }
        } else if let Some(proxy) = &config.proxy {
            if proxy.is_enabled() {
                lines.push(Line::from(vec![
                    Span::styled("代理: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("自定义代理"),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("  类型: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(format!("{:?}", proxy.proxy_type)),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("  地址: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(format!(
                        "{}:{}",
                        proxy.host,
                        proxy.port.unwrap_or(match proxy.proxy_type {
                            crate::proxy::ProxyType::Socks5 => 1080,
                            crate::proxy::ProxyType::Http => 8080,
                            crate::proxy::ProxyType::None => 0,
                        })
                    )),
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::styled("代理: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("无"),
                ]));
            }
        } else {
            lines.push(Line::from(vec![
                Span::styled("代理: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("无"),
            ]));
        }

        lines
    } else {
        vec![Line::from("无配置")]
    };

    let paragraph = Paragraph::new(details)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("配置详情")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}
