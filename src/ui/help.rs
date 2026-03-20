use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

/// Render the help screen
pub fn render(f: &mut Frame, _app: &mut App) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(5)])
        .split(area);

    let help_text = vec![
        Line::from(Span::styled(
            "  LabTUI – Lability Lab Manager",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  ABOUT",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "  LabTUI is a terminal user interface for managing Lability",
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            "  (https://github.com/VirtualEngine/Lability) lab environments.",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "  It allows you to define VMs, networks, DSC resources and deploy",
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            "  Hyper-V labs without writing PowerShell by hand.",
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  GLOBAL KEY BINDINGS",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::styled("  ↑ / ↓       ", Style::default().fg(Color::Cyan)),
            Span::raw("Move selection up / down"),
        ]),
        Line::from(vec![
            Span::styled("  Enter       ", Style::default().fg(Color::Cyan)),
            Span::raw("Confirm / activate selected item"),
        ]),
        Line::from(vec![
            Span::styled("  Esc         ", Style::default().fg(Color::Cyan)),
            Span::raw("Go back to main menu  (or cancel an active edit)"),
        ]),
        Line::from(vec![
            Span::styled("  q / Q       ", Style::default().fg(Color::Cyan)),
            Span::raw("Quit the application  (disabled while editing)"),
        ]),
        Line::from(vec![
            Span::styled("  ?           ", Style::default().fg(Color::Cyan)),
            Span::raw("Open this help screen"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  INLINE TEXT EDITING",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::styled("  Enter       ", Style::default().fg(Color::Cyan)),
            Span::raw("Start editing the focused text field"),
        ]),
        Line::from(vec![
            Span::styled("  Backspace   ", Style::default().fg(Color::Cyan)),
            Span::raw("Delete the last character"),
        ]),
        Line::from(vec![
            Span::styled("  Enter       ", Style::default().fg(Color::Cyan)),
            Span::raw("Confirm edit and save value"),
        ]),
        Line::from(vec![
            Span::styled("  Tab / S-Tab ", Style::default().fg(Color::Cyan)),
            Span::raw("Confirm edit and move to next / previous field"),
        ]),
        Line::from(vec![
            Span::styled("  Esc         ", Style::default().fg(Color::Cyan)),
            Span::raw("Cancel edit without saving"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  DASHBOARD  (main screen)",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::styled("  n           ", Style::default().fg(Color::Cyan)),
            Span::raw("Rename the lab (inline edit)"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  VM CONFIGURATION  (menu → VM Configuration)",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::styled("  a           ", Style::default().fg(Color::Cyan)),
            Span::raw("Add a new node"),
        ]),
        Line::from(vec![
            Span::styled("  d           ", Style::default().fg(Color::Cyan)),
            Span::raw("Delete selected node"),
        ]),
        Line::from(vec![
            Span::styled("  Tab / S-Tab ", Style::default().fg(Color::Cyan)),
            Span::raw("Move between fields (confirms any active text edit)"),
        ]),
        Line::from(vec![
            Span::styled("  Enter       ", Style::default().fg(Color::Cyan)),
            Span::raw("Edit text fields: Node Name, Role, Media ID, IP Address"),
        ]),
        Line::from(vec![
            Span::styled("  +/-         ", Style::default().fg(Color::Cyan)),
            Span::raw("Increment / decrement numeric fields (CPU, memory)"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  NETWORK SETUP  (menu → Network Setup)",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::styled("  a           ", Style::default().fg(Color::Cyan)),
            Span::raw("Add virtual switch"),
        ]),
        Line::from(vec![
            Span::styled("  d           ", Style::default().fg(Color::Cyan)),
            Span::raw("Delete selected switch"),
        ]),
        Line::from(vec![
            Span::styled("  Enter       ", Style::default().fg(Color::Cyan)),
            Span::raw("Rename selected switch (inline edit)"),
        ]),
        Line::from(vec![
            Span::styled("  t           ", Style::default().fg(Color::Cyan)),
            Span::raw("Cycle switch type (Internal → External → Private)"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  MEDIA SELECTION  (menu → Media Selection)",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::styled("  Enter       ", Style::default().fg(Color::Cyan)),
            Span::raw("Assign selected media to the current node"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  DEPLOYMENT  (menu → Deployment)",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::styled("  o           ", Style::default().fg(Color::Cyan)),
            Span::raw("Edit the output directory (where .psd1 / .ps1 are written)"),
        ]),
        Line::from(vec![
            Span::styled("  Enter       ", Style::default().fg(Color::Cyan)),
            Span::raw("Execute the highlighted action"),
        ]),
        Line::from(vec![
            Span::styled("  Gen. Config ", Style::default().fg(Color::Blue)),
            Span::raw("Writes <Lab>.psd1 and <Lab>.ps1 to the output directory"),
        ]),
        Line::from(vec![
            Span::styled("  Start Lab   ", Style::default().fg(Color::Green)),
            Span::raw("Generates config files, compiles DSC MOFs, then starts the lab"),
        ]),
        Line::from(vec![
            Span::styled("  Stop Lab    ", Style::default().fg(Color::Yellow)),
            Span::raw("Stops all lab VMs"),
        ]),
        Line::from(vec![
            Span::styled("  Reset Lab   ", Style::default().fg(Color::Cyan)),
            Span::raw("Deletes and recreates the lab from scratch"),
        ]),
        Line::from(vec![
            Span::styled("  Delete Lab  ", Style::default().fg(Color::Red)),
            Span::raw("Removes the lab and all VMs permanently"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  GENERATED FILES",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "  <LabName>.psd1  Lability configuration data (AllNodes + NonNodeData)",
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            "  <LabName>.ps1   DSC configuration script (compiles to MOF)",
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  REQUIREMENTS",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "  • Windows 10/11 or Windows Server 2016+ with Hyper-V enabled",
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            "  • PowerShell 5.1+ or PowerShell 7+",
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            "  • Lability module: Install-Module -Name Lability",
            Style::default().fg(Color::White),
        )),
    ];

    let help_block = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title(" Help ")
                .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .padding(Padding::uniform(1)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(help_block, chunks[0]);

    let hints = Paragraph::new(vec![Line::from(vec![
        Span::styled(" Esc / q", Style::default().fg(Color::Cyan)),
        Span::raw("  Return to menu"),
    ])])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    )
    .alignment(Alignment::Center);

    f.render_widget(hints, chunks[1]);
}
