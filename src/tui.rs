use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::{
    app::{App, Screen},
    manager::StatusState,
};

pub fn draw(frame: &mut Frame<'_>, app: &mut App) {
    match app.screen() {
        Screen::Menu => draw_menu(frame, app),
        Screen::Software => draw_software(frame, app),
    }
}

fn draw_menu(frame: &mut Frame<'_>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
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

    let instructions = Paragraph::new(
        "Enter selects • ↑/↓ or j/k navigate • r refresh statuses • Esc/m return to menu • q quit",
    )
    .block(
        Block::default()
            .title("Menu controls")
            .borders(Borders::ALL),
    );
    frame.render_widget(instructions, chunks[1]);
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

    let log_text = if app.log().is_empty() {
        "No actions executed yet.".to_string()
    } else {
        app.log().join("\n")
    };
    let log =
        Paragraph::new(log_text).block(Block::default().title("Action log").borders(Borders::ALL));
    frame.render_widget(log, lower[1]);

    let controls = Paragraph::new(
        "Controls: ↑/↓ or j/k select • Enter install • u update • x uninstall • a install missing • r refresh • m/Esc menu • q quit",
    )
    .block(Block::default().title("Controls").borders(Borders::ALL));
    frame.render_widget(controls, lower[2]);
}
