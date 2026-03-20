use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph},
    Frame,
};

use crate::app::App;
use crate::lability::config::default_media_list;

/// Render the media selection screen
pub fn render(f: &mut Frame, app: &mut App) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(10)])
        .split(area);

    render_media_list(f, app, chunks[0]);
    render_media_detail(f, app, chunks[1]);
}

fn render_media_list(f: &mut Frame, app: &mut App, area: Rect) {
    let media = default_media_list();

    let items: Vec<ListItem> = media
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let is_selected = Some(i) == app.selected_media;
            let style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let id_style = if is_selected {
                style
            } else {
                Style::default().fg(Color::Yellow)
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("  {:40}", entry.id), id_style),
                Span::styled(
                    format!("  {}", entry.architecture),
                    if is_selected {
                        style
                    } else {
                        Style::default().fg(Color::DarkGray)
                    },
                ),
            ]))
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(app.selected_media);

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title(" Available Media ")
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

    f.render_stateful_widget(list, area, &mut list_state);
}

fn render_media_detail(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    let media = default_media_list();
    let lines: Vec<Line> = if let Some(idx) = app.selected_media {
        if let Some(entry) = media.get(idx) {
            vec![
                Line::from(vec![
                    Span::styled("  ID          : ", Style::default().fg(Color::DarkGray)),
                    Span::styled(entry.id.clone(), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::styled("  Description : ", Style::default().fg(Color::DarkGray)),
                    Span::styled(entry.description.clone(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("  Architecture: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(entry.architecture.clone(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("  Type        : ", Style::default().fg(Color::DarkGray)),
                    Span::styled(entry.media_type.clone(), Style::default().fg(Color::White)),
                ]),
            ]
        } else {
            vec![Line::from(Span::styled(
                "  No media selected.",
                Style::default().fg(Color::DarkGray),
            ))]
        }
    } else {
        vec![Line::from(Span::styled(
            "  Select a media entry above.",
            Style::default().fg(Color::DarkGray),
        ))]
    };

    let detail = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .title(" Media Details ")
            .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .padding(Padding::uniform(1)),
    );
    f.render_widget(detail, chunks[0]);

    // Usage hints
    let hints = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(" ↑/↓", Style::default().fg(Color::Cyan)),
            Span::raw(" Browse  "),
        ]),
        Line::from(vec![
            Span::styled(" Enter", Style::default().fg(Color::Cyan)),
            Span::raw(" Assign to selected node  "),
        ]),
        Line::from(vec![
            Span::styled(" Esc", Style::default().fg(Color::Cyan)),
            Span::raw(" Back to menu  "),
        ]),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" Keys ")
            .title_style(Style::default().fg(Color::DarkGray))
            .padding(Padding::uniform(1)),
    )
    .alignment(Alignment::Left);
    f.render_widget(hints, chunks[1]);
}
