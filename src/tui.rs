use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::{
    app::{App, E2EStepStatus, E2ETab, Screen},
    manager::StatusState,
};

pub fn draw(frame: &mut Frame<'_>, app: &mut App) {
    match app.screen() {
        Screen::Menu => draw_menu(frame, app),
        Screen::Software => draw_software(frame, app),
        Screen::E2ETest => draw_e2e_test(frame, app),
    }
}

fn draw_menu(frame: &mut Frame<'_>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(72), Constraint::Percentage(28)])
        .split(frame.size());

    let items: Vec<ListItem> = app
        .menu_items()
        .iter()
        .map(|entry| {
            ListItem::new(Line::from(vec![
                Span::styled(
                    entry.label,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" — "),
                Span::styled(entry.description, Style::default().fg(Color::Gray)),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("MazIQ workflows")
                .borders(Borders::ALL),
        )
        .highlight_symbol(">> ")
        .highlight_style(
            Style::default()
                .bg(Color::LightCyan)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_stateful_widget(list, chunks[0], app.menu_state_mut());

    let lower = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(4),
            Constraint::Length(4),
        ])
        .split(chunks[1]);

    let message =
        Paragraph::new(app.message()).block(Block::default().title("Status").borders(Borders::ALL));
    frame.render_widget(message, lower[0]);

    let log_text = if app.show_tasks() {
        render_task_logs(app)
    } else if app.log().is_empty() {
        "No actions executed yet.".to_string()
    } else {
        app.log().join("\n")
    };
    let log_title = if app.show_tasks() {
        "Tasks"
    } else {
        "Action log"
    };
    let log =
        Paragraph::new(log_text).block(Block::default().title(log_title).borders(Borders::ALL));
    frame.render_widget(log, lower[1]);

    let dry_run_notice = if crate::options::global_dry_run() {
        " (dry-run: actions previewed only)"
    } else {
        ""
    };
    let instructions = Paragraph::new(format!(
        "Enter or 1-5 selects • ↑/↓ or j/k navigate • r refresh statuses • t toggle log • Esc/m stay here • q quit{}",
        dry_run_notice
    ))
    .block(
        Block::default()
            .title("Menu controls")
            .borders(Borders::ALL),
    );
    frame.render_widget(instructions, lower[2]);
}

fn draw_software(frame: &mut Frame<'_>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(72), Constraint::Percentage(28)])
        .split(frame.size());

    let items: Vec<ListItem> = app
        .handles()
        .iter()
        .map(|handle| {
            let state = app.status_for(handle.id);
            let (status_text, detail_span, status_color) = match state.clone() {
                StatusState::Installed { version } => {
                    let text = version.unwrap_or_else(|| "Installed".into());
                    (format!("✅ {text}"), None, Color::Green)
                }
                StatusState::NotInstalled => ("❌ Missing".into(), None, Color::Red),
                StatusState::ManualCheck(note) => (
                    "⚠ Manual".into(),
                    Some(Span::styled(
                        format!(" ({note})"),
                        Style::default().fg(Color::Yellow),
                    )),
                    Color::Yellow,
                ),
                StatusState::Unknown(note) => (
                    "?? Unknown".into(),
                    Some(Span::styled(
                        format!(" ({note})"),
                        Style::default().fg(Color::LightRed),
                    )),
                    Color::Magenta,
                ),
            };
            let status_span = Span::styled(status_text, Style::default().fg(status_color));

            let mut line = vec![
                Span::styled(
                    format!("[{}] ", handle.category),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    handle.id.name(),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" - "),
                status_span,
                Span::raw(" • "),
                Span::styled(handle.id.summary(), Style::default().fg(Color::Gray)),
            ];
            if let Some(detail) = detail_span {
                line.push(detail);
            }
            ListItem::new(Line::from(line))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("macOS Provisioning Catalog")
                .borders(Borders::ALL),
        )
        .highlight_symbol(">> ")
        .highlight_style(
            Style::default()
                .bg(Color::LightBlue)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_stateful_widget(list, chunks[0], app.list_state_mut());

    let lower = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(4),
            Constraint::Length(4),
        ])
        .split(chunks[1]);

    let message =
        Paragraph::new(app.message()).block(Block::default().title("Status").borders(Borders::ALL));
    frame.render_widget(message, lower[0]);

    let log_text = if app.show_tasks() {
        render_task_logs(app)
    } else if app.log().is_empty() {
        "No actions executed yet.".to_string()
    } else {
        app.log().join("\n")
    };
    let log_title = if app.show_tasks() {
        "Tasks"
    } else {
        "Action log"
    };
    let log =
        Paragraph::new(log_text).block(Block::default().title(log_title).borders(Borders::ALL));
    frame.render_widget(log, lower[1]);

    let controls = Paragraph::new(
        "Controls: ↑/↓ or j/k select • Enter install • u update • x uninstall • a install missing • r refresh • t toggle log • m/Esc menu • q quit",
    )
    .block(Block::default().title("Controls").borders(Borders::ALL));
    frame.render_widget(controls, lower[2]);
}

fn render_task_logs(app: &App) -> String {
    if app.task_logs().is_empty() {
        return "No tasks queued.".into();
    }
    let mut lines = Vec::new();
    for task in app.task_logs() {
        lines.push(format!("#{} {}", task.id, task.label));
        for line in &task.lines {
            lines.push(format!("  {}", line));
        }
    }
    lines.join("\n")
}

fn draw_e2e_test(frame: &mut Frame<'_>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(frame.size());

    // Title
    let title = Paragraph::new("Brew End-to-End Test")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    // Software selection or tabs
    if app.e2e_software().is_none() {
        let selection_text = "Select software: 1=Neovim, 2=btop (Esc/m to return to menu)";
        let selection = Paragraph::new(selection_text)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().title("Selection").borders(Borders::ALL));
        frame.render_widget(selection, chunks[1]);

        let info = Paragraph::new("This test will run install → update → remove for the selected software.\nUse this to verify the complete lifecycle of package management.")
            .block(Block::default().title("Info").borders(Borders::ALL));
        frame.render_widget(info, chunks[2]);
    } else {
        // Draw horizontal tabs
        let current_tab = app.e2e_tab();
        let tabs_text = format!(
            "{}  {}  {}  {}",
            if current_tab == E2ETab::Install { "[ Install ]" } else { "  Install  " },
            if current_tab == E2ETab::Update { "[ Update ]" } else { "  Update  " },
            if current_tab == E2ETab::Remove { "[ Remove ]" } else { "  Remove  " },
            if current_tab == E2ETab::Execute { "[ Execute ]" } else { "  Execute  " },
        );
        let tabs = Paragraph::new(tabs_text)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().title("Steps").borders(Borders::ALL));
        frame.render_widget(tabs, chunks[1]);

        // Tab content
        let content = match current_tab {
            E2ETab::Install => {
                let status_icon = match app.e2e_install_status() {
                    E2EStepStatus::Pending => "⏸",
                    E2EStepStatus::Running => "▶",
                    E2EStepStatus::Done => "✓",
                    E2EStepStatus::Skipped => "⊘",
                };
                format!(
                    "{} Install Step\n\nThis step will install the selected software.\n\nStatus: {}",
                    status_icon,
                    match app.e2e_install_status() {
                        E2EStepStatus::Pending => "Pending",
                        E2EStepStatus::Running => "Running...",
                        E2EStepStatus::Done => "Complete",
                        E2EStepStatus::Skipped => "Skipped",
                    }
                )
            }
            E2ETab::Update => {
                let status_icon = match app.e2e_update_status() {
                    E2EStepStatus::Pending => "⏸",
                    E2EStepStatus::Running => "▶",
                    E2EStepStatus::Done => "✓",
                    E2EStepStatus::Skipped => "⊘",
                };
                format!(
                    "{} Update Step\n\nThis step will update the software to the latest version.\n\nStatus: {}",
                    status_icon,
                    match app.e2e_update_status() {
                        E2EStepStatus::Pending => "Pending",
                        E2EStepStatus::Running => "Running...",
                        E2EStepStatus::Done => "Complete",
                        E2EStepStatus::Skipped => "Skipped",
                    }
                )
            }
            E2ETab::Remove => {
                let status_icon = match app.e2e_remove_status() {
                    E2EStepStatus::Pending => "⏸",
                    E2EStepStatus::Running => "▶",
                    E2EStepStatus::Done => "✓",
                    E2EStepStatus::Skipped => "⊘",
                };
                format!(
                    "{} Remove Step\n\nThis step will uninstall the software.\n\nStatus: {}",
                    status_icon,
                    match app.e2e_remove_status() {
                        E2EStepStatus::Pending => "Pending",
                        E2EStepStatus::Running => "Running...",
                        E2EStepStatus::Done => "Complete",
                        E2EStepStatus::Skipped => "Skipped",
                    }
                )
            }
            E2ETab::Execute => {
                if app.e2e_executing() {
                    "Execution in progress...\n\nCheck the logs below for real-time progress.".to_string()
                } else {
                    "Ready to Execute\n\nPress Enter to start the E2E test.\n\nThe test will run:\n1. Install\n2. Update\n3. Remove\n\nLogs will appear below in real-time.".to_string()
                }
            }
        };

        let content_widget = Paragraph::new(content)
            .block(Block::default().title("Step Details").borders(Borders::ALL));
        frame.render_widget(content_widget, chunks[2]);
    }

    // Instructions
    let instructions_text = if app.e2e_software().is_none() {
        "1/2 select software • Esc/m menu • q quit".to_string()
    } else if app.e2e_executing() {
        "t toggle task view • Esc/m menu • q quit".to_string()
    } else {
        "←/→ or Tab navigate tabs • Enter execute (on Execute tab) • Esc/m menu • q quit".to_string()
    };
    let instructions = Paragraph::new(instructions_text)
        .block(Block::default().title("Controls").borders(Borders::ALL));
    frame.render_widget(instructions, chunks[3]);
}
