use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Screen};

/// Render the main dashboard / menu screen
pub fn render(f: &mut Frame, app: &mut App) {
    let area = f.area();

    // Split into left menu and right info panel
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(36), Constraint::Min(0)])
        .split(area);

    render_menu(f, app, chunks[0]);
    render_info_panel(f, app, chunks[1]);
}

fn render_menu(f: &mut Frame, app: &mut App, area: Rect) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(0)])
        .split(area);

    // ASCII art title
    let title = Paragraph::new(vec![
        Line::from(Span::styled(
            " _          _   _______  _   _  ___ ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "| |   __ _ | |_|_   _| || || |_ _|",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            "| |_ / _` || '_ \\| || || || | | | ",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            "|___|\\__,_||_.__/|_| \\___/|_|___|",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            " Lability Lab Manager v0.1.0        ",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    )
    .alignment(Alignment::Left);

    f.render_widget(title, outer[0]);

    // Build menu items
    let menu_entries: Vec<(&str, Screen)> = vec![
        ("  VM Configuration", Screen::VmConfig),
        ("  Network Setup", Screen::Network),
        ("  Media Selection", Screen::Media),
        ("  Deployment", Screen::Deployment),
        ("  Help", Screen::Help),
        ("  Quit", Screen::Dashboard), // handled by key binding
    ];

    let items: Vec<ListItem> = menu_entries
        .iter()
        .map(|(label, screen)| {
            let style = if std::mem::discriminant(&app.current_screen)
                == std::mem::discriminant(screen)
            {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(Span::styled(*label, style)))
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(app.menu_index));

    let menu_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title(" Menu ")
                .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .padding(Padding::uniform(1)),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(menu_list, outer[1], &mut list_state);
}

fn render_info_panel(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(5)])
        .split(area);

    // Lab name: show editing_value with cursor when renaming
    let lab_name = if app.current_screen == Screen::Dashboard && app.is_editing {
        format!("{}█", app.editing_value)
    } else if app.lab_config.environment_name.is_empty() {
        "(no lab configured)".to_string()
    } else {
        app.lab_config.environment_name.clone()
    };

    let lab_name_style = if app.current_screen == Screen::Dashboard && app.is_editing {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
    };

    let node_count = app.lab_config.nodes.len();
    let module_count = app.lab_config.dsc_modules.len();
    let network_count = app.lab_config.networks.len();
    let status_str = app.deployment.status.to_string();

    let summary = vec![
        Line::from(vec![
            Span::styled("  Lab Name     : ", Style::default().fg(Color::DarkGray)),
            Span::styled(lab_name, lab_name_style),
        ]),
        Line::from(vec![
            Span::styled("  Nodes        : ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                node_count.to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::styled("  DSC Modules  : ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                module_count.to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Networks     : ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                network_count.to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Deployment   : ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                status_str,
                deployment_status_style(&app.deployment.status),
            ),
        ]),
    ];

    let info_block = Paragraph::new(summary)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title(" Lab Overview ")
                .title_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
                .padding(Padding::uniform(1)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(info_block, chunks[0]);

    // Key hints at the bottom
    let hint_line = if app.is_editing {
        vec![
            Span::styled(" Enter", Style::default().fg(Color::Cyan)),
            Span::raw(" Confirm  "),
            Span::styled("Esc", Style::default().fg(Color::Cyan)),
            Span::raw(" Cancel"),
        ]
    } else {
        vec![
            Span::styled(" ↑/↓", Style::default().fg(Color::Cyan)),
            Span::raw(" Navigate  "),
            Span::styled("Enter", Style::default().fg(Color::Cyan)),
            Span::raw(" Select  "),
            Span::styled("n", Style::default().fg(Color::Cyan)),
            Span::raw(" Rename Lab  "),
            Span::styled("q", Style::default().fg(Color::Cyan)),
            Span::raw(" Quit  "),
            Span::styled("?", Style::default().fg(Color::Cyan)),
            Span::raw(" Help"),
        ]
    };

    let hints = Paragraph::new(vec![Line::from(hint_line)])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .alignment(Alignment::Center);

    f.render_widget(hints, chunks[1]);
}

fn deployment_status_style(status: &crate::lability::deployment::DeploymentStatus) -> Style {
    use crate::lability::deployment::DeploymentStatus;
    match status {
        DeploymentStatus::NotStarted => Style::default().fg(Color::DarkGray),
        DeploymentStatus::InProgress => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        DeploymentStatus::Completed => Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        DeploymentStatus::Failed(_) => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        DeploymentStatus::Stopped => Style::default().fg(Color::Magenta),
    }
}
