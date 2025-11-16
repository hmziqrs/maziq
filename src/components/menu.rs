use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
};
use tui_realm_stdlib::{List as StdlibList, Paragraph as StdlibParagraph};
use tuirealm::{
    Component, Event, MockComponent, State, StateValue,
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent, KeyModifiers},
    props::{Alignment, BorderType, Borders as PropBorders, Color as PropColor, TableBuilder, TextSpan},
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
    list: StdlibList,
}

impl Default for MenuComponent {
    fn default() -> Self {
        // Build menu rows
        let mut table_builder = TableBuilder::default();
        for entry in MENU_ENTRIES {
            table_builder
                .add_col(TextSpan::from(entry.label).fg(PropColor::Cyan).bold())
                .add_col(TextSpan::from(" — "))
                .add_col(TextSpan::from(entry.description).fg(PropColor::DarkGray))
                .add_row();
        }

        let list = StdlibList::default()
            .rows(table_builder.build())
            .selected_line(0)
            .scroll(true)
            .rewind(true)
            .highlighted_str(">> ")
            .highlighted_color(PropColor::LightCyan)
            .borders(
                PropBorders::default()
                    .modifiers(BorderType::Rounded)
                    .color(PropColor::White),
            )
            .title("MazIQ workflows", Alignment::Left);

        Self {
            selected: 0,
            message: "Select a workflow from the menu (Onboard/Update/Config/Catalog).".into(),
            log_lines: Vec::new(),
            show_tasks: true,
            task_lines: Vec::new(),
            list,
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

        // Render menu list using stdlib List
        self.list.attr(
            tuirealm::Attribute::Value,
            tuirealm::AttrValue::Number(self.selected as isize),
        );
        self.list.view(frame, chunks[0]);

        // Render lower section with status, log, and controls
        let lower = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(4),
                Constraint::Length(4),
            ])
            .split(chunks[1]);

        // Status message using stdlib Paragraph
        let status_text = vec![TextSpan::from(&self.message)];
        let mut status = StdlibParagraph::default()
            .foreground(PropColor::White)
            .borders(
                PropBorders::default()
                    .modifiers(BorderType::Rounded)
                    .color(PropColor::White),
            )
            .title("Status", Alignment::Left)
            .text(&status_text);
        status.view(frame, lower[0]);

        // Log panel using stdlib Paragraph
        let log_text: Vec<TextSpan> = if self.show_tasks {
            if self.task_lines.is_empty() {
                vec![TextSpan::from("No tasks queued.")]
            } else {
                self.task_lines.iter().map(|line| TextSpan::from(line)).collect()
            }
        } else if self.log_lines.is_empty() {
            vec![TextSpan::from("No actions executed yet.")]
        } else {
            self.log_lines.iter().map(|line| TextSpan::from(line)).collect()
        };

        let log_title = if self.show_tasks { "Tasks" } else { "Action log" };
        let mut log = StdlibParagraph::default()
            .foreground(PropColor::White)
            .borders(
                PropBorders::default()
                    .modifiers(BorderType::Rounded)
                    .color(PropColor::White),
            )
            .title(log_title, Alignment::Left)
            .text(&log_text);
        log.view(frame, lower[1]);

        // Controls - still using ratatui Paragraph for instructions
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
