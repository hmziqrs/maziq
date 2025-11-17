use std::collections::HashMap;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
};
use tui_realm_stdlib::{Table as StdlibTable, Paragraph as StdlibParagraph};
use tuirealm::{
    Component, Event, MockComponent, State, StateValue,
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent, KeyModifiers},
    props::{Alignment, BorderType, Borders as PropBorders, Color as PropColor, TableBuilder, TextSpan},
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

    fn build_table(&self) -> StdlibTable {
        let mut table_builder = TableBuilder::default();

        for handle in &self.handles {
            let state = self.statuses
                .get(&handle.id)
                .cloned()
                .unwrap_or(StatusState::NotInstalled);

            let (status_icon, status_text, status_color) = match state {
                StatusState::Installed { ref version } => {
                    let text = version.as_deref().unwrap_or("Installed");
                    ("✅", text, PropColor::Green)
                }
                StatusState::NotInstalled => ("❌", "Missing", PropColor::Red),
                StatusState::ManualCheck(ref note) => ("⚠", note.as_str(), PropColor::Yellow),
                StatusState::Unknown(ref note) => ("??", note.as_str(), PropColor::Magenta),
            };

            table_builder
                .add_col(TextSpan::from(handle.category).fg(PropColor::Yellow).bold())
                .add_col(TextSpan::from(handle.id.name()).fg(PropColor::White).bold())
                .add_col(TextSpan::from(status_icon).fg(status_color))
                .add_col(TextSpan::from(status_text).fg(status_color))
                .add_col(TextSpan::from(handle.id.summary()).fg(PropColor::DarkGray))
                .add_row();
        }

        StdlibTable::default()
            .borders(
                PropBorders::default()
                    .modifiers(BorderType::Rounded)
                    .color(PropColor::White),
            )
            .title("macOS Provisioning Catalog", Alignment::Left)
            .scroll(true)
            .rewind(true)
            .highlighted_color(PropColor::LightBlue)
            .highlighted_str(">> ")
            .headers(&["Category", "Name", "", "Status", "Description"])
            .column_spacing(2)
            .widths(&[15, 20, 3, 15, 50])
            .table(table_builder.build())
            .selected_line(self.selected)
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

        // Render software table using stdlib Table
        let mut table = self.build_table();
        table.view(frame, chunks[0]);

        // Render lower section
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
        if let Event::Keyboard(KeyEvent { code, modifiers }) = ev {
            match code {
                Key::Down => {
                    if !self.handles.is_empty() {
                        self.selected = (self.selected + 1) % self.handles.len();
                    }
                    return Some(AppMsg::None);
                }
                Key::Up => {
                    if !self.handles.is_empty() {
                        self.selected = if self.selected == 0 {
                            self.handles.len() - 1
                        } else {
                            self.selected - 1
                        };
                    }
                    return Some(AppMsg::None);
                }
                Key::Char('j') if modifiers == KeyModifiers::NONE => {
                    if !self.handles.is_empty() {
                        self.selected = (self.selected + 1) % self.handles.len();
                    }
                    return Some(AppMsg::None);
                }
                Key::Char('k') if modifiers == KeyModifiers::NONE => {
                    if !self.handles.is_empty() {
                        self.selected = if self.selected == 0 {
                            self.handles.len() - 1
                        } else {
                            self.selected - 1
                        };
                    }
                    return Some(AppMsg::None);
                }
                Key::Enter if modifiers == KeyModifiers::NONE => {
                    return if let Some(id) = self.selected_software() {
                        Some(AppMsg::InstallSoftware(id))
                    } else {
                        Some(AppMsg::None)
                    };
                }
                Key::Char('u') if modifiers == KeyModifiers::NONE => {
                    return if let Some(id) = self.selected_software() {
                        Some(AppMsg::UpdateSoftware(id))
                    } else {
                        Some(AppMsg::None)
                    };
                }
                Key::Char('x') if modifiers == KeyModifiers::NONE => {
                    return if let Some(id) = self.selected_software() {
                        Some(AppMsg::UninstallSoftware(id))
                    } else {
                        Some(AppMsg::None)
                    };
                }
                Key::Char('a') if modifiers == KeyModifiers::NONE => {
                    return Some(AppMsg::InstallAllMissing);
                }
                Key::Char('r') if modifiers == KeyModifiers::NONE => {
                    return Some(AppMsg::RefreshStatuses);
                }
                Key::Char('t') if modifiers == KeyModifiers::NONE => {
                    return Some(AppMsg::ToggleTaskView);
                }
                Key::Char('q') if modifiers == KeyModifiers::NONE => {
                    return Some(AppMsg::Quit);
                }
                Key::Esc | Key::Char('m') if modifiers == KeyModifiers::NONE => {
                    return Some(AppMsg::GoToMenu);
                }
                _ => {}
            }
        }
        None
    }
}
