use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::lability::config::{NetworkAdapter, NodeConfig};

/// Which field in the node-edit form is focused
#[derive(Debug, Clone, PartialEq, Default)]
pub enum VmField {
    #[default]
    NodeName,
    Role,
    CpuCount,
    StartMemory,
    MinMemory,
    MaxMemory,
    MediaId,
    IpAddress,
}

/// Render the VM configuration screen
pub fn render(f: &mut Frame, app: &mut App) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    render_node_list(f, app, chunks[0]);
    render_node_editor(f, app, chunks[1]);
}

fn render_node_list(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(5)])
        .split(area);

    let items: Vec<ListItem> = app
        .lab_config
        .nodes
        .iter()
        .enumerate()
        .map(|(i, node)| {
            let style = if Some(i) == app.selected_node {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("  {:20}", node.node_name), style),
                Span::styled(
                    format!(" {} CPU  {}GB", node.cpu_count, node.start_up_memory_bytes / (1024 * 1024 * 1024)),
                    style.fg(if Some(i) == app.selected_node { Color::Black } else { Color::DarkGray }),
                ),
            ]))
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(app.selected_node);

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title(" Nodes ")
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

    let hints = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(" a", Style::default().fg(Color::Cyan)),
            Span::raw(" Add  "),
            Span::styled("d", Style::default().fg(Color::Cyan)),
            Span::raw(" Delete  "),
            Span::styled("↑/↓", Style::default().fg(Color::Cyan)),
            Span::raw(" Select  "),
            Span::styled("Esc", Style::default().fg(Color::Cyan)),
            Span::raw(" Back"),
        ]),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    )
    .alignment(Alignment::Center);

    f.render_widget(hints, chunks[1]);
}

fn render_node_editor(f: &mut Frame, app: &mut App, area: Rect) {
    if let Some(idx) = app.selected_node {
        if let Some(node) = app.lab_config.nodes.get(idx) {
            let node = node.clone();
            render_node_fields(f, app, area, &node);
        }
    } else {
        let placeholder = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(
                "  Select or add a node to configure it.",
                Style::default().fg(Color::DarkGray),
            )),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title(" Node Editor ")
                .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        )
        .wrap(Wrap { trim: false });
        f.render_widget(placeholder, area);
    }
}

fn render_node_fields(f: &mut Frame, app: &mut App, area: Rect, node: &NodeConfig) {
    let fields: Vec<(String, String, VmField)> = vec![
        ("Node Name".to_string(), node.node_name.clone(), VmField::NodeName),
        ("Role".to_string(), node.role.clone(), VmField::Role),
        ("CPU Count".to_string(), node.cpu_count.to_string(), VmField::CpuCount),
        (
            "Startup Memory (GB)".to_string(),
            format!("{:.1}", node.start_up_memory_bytes as f64 / (1024.0 * 1024.0 * 1024.0)),
            VmField::StartMemory,
        ),
        (
            "Min Memory (GB)".to_string(),
            format!("{:.1}", node.minimum_memory_bytes as f64 / (1024.0 * 1024.0 * 1024.0)),
            VmField::MinMemory,
        ),
        (
            "Max Memory (GB)".to_string(),
            format!("{:.1}", node.maximum_memory_bytes as f64 / (1024.0 * 1024.0 * 1024.0)),
            VmField::MaxMemory,
        ),
        ("Media ID".to_string(), node.media_id.clone(), VmField::MediaId),
        (
            "IP Address".to_string(),
            node.ip_address.clone().unwrap_or_default(),
            VmField::IpAddress,
        ),
    ];

    let rows: Vec<Line> = fields
        .iter()
        .map(|(label, value, field)| {
            let is_focused = &app.vm_focused_field == field;
            let label_style = if is_focused {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            let value_style = if is_focused {
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let cursor = if is_focused { "▌" } else { " " };
            Line::from(vec![
                Span::styled(format!("  {:22}", label), label_style),
                Span::styled(": ", Style::default().fg(Color::DarkGray)),
                Span::styled(value, value_style),
                Span::styled(cursor, Style::default().fg(Color::Cyan)),
            ])
        })
        .collect();

    // DSC modules section
    let mut all_rows = rows;
    all_rows.push(Line::from(""));
    all_rows.push(Line::from(Span::styled(
        "  DSC Resources:",
        Style::default().fg(Color::DarkGray),
    )));
    for r in &node.dsc_resources {
        all_rows.push(Line::from(Span::styled(
            format!("    • {r}"),
            Style::default().fg(Color::White),
        )));
    }
    if node.dsc_resources.is_empty() {
        all_rows.push(Line::from(Span::styled(
            "    (none)",
            Style::default().fg(Color::DarkGray),
        )));
    }

    // Network adapters
    all_rows.push(Line::from(""));
    all_rows.push(Line::from(Span::styled(
        "  Network Adapters:",
        Style::default().fg(Color::DarkGray),
    )));
    for na in &node.network_adapters {
        all_rows.push(Line::from(Span::styled(
            format!("    • Switch: {}", na.switch_name),
            Style::default().fg(Color::White),
        )));
    }
    if node.network_adapters.is_empty() {
        all_rows.push(Line::from(Span::styled(
            "    (none)",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let para = Paragraph::new(all_rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title(format!(" Node: {} ", node.node_name))
                .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .padding(Padding::uniform(1)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(para, area);
}

/// Create a default new node
pub fn new_node(index: usize) -> NodeConfig {
    NodeConfig {
        node_name: format!("Node{:02}", index + 1),
        network_adapters: vec![NetworkAdapter {
            switch_name: "Default Switch".to_string(),
            mac_address: None,
        }],
        ..NodeConfig::default()
    }
}
