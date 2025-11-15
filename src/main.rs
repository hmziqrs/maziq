use std::{
    error::Error,
    io,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

mod app;
mod catalog;
mod cli;
mod manager;
mod templates;
mod tui;

fn main() -> std::process::ExitCode {
    // Parse CLI first; if no subcommand, fall back to the interactive TUI.
    let cli_args = cli::Cli::parse();
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
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = app::App::new();
    let tick_rate = Duration::from_millis(200);
    let mut last_tick = Instant::now();

    let should_quit = Arc::new(AtomicBool::new(false));
    {
        let signal_flag = Arc::clone(&should_quit);
        ctrlc::set_handler(move || {
            signal_flag.store(true, Ordering::SeqCst);
        })?;
    }

    loop {
        terminal.draw(|frame| tui::draw(frame, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    app.quit = true;
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') => app.quit = true,
                    KeyCode::Esc => app.enter_menu(),
                    KeyCode::Char('m') => app.enter_menu(),
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Enter | KeyCode::Char(' ') => match app.screen() {
                        app::Screen::Menu => app.activate_menu(),
                        app::Screen::Software => app.install_selected(),
                    },
                    KeyCode::Char('u') => {
                        if matches!(app.screen(), app::Screen::Software) {
                            app.update_selected();
                        }
                    }
                    KeyCode::Char('x') => {
                        if matches!(app.screen(), app::Screen::Software) {
                            app.uninstall_selected();
                        }
                    }
                    KeyCode::Char('a') => {
                        if matches!(app.screen(), app::Screen::Software) {
                            app.install_all_missing();
                        }
                    }
                    KeyCode::Char('r') => app.refresh_statuses_with_feedback(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        if should_quit.load(Ordering::SeqCst) {
            app.quit = true;
        }

        if app.quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
