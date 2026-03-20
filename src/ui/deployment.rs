use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::lability::deployment::{DeploymentAction, DeploymentStatus};

const ACTIONS: &[DeploymentAction] = &[
    DeploymentAction::GenerateConfig,
    DeploymentAction::Start,
    DeploymentAction::Stop,
    DeploymentAction::Reset,
    DeploymentAction::Delete,
];

/// Render the deployment management screen
pub fn render(f: &mut Frame, app: &mut App) {
    let area = f.area();

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Min(0)])
        .split(area);

    render_status_bar(f, app, outer[0]);

    let inner = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(0)])
        .split(outer[1]);

    render_action_panel(f, app, inner[0]);
    render_log_panel(f, app, inner[1]);
}

fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let status_str = app.deployment.status.to_string();
    let (status_color, indicator) = match &app.deployment.status {
        DeploymentStatus::NotStarted => (Color::DarkGray, "○"),
        DeploymentStatus::InProgress => (Color::Yellow, "◉"),
        DeploymentStatus::Completed => (Color::Green, "●"),
        DeploymentStatus::Failed(_) => (Color::Red, "✗"),
        DeploymentStatus::Stopped => (Color::Magenta, "◌"),
    };

    let env = if app.lab_config.environment_name.is_empty() {
        "(not configured)".to_string()
    } else {
        app.lab_config.environment_name.clone()
    };

    // Show live editing value for the output dir when the user presses 'o'
    let out_dir_display = if app.is_editing {
        format!("{}█", app.editing_value)
    } else {
        app.lab_output_dir.clone()
    };
    let out_dir_style = if app.is_editing {
        Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Yellow)
    };

    let rows = vec![
        Line::from(vec![
            Span::styled(format!("  {} Status: ", indicator), Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
            Span::styled(status_str, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
            Span::raw("    "),
            Span::styled("Lab: ", Style::default().fg(Color::DarkGray)),
            Span::styled(env, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw("    "),
            Span::styled(format!("Nodes: {}", app.lab_config.nodes.len()), Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  Output Dir   : ", Style::default().fg(Color::DarkGray)),
            Span::styled(out_dir_display, out_dir_style),
            if !app.is_editing {
                Span::styled("  [o to change]", Style::default().fg(Color::DarkGray))
            } else {
                Span::styled("  [Enter/Esc]", Style::default().fg(Color::DarkGray))
            },
        ]),
    ];

    let bar = Paragraph::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title(" Deployment Status ")
                .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        )
        .alignment(Alignment::Left);

    f.render_widget(bar, area);
}

fn render_action_panel(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(5)])
        .split(area);

    let items: Vec<ListItem> = ACTIONS
        .iter()
        .enumerate()
        .map(|(i, action)| {
            let focused = i == app.selected_action;
            let style = if focused {
                Style::default()
                    .fg(Color::Black)
                    .bg(action_fg(action))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(action_fg(action))
            };
            ListItem::new(Line::from(Span::styled(
                format!("  {}", action),
                style,
            )))
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(app.selected_action));

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title(" Actions ")
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

    let hints = Paragraph::new(vec![Line::from(vec![
        Span::styled(" Enter", Style::default().fg(Color::Cyan)),
        Span::raw(" Run  "),
        Span::styled("Esc", Style::default().fg(Color::Cyan)),
        Span::raw(" Back"),
    ])])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    )
    .alignment(Alignment::Center);

    f.render_widget(hints, chunks[1]);
}

fn render_log_panel(f: &mut Frame, app: &App, area: Rect) {
    let max_lines = (area.height.saturating_sub(4)) as usize;
    let log_lines = &app.deployment.log_lines;
    let start = if log_lines.len() > max_lines {
        log_lines.len() - max_lines
    } else {
        0
    };
    let visible: Vec<Line> = log_lines[start..]
        .iter()
        .map(|l| {
            let style = if l.starts_with("[ERROR]") || l.starts_with("✗") {
                Style::default().fg(Color::Red)
            } else if l.starts_with("[OK]") || l.starts_with("✓") {
                Style::default().fg(Color::Green)
            } else if l.starts_with("[INFO]") {
                Style::default().fg(Color::Cyan)
            } else if l.starts_with("[WARN]") {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::White)
            };
            Line::from(Span::styled(format!("  {l}"), style))
        })
        .collect();

    let log_para = Paragraph::new(visible)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title(" Deployment Log ")
                .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .padding(Padding::uniform(1)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(log_para, area);
}

fn action_fg(action: &DeploymentAction) -> Color {
    match action {
        DeploymentAction::GenerateConfig => Color::Blue,
        DeploymentAction::Start => Color::Green,
        DeploymentAction::Stop => Color::Yellow,
        DeploymentAction::Reset => Color::Cyan,
        DeploymentAction::Delete => Color::Red,
    }
}
