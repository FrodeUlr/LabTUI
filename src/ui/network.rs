use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph},
    Frame,
};

use crate::app::App;
use crate::lability::config::{SwitchType, VirtualSwitch};

/// Render the network configuration screen
pub fn render(f: &mut Frame, app: &mut App) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(area);

    render_switch_list(f, app, chunks[0]);
    render_switch_detail(f, app, chunks[1]);
}

fn render_switch_list(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(5)])
        .split(area);

    let items: Vec<ListItem> = app
        .lab_config
        .networks
        .iter()
        .enumerate()
        .map(|(i, sw)| {
            let focused = Some(i) == app.selected_network;
            // Show live editing value if we're renaming this switch
            let display_name = if focused && app.is_editing {
                format!("{}█", app.editing_value)
            } else {
                sw.name.clone()
            };
            let style = if focused {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let type_style = if focused {
                style.fg(Color::Black)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("  {:22}", display_name), style),
                Span::styled(format!("[{}]", sw.switch_type), type_style),
            ]))
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(app.selected_network);

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title(" Virtual Switches ")
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

    f.render_stateful_widget(list, chunks[0], &mut list_state);

    let hint_spans = if app.is_editing {
        vec![
            Span::styled(" Enter", Style::default().fg(Color::Cyan)),
            Span::raw(" Confirm  "),
            Span::styled("Esc", Style::default().fg(Color::Cyan)),
            Span::raw(" Cancel"),
        ]
    } else {
        vec![
            Span::styled(" a", Style::default().fg(Color::Cyan)),
            Span::raw(" Add  "),
            Span::styled("d", Style::default().fg(Color::Cyan)),
            Span::raw(" Delete  "),
            Span::styled("Enter", Style::default().fg(Color::Cyan)),
            Span::raw(" Rename  "),
            Span::styled("t", Style::default().fg(Color::Cyan)),
            Span::raw(" Toggle Type  "),
            Span::styled("Esc", Style::default().fg(Color::Cyan)),
            Span::raw(" Back"),
        ]
    };

    let hints = Paragraph::new(vec![Line::from(hint_spans)])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    )
    .alignment(Alignment::Center);

    f.render_widget(hints, chunks[1]);
}

fn render_switch_detail(f: &mut Frame, app: &App, area: Rect) {
    if let Some(idx) = app.selected_network
        && let Some(sw) = app.lab_config.networks.get(idx) {
            let rows = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("  Name           : ", Style::default().fg(Color::DarkGray)),
                    Span::styled(sw.name.clone(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::styled("  Type           : ", Style::default().fg(Color::DarkGray)),
                    Span::styled(sw.switch_type.to_string(), Style::default().fg(Color::Yellow)),
                ]),
                Line::from(vec![
                    Span::styled("  Management OS  : ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        if sw.allow_management_os { "Yes" } else { "No" },
                        Style::default().fg(if sw.allow_management_os { Color::Green } else { Color::Red }),
                    ),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "  Switch Types:",
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(Span::styled(
                    "    Internal  – Host ↔ VMs only",
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(Span::styled(
                    "    External  – Host ↔ VMs ↔ Physical network",
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(Span::styled(
                    "    Private   – VMs only, no host access",
                    Style::default().fg(Color::DarkGray),
                )),
            ];

            let para = Paragraph::new(rows).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue))
                    .title(format!(" Switch: {} ", sw.name))
                    .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                    .padding(Padding::uniform(1)),
            );
            f.render_widget(para, area);
            return;
        }

    let placeholder = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Select or add a virtual switch.",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .title(" Switch Detail ")
            .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    );
    f.render_widget(placeholder, area);
}

/// Build a new default virtual switch
pub fn new_switch(index: usize) -> VirtualSwitch {
    VirtualSwitch {
        name: format!("LabSwitch{:02}", index + 1),
        switch_type: SwitchType::Internal,
        allow_management_os: true,
    }
}

/// Cycle a switch through Internal → External → Private → Internal
pub fn cycle_switch_type(sw: &mut VirtualSwitch) {
    sw.switch_type = match sw.switch_type {
        SwitchType::Internal => SwitchType::External,
        SwitchType::External => SwitchType::Private,
        SwitchType::Private => SwitchType::Internal,
    };
}
