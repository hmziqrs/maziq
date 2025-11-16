use std::{
    error::Error,
    io,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use clap::Parser;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::Terminal;
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers};

mod app;
mod catalog;
mod cli;
mod configurator;
mod history;
mod manager;
mod options;
mod templates;
mod messages;
mod components;
mod realm_app;

fn main() -> std::process::ExitCode {
    // Parse CLI first; if no subcommand, fall back to the interactive TUI.
    let cli_args = cli::Cli::parse();
    if cli_args.dry_run {
        options::set_global_dry_run(true);
    }

    if let Some(command) = cli_args.command {
        return match cli::run(command) {
            Ok(_) => std::process::ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("{err}");
                std::process::ExitCode::FAILURE
            }
        };
    }

    match run_tui() {
        Ok(_) => std::process::ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            std::process::ExitCode::FAILURE
        }
    }
}

fn run_tui() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = realm_app::RealmApp::new();

    let should_quit = Arc::new(AtomicBool::new(false));
    {
        let signal_flag = Arc::clone(&should_quit);
        ctrlc::set_handler(move || {
            signal_flag.store(true, Ordering::SeqCst);
        })?;
    }

    loop {
        // Tick the application
        app.tick();

        // Render the view
        app.view(&mut terminal);

        // Poll for events from crossterm
        if crossterm::event::poll(std::time::Duration::from_millis(10))? {
            let crossterm_event = crossterm::event::read()?;

            // Convert crossterm event to tuirealm event
            let event = match crossterm_event {
                crossterm::event::Event::Key(key) => {
                    // Handle Ctrl+C
                    if key.code == crossterm::event::KeyCode::Char('c')
                        && key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL)
                    {
                        break;
                    }

                    // Convert crossterm KeyEvent to tuirealm Event
                    Some(convert_key_event(key))
                }
                _ => None,
            };

            if let Some(ev) = event {
                if let Some(msg) = app.handle_event(ev) {
                    app.update(msg);
                }
            }
        }

        // Check quit conditions
        if should_quit.load(Ordering::SeqCst) || app.should_quit() {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn convert_key_event(key: crossterm::event::KeyEvent) -> Event<tuirealm::NoUserEvent> {
    let key_code = match key.code {
        crossterm::event::KeyCode::Backspace => Key::Backspace,
        crossterm::event::KeyCode::Enter => Key::Enter,
        crossterm::event::KeyCode::Left => Key::Left,
        crossterm::event::KeyCode::Right => Key::Right,
        crossterm::event::KeyCode::Up => Key::Up,
        crossterm::event::KeyCode::Down => Key::Down,
        crossterm::event::KeyCode::Home => Key::Home,
        crossterm::event::KeyCode::End => Key::End,
        crossterm::event::KeyCode::PageUp => Key::PageUp,
        crossterm::event::KeyCode::PageDown => Key::PageDown,
        crossterm::event::KeyCode::Tab => Key::Tab,
        crossterm::event::KeyCode::BackTab => Key::BackTab,
        crossterm::event::KeyCode::Delete => Key::Delete,
        crossterm::event::KeyCode::Insert => Key::Insert,
        crossterm::event::KeyCode::F(n) => Key::Function(n),
        crossterm::event::KeyCode::Char(c) => Key::Char(c),
        crossterm::event::KeyCode::Null => Key::Null,
        crossterm::event::KeyCode::Esc => Key::Esc,
        _ => Key::Null,
    };

    let modifiers = if key.modifiers.contains(crossterm::event::KeyModifiers::SHIFT) {
        KeyModifiers::SHIFT
    } else if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
        KeyModifiers::CONTROL
    } else if key.modifiers.contains(crossterm::event::KeyModifiers::ALT) {
        KeyModifiers::ALT
    } else {
        KeyModifiers::NONE
    };

    Event::Keyboard(KeyEvent {
        code: key_code,
        modifiers,
    })
}
