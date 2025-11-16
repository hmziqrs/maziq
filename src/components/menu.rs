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

use crate::messages::AppMsg;

#[derive(Clone, Copy, Debug)]
pub struct MenuEntry {
    pub label: &'static str,
    pub description: &'static str,
    pub index: usize,
}

const MENU_ENTRIES: &[MenuEntry] = &[
    MenuEntry {
        label: "Onboard (Fresh install)",
        description: "Install the default hmziq template in dependency order.",
        index: 0,
    },
    MenuEntry {
        label: "Update template",
        description: "Update everything defined in the hmziq template.",
        index: 1,
    },
    MenuEntry {
        label: "Configurator (experimental)",
        description: "Preview Git/SSH/GPG config helpers.",
        index: 2,
    },
    MenuEntry {
        label: "Software catalog",
        description: "Browse and manage individual software entries.",
        index: 3,
    },
    MenuEntry {
        label: "Versions",
        description: "Refresh and display detected versions/statuses.",
        index: 4,
    },
    MenuEntry {
        label: "Brew End to End Test",
        description: "Test install, update, and remove flows for neovim or btop.",
        index: 5,
    },
];

pub struct MenuComponent {
    selected: usize,
    message: String,
    log_lines: Vec<String>,
    show_tasks: bool,
    task_lines: Vec<String>,
}

impl Default for MenuComponent {
    fn default() -> Self {
        Self {
            selected: 0,
            message: "Select a workflow from the menu (Onboard/Update/Config/Catalog).".into(),
            log_lines: Vec::new(),
            show_tasks: true,
            task_lines: Vec::new(),
        }
    }
}

impl MenuComponent {
    pub fn new() -> Self {
        Self::default()
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
}

impl MockComponent for MenuComponent {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(72), Constraint::Percentage(28)])
            .split(area);

        // Render menu list
        let items: Vec<ListItem> = MENU_ENTRIES
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

        let mut list_state = ratatui::widgets::ListState::default();
        list_state.select(Some(self.selected));

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

        frame.render_stateful_widget(list, chunks[0], &mut list_state);

        // Render lower section with status, log, and controls
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
        let dry_run_notice = if crate::options::global_dry_run() {
            " (dry-run: actions previewed only)"
        } else {
            ""
        };
        let instructions = Paragraph::new(format!(
            "Enter or 1-6 selects • ↑/↓ or j/k navigate • r refresh statuses • t toggle log • Esc/m stay here • q quit{}",
            dry_run_notice
        ))
        .block(
            Block::default()
                .title("Menu controls")
                .borders(Borders::ALL),
        );
        frame.render_widget(instructions, lower[2]);
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
                    self.selected = n.max(0).min((MENU_ENTRIES.len() - 1) as isize) as usize;
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
                self.selected = (self.selected + 1) % MENU_ENTRIES.len();
                CmdResult::Changed(State::One(StateValue::Usize(self.selected)))
            }
            Cmd::Move(tuirealm::command::Direction::Up) => {
                self.selected = if self.selected == 0 {
                    MENU_ENTRIES.len() - 1
                } else {
                    self.selected - 1
                };
                CmdResult::Changed(State::One(StateValue::Usize(self.selected)))
            }
            Cmd::Submit => {
                CmdResult::Submit(State::One(StateValue::Usize(self.selected)))
            }
            _ => CmdResult::None,
        }
    }
}

impl Component<AppMsg, tuirealm::NoUserEvent> for MenuComponent {
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
                code: Key::Enter | Key::Char(' '),
                modifiers: KeyModifiers::NONE,
            }) => Some(AppMsg::ActivateMenuItem(self.selected)),
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch @ '1'..='6'),
                modifiers: KeyModifiers::NONE,
            }) => {
                let index = (ch as u8 - b'1') as usize;
                if index < MENU_ENTRIES.len() {
                    Some(AppMsg::ActivateMenuItem(index))
                } else {
                    Some(AppMsg::None)
                }
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('t'),
                modifiers: KeyModifiers::NONE,
            }) => Some(AppMsg::ToggleTaskView),
            Event::Keyboard(KeyEvent {
                code: Key::Char('r'),
                modifiers: KeyModifiers::NONE,
            }) => Some(AppMsg::RefreshStatuses),
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
