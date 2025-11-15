use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::App;

pub fn draw(frame: &mut Frame<'_>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(72), Constraint::Percentage(28)])
        .split(frame.size());

    let items: Vec<ListItem> = app
        .handles()
        .iter()
        .map(|handle| {
            let status = app.status_for(handle.id);
            let status_text = if status.installed {
                Span::styled("✅ Installed", Style::default().fg(Color::Green))
            } else {
                Span::styled("❌ Missing", Style::default().fg(Color::Red))
            };
            let error_span = status.error.as_ref().map(|err| {
                Span::styled(
                    format!(" (Error: {err})"),
                    Style::default().fg(Color::LightRed),
                )
            });

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
                status_text,
                Span::raw(" • "),
                Span::styled(handle.id.summary(), Style::default().fg(Color::Gray)),
            ];
            if let Some(err) = error_span {
                line.push(err);
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
        .constraints([Constraint::Length(5), Constraint::Min(0)])
        .split(chunks[1]);

    let message =
        Paragraph::new(app.message()).block(Block::default().title("Status").borders(Borders::ALL));
    frame.render_widget(message, lower[0]);

    let controls = Paragraph::new(
        "Controls: ↑/↓ or j/k navigate • Enter installs selection • a installs all missing • r refreshes statuses • q quits",
    )
    .block(Block::default().title("Controls").borders(Borders::ALL));
    frame.render_widget(controls, lower[1]);
}
