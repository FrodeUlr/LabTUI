mod app;
#[cfg(test)]
mod app_tests;
mod lability;
mod ui;

use anyhow::Result;
use app::{App, Screen};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| draw(f, app))?;

        if let Event::Key(key) = event::read()? {
            // Only process key-press events (avoids duplicate events on some platforms)
            if key.kind != KeyEventKind::Press {
                continue;
            }
            handle_key(app, key.code, key.modifiers);
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn draw(f: &mut ratatui::Frame, app: &mut App) {
    match app.current_screen {
        Screen::Dashboard => ui::dashboard::render(f, app),
        Screen::VmConfig => ui::vm_config::render(f, app),
        Screen::Network => ui::network::render(f, app),
        Screen::Media => ui::media::render(f, app),
        Screen::Deployment => ui::deployment::render(f, app),
        Screen::Help => ui::help::render(f, app),
    }
}

fn handle_key(app: &mut App, code: KeyCode, modifiers: KeyModifiers) {
    // Ctrl-C always quits, even during editing
    if let KeyCode::Char('c') = code
        && modifiers.contains(KeyModifiers::CONTROL) {
            app.should_quit = true;
            return;
        }

    // While in editing mode, all other keys feed the editor
    if app.is_editing {
        handle_editing(app, code);
        return;
    }

    // Global keys (only when NOT editing)
    match code {
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.should_quit = true;
            return;
        }
        KeyCode::Char('?') => {
            app.current_screen = Screen::Help;
            return;
        }
        KeyCode::Esc => {
            app.go_home();
            return;
        }
        _ => {}
    }

    // Screen-specific key handling
    match app.current_screen {
        Screen::Dashboard => handle_dashboard(app, code),
        Screen::VmConfig => handle_vm_config(app, code),
        Screen::Network => handle_network(app, code),
        Screen::Media => handle_media(app, code),
        Screen::Deployment => handle_deployment(app, code),
        Screen::Help => {} // any key handled by global Esc/q above
    }
}

/// Handle key events while the user is editing an inline text field.
fn handle_editing(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char(c) => app.push_edit_char(c),
        KeyCode::Backspace => app.backspace_edit(),
        KeyCode::Enter => app.confirm_edit(),
        KeyCode::Esc => app.cancel_edit(),
        // Tab confirms the current field and moves focus to the next
        KeyCode::Tab => {
            app.confirm_edit();
            if app.current_screen == Screen::VmConfig {
                app.vm_field_next();
            }
        }
        KeyCode::BackTab => {
            app.confirm_edit();
            if app.current_screen == Screen::VmConfig {
                app.vm_field_prev();
            }
        }
        _ => {}
    }
}

fn handle_dashboard(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Up => app.menu_up(),
        KeyCode::Down => app.menu_down(),
        KeyCode::Enter => app.menu_select(),
        // 'n' renames the lab
        KeyCode::Char('n') | KeyCode::Char('N') => {
            app.current_screen = Screen::Dashboard;
            app.start_editing();
        }
        _ => {}
    }
}

fn handle_vm_config(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Up => app.node_up(),
        KeyCode::Down => app.node_down(),
        KeyCode::Char('a') | KeyCode::Char('A') => app.add_node(),
        KeyCode::Char('d') | KeyCode::Char('D') => app.delete_selected_node(),
        KeyCode::Tab => app.vm_field_next(),
        KeyCode::BackTab => app.vm_field_prev(),
        // Enter starts editing the focused text field
        KeyCode::Enter => app.start_editing(),
        KeyCode::Char('+') => app.vm_field_increment(),
        KeyCode::Char('-') => app.vm_field_decrement(),
        _ => {}
    }
}

fn handle_network(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Up => app.network_up(),
        KeyCode::Down => app.network_down(),
        KeyCode::Char('a') | KeyCode::Char('A') => app.add_network(),
        KeyCode::Char('d') | KeyCode::Char('D') => app.delete_selected_network(),
        KeyCode::Char('t') | KeyCode::Char('T') => app.toggle_switch_type(),
        // Enter renames the selected switch
        KeyCode::Enter => app.start_editing(),
        _ => {}
    }
}

fn handle_media(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Up => app.media_up(),
        KeyCode::Down => app.media_down(),
        KeyCode::Enter => app.assign_media_to_node(),
        _ => {}
    }
}

fn handle_deployment(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Up => app.action_up(),
        KeyCode::Down => app.action_down(),
        KeyCode::Enter => app.run_deployment_action(),
        // 'o' edits the output directory
        KeyCode::Char('o') | KeyCode::Char('O') => app.start_editing(),
        _ => {}
    }
}
