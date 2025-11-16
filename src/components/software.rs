use std::collections::HashMap;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use tuirealm::{
    Component, Event, MockComponent, State, StateValue,
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent, KeyModifiers},
};

use crate::{
    catalog::{SoftwareHandle, SoftwareId},
    manager::StatusState,
    messages::AppMsg,
};

pub struct SoftwareComponent {
    handles: Vec<SoftwareHandle>,
    statuses: HashMap<SoftwareId, StatusState>,
    selected: usize,
    message: String,
    log_lines: Vec<String>,
    show_tasks: bool,
    task_lines: Vec<String>,
}

impl SoftwareComponent {
    pub fn new(handles: Vec<SoftwareHandle>) -> Self {
        Self {
            handles,
            statuses: HashMap::new(),
            selected: 0,
            message: "Browse the catalog. Use Enter/u/x to install/update/uninstall.".into(),
            log_lines: Vec::new(),
            show_tasks: true,
            task_lines: Vec::new(),
        }
    }

    pub fn set_statuses(&mut self, statuses: HashMap<SoftwareId, StatusState>) {
        self.statuses = statuses;
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn set_log_lines(&mut self, lines: Vec<String>) {
        self.log_lines = lines;
    }

    pub fn set_task_lines(&mut self, lines: Vec<String>) {
        self.task_lines = lines;
    }

    pub fn set_show_tasks(&mut self, show: bool) {
        self.show_tasks = show;
    }

    pub fn selected_software(&self) -> Option<SoftwareId> {
        self.handles.get(self.selected).map(|h| h.id)
    }

    fn status_for(&self, id: SoftwareId) -> StatusState {
        self.statuses
            .get(&id)
            .cloned()
            .unwrap_or(StatusState::NotInstalled)
    }
}

impl MockComponent for SoftwareComponent {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(72), Constraint::Percentage(28)])
            .split(area);

        // Render software list
        let items: Vec<ListItem> = self
            .handles
            .iter()
            .map(|handle| {
                let state = self.status_for(handle.id);
                let (status_text, detail_span, status_color) = match state {
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

        let mut list_state = ratatui::widgets::ListState::default();
        list_state.select(Some(self.selected));

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

        frame.render_stateful_widget(list, chunks[0], &mut list_state);

        // Render lower section
        let lower = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(4),
                Constraint::Length(4),
            ])
            .split(chunks[1]);

        // Status message
        let message = Paragraph::new(self.message.as_str())
            .block(Block::default().title("Status").borders(Borders::ALL));
        frame.render_widget(message, lower[0]);

        // Log panel
        let log_text = if self.show_tasks {
            if self.task_lines.is_empty() {
                "No tasks queued.".to_string()
            } else {
                self.task_lines.join("\n")
            }
        } else if self.log_lines.is_empty() {
            "No actions executed yet.".to_string()
        } else {
            self.log_lines.join("\n")
        };

        let log_title = if self.show_tasks { "Tasks" } else { "Action log" };
        let log = Paragraph::new(log_text)
            .block(Block::default().title(log_title).borders(Borders::ALL));
        frame.render_widget(log, lower[1]);

        // Controls
        let controls = Paragraph::new(
            "Controls: ↑/↓ or j/k select • Enter install • u update • x uninstall • a install missing • r refresh • t toggle log • m/Esc menu • q quit",
        )
        .block(Block::default().title("Controls").borders(Borders::ALL));
        frame.render_widget(controls, lower[2]);
    }

    fn query(&self, attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
        match attr {
            tuirealm::Attribute::Value => Some(tuirealm::AttrValue::Number(self.selected as isize)),
            _ => None,
        }
    }

    fn attr(&mut self, attr: tuirealm::Attribute, value: tuirealm::AttrValue) {
        match attr {
            tuirealm::Attribute::Value => {
                if let tuirealm::AttrValue::Number(n) = value {
                    self.selected = n.max(0).min((self.handles.len() - 1) as isize) as usize;
                }
            }
            _ => {}
        }
    }

    fn state(&self) -> State {
        State::One(StateValue::Usize(self.selected))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(tuirealm::command::Direction::Down) => {
                if !self.handles.is_empty() {
                    self.selected = (self.selected + 1) % self.handles.len();
                    CmdResult::Changed(State::One(StateValue::Usize(self.selected)))
                } else {
                    CmdResult::None
                }
            }
            Cmd::Move(tuirealm::command::Direction::Up) => {
                if !self.handles.is_empty() {
                    self.selected = if self.selected == 0 {
                        self.handles.len() - 1
                    } else {
                        self.selected - 1
                    };
                    CmdResult::Changed(State::One(StateValue::Usize(self.selected)))
                } else {
                    CmdResult::None
                }
            }
            Cmd::Submit => {
                CmdResult::Submit(State::One(StateValue::Usize(self.selected)))
            }
            _ => CmdResult::None,
        }
    }
}

impl Component<AppMsg, tuirealm::NoUserEvent> for SoftwareComponent {
    fn on(&mut self, ev: Event<tuirealm::NoUserEvent>) -> Option<AppMsg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down | Key::Char('j'),
                modifiers: KeyModifiers::NONE,
            }) => {
                self.perform(Cmd::Move(tuirealm::command::Direction::Down));
                Some(AppMsg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Up | Key::Char('k'),
                modifiers: KeyModifiers::NONE,
            }) => {
                self.perform(Cmd::Move(tuirealm::command::Direction::Up));
                Some(AppMsg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                modifiers: KeyModifiers::NONE,
            }) => {
                if let Some(id) = self.selected_software() {
                    Some(AppMsg::InstallSoftware(id))
                } else {
                    Some(AppMsg::None)
                }
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('u'),
                modifiers: KeyModifiers::NONE,
            }) => {
                if let Some(id) = self.selected_software() {
                    Some(AppMsg::UpdateSoftware(id))
                } else {
                    Some(AppMsg::None)
                }
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('x'),
                modifiers: KeyModifiers::NONE,
            }) => {
                if let Some(id) = self.selected_software() {
                    Some(AppMsg::UninstallSoftware(id))
                } else {
                    Some(AppMsg::None)
                }
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('a'),
                modifiers: KeyModifiers::NONE,
            }) => Some(AppMsg::InstallAllMissing),
            Event::Keyboard(KeyEvent {
                code: Key::Char('r'),
                modifiers: KeyModifiers::NONE,
            }) => Some(AppMsg::RefreshStatuses),
            Event::Keyboard(KeyEvent {
                code: Key::Char('t'),
                modifiers: KeyModifiers::NONE,
            }) => Some(AppMsg::ToggleTaskView),
            Event::Keyboard(KeyEvent {
                code: Key::Char('q'),
                modifiers: KeyModifiers::NONE,
            }) => Some(AppMsg::Quit),
            Event::Keyboard(KeyEvent {
                code: Key::Esc | Key::Char('m'),
                modifiers: KeyModifiers::NONE,
            }) => Some(AppMsg::GoToMenu),
            _ => None,
        }
    }
}
