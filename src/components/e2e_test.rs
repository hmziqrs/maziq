use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};
use tuirealm::{
    Component, Event, MockComponent, State, StateValue,
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent, KeyModifiers},
};

use crate::{catalog::SoftwareId, messages::AppMsg};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum E2ETab {
    Install,
    Update,
    Remove,
    Execute,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum E2EStepStatus {
    Pending,
    Running,
    Done,
    Skipped,
}

pub struct E2ETestComponent {
    software: Option<SoftwareId>,
    tab: E2ETab,
    install_enabled: bool,
    update_enabled: bool,
    remove_enabled: bool,
    install_status: E2EStepStatus,
    update_status: E2EStepStatus,
    remove_status: E2EStepStatus,
    executing: bool,
    task_lines: Vec<String>,
}

impl Default for E2ETestComponent {
    fn default() -> Self {
        Self {
            software: None,
            tab: E2ETab::Install,
            install_enabled: true,
            update_enabled: true,
            remove_enabled: true,
            install_status: E2EStepStatus::Pending,
            update_status: E2EStepStatus::Pending,
            remove_status: E2EStepStatus::Pending,
            executing: false,
            task_lines: Vec::new(),
        }
    }
}

impl E2ETestComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.software = None;
        self.tab = E2ETab::Install;
        self.install_enabled = true;
        self.update_enabled = true;
        self.remove_enabled = true;
        self.install_status = E2EStepStatus::Pending;
        self.update_status = E2EStepStatus::Pending;
        self.remove_status = E2EStepStatus::Pending;
        self.executing = false;
        self.task_lines.clear();
    }

    pub fn set_software(&mut self, software: SoftwareId) {
        self.software = Some(software);
    }

    pub fn set_task_lines(&mut self, lines: Vec<String>) {
        self.task_lines = lines;
    }

    pub fn set_executing(&mut self, executing: bool) {
        self.executing = executing;
    }

    pub fn is_executing(&self) -> bool {
        self.executing
    }

    pub fn toggle_current_step(&mut self) {
        if self.executing {
            return;
        }
        match self.tab {
            E2ETab::Install => self.install_enabled = !self.install_enabled,
            E2ETab::Update => self.update_enabled = !self.update_enabled,
            E2ETab::Remove => self.remove_enabled = !self.remove_enabled,
            E2ETab::Execute => {}
        }
    }

    pub fn can_execute(&self) -> bool {
        self.software.is_some() && !self.executing && self.tab == E2ETab::Execute
    }
}

impl MockComponent for E2ETestComponent {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(8),
                Constraint::Length(3),
            ])
            .split(area);

        // Title
        let title = Paragraph::new("Brew End-to-End Test")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, main_chunks[0]);

        // Software selection or tabs
        if self.software.is_none() {
            let selection_text = "Select software: 1=Neovim, 2=btop (Esc/m to return to menu)";
            let selection = Paragraph::new(selection_text)
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().title("Selection").borders(Borders::ALL));
            frame.render_widget(selection, main_chunks[1]);

            let info = Paragraph::new("This test will run install → update → remove for the selected software.\nUse this to verify the complete lifecycle of package management.")
                .block(Block::default().title("Info").borders(Borders::ALL));
            frame.render_widget(info, main_chunks[2]);
        } else {
            // Draw horizontal tabs
            let tabs_text = format!(
                "{}  {}  {}  {}",
                if self.tab == E2ETab::Install { "[ Install ]" } else { "  Install  " },
                if self.tab == E2ETab::Update { "[ Update ]" } else { "  Update  " },
                if self.tab == E2ETab::Remove { "[ Remove ]" } else { "  Remove  " },
                if self.tab == E2ETab::Execute { "[ Execute ]" } else { "  Execute  " },
            );
            let tabs = Paragraph::new(tabs_text)
                .style(Style::default().fg(Color::Cyan))
                .block(Block::default().title("Steps").borders(Borders::ALL));
            frame.render_widget(tabs, main_chunks[1]);

            // Split content area if executing to show logs
            let content_chunks = if self.executing {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(6), Constraint::Min(4)])
                    .split(main_chunks[2])
            } else {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(8)])
                    .split(main_chunks[2])
            };

            // Tab content
            let content = match self.tab {
                E2ETab::Install => {
                    let checkbox = if self.install_enabled { "[✓]" } else { "[ ]" };
                    let status_icon = match self.install_status {
                        E2EStepStatus::Pending => "⏸",
                        E2EStepStatus::Running => "▶",
                        E2EStepStatus::Done => "✓",
                        E2EStepStatus::Skipped => "⊘",
                    };
                    format!(
                        "{} Install Step\n\n{} Enabled\n\nThis step will install the selected software.\n\nStatus: {}\n\nPress Space to toggle this step.",
                        status_icon,
                        checkbox,
                        match self.install_status {
                            E2EStepStatus::Pending => "Pending",
                            E2EStepStatus::Running => "Running...",
                            E2EStepStatus::Done => "Complete",
                            E2EStepStatus::Skipped => "Skipped",
                        }
                    )
                }
                E2ETab::Update => {
                    let checkbox = if self.update_enabled { "[✓]" } else { "[ ]" };
                    let status_icon = match self.update_status {
                        E2EStepStatus::Pending => "⏸",
                        E2EStepStatus::Running => "▶",
                        E2EStepStatus::Done => "✓",
                        E2EStepStatus::Skipped => "⊘",
                    };
                    format!(
                        "{} Update Step\n\n{} Enabled\n\nThis step will update the software to the latest version.\n\nStatus: {}\n\nPress Space to toggle this step.",
                        status_icon,
                        checkbox,
                        match self.update_status {
                            E2EStepStatus::Pending => "Pending",
                            E2EStepStatus::Running => "Running...",
                            E2EStepStatus::Done => "Complete",
                            E2EStepStatus::Skipped => "Skipped",
                        }
                    )
                }
                E2ETab::Remove => {
                    let checkbox = if self.remove_enabled { "[✓]" } else { "[ ]" };
                    let status_icon = match self.remove_status {
                        E2EStepStatus::Pending => "⏸",
                        E2EStepStatus::Running => "▶",
                        E2EStepStatus::Done => "✓",
                        E2EStepStatus::Skipped => "⊘",
                    };
                    format!(
                        "{} Remove Step\n\n{} Enabled\n\nThis step will uninstall the software.\n\nStatus: {}\n\nPress Space to toggle this step.",
                        status_icon,
                        checkbox,
                        match self.remove_status {
                            E2EStepStatus::Pending => "Pending",
                            E2EStepStatus::Running => "Running...",
                            E2EStepStatus::Done => "Complete",
                            E2EStepStatus::Skipped => "Skipped",
                        }
                    )
                }
                E2ETab::Execute => {
                    if self.executing {
                        "Execution in progress...\n\nCheck the logs below for real-time progress.".to_string()
                    } else {
                        let mut steps = Vec::new();
                        if self.install_enabled {
                            steps.push("1. Install");
                        }
                        if self.update_enabled {
                            steps.push("2. Update");
                        }
                        if self.remove_enabled {
                            steps.push("3. Remove");
                        }

                        if steps.is_empty() {
                            "No steps selected!\n\nPlease enable at least one step before executing.\n\nNavigate to Install, Update, or Remove tabs and press Space to enable them.".to_string()
                        } else {
                            format!(
                                "Ready to Execute\n\nPress Enter to start the E2E test.\n\nThe test will run:\n{}\n\nLogs will appear below in real-time.",
                                steps.join("\n")
                            )
                        }
                    }
                }
            };

            let content_widget = Paragraph::new(content)
                .block(Block::default().title("Step Details").borders(Borders::ALL));
            frame.render_widget(content_widget, content_chunks[0]);

            // Render logs if executing
            if self.executing {
                let log_text = if self.task_lines.is_empty() {
                    "No logs yet...".to_string()
                } else {
                    self.task_lines.join("\n")
                };
                let log_widget = Paragraph::new(log_text)
                    .block(Block::default().title("Live Logs").borders(Borders::ALL));
                frame.render_widget(log_widget, content_chunks[1]);
            }
        }

        // Instructions
        let instructions_text = if self.software.is_none() {
            "1/2 select software • Esc/m menu • q quit".to_string()
        } else if self.executing {
            "t toggle task view • Esc/m menu • q quit".to_string()
        } else {
            "←/→ or Tab navigate tabs • Space toggle step • Enter execute • Esc/m menu • q quit".to_string()
        };
        let instructions = Paragraph::new(instructions_text)
            .block(Block::default().title("Controls").borders(Borders::ALL));
        frame.render_widget(instructions, main_chunks[3]);
    }

    fn query(&self, attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
        match attr {
            tuirealm::Attribute::Value => {
                Some(tuirealm::AttrValue::Number(self.tab as isize))
            }
            _ => None,
        }
    }

    fn attr(&mut self, _attr: tuirealm::Attribute, _value: tuirealm::AttrValue) {
        // No attributes to set
    }

    fn state(&self) -> State {
        State::One(StateValue::Usize(self.tab as usize))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(tuirealm::command::Direction::Right) => {
                if self.software.is_some() && !self.executing {
                    self.tab = match self.tab {
                        E2ETab::Install => E2ETab::Update,
                        E2ETab::Update => E2ETab::Remove,
                        E2ETab::Remove => E2ETab::Execute,
                        E2ETab::Execute => E2ETab::Install,
                    };
                    CmdResult::Changed(State::One(StateValue::Usize(self.tab as usize)))
                } else {
                    CmdResult::None
                }
            }
            Cmd::Move(tuirealm::command::Direction::Left) => {
                if self.software.is_some() && !self.executing {
                    self.tab = match self.tab {
                        E2ETab::Install => E2ETab::Execute,
                        E2ETab::Update => E2ETab::Install,
                        E2ETab::Remove => E2ETab::Update,
                        E2ETab::Execute => E2ETab::Remove,
                    };
                    CmdResult::Changed(State::One(StateValue::Usize(self.tab as usize)))
                } else {
                    CmdResult::None
                }
            }
            _ => CmdResult::None,
        }
    }
}

impl Component<AppMsg, tuirealm::NoUserEvent> for E2ETestComponent {
    fn on(&mut self, ev: Event<tuirealm::NoUserEvent>) -> Option<AppMsg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Right | Key::Tab,
                modifiers: KeyModifiers::NONE,
            }) => {
                if self.software.is_some() && !self.executing {
                    self.perform(Cmd::Move(tuirealm::command::Direction::Right));
                }
                Some(AppMsg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left,
                modifiers: KeyModifiers::NONE,
            }) => {
                if self.software.is_some() && !self.executing {
                    self.perform(Cmd::Move(tuirealm::command::Direction::Left));
                }
                Some(AppMsg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(' '),
                modifiers: KeyModifiers::NONE,
            }) => {
                if self.software.is_some() && !self.executing {
                    match self.tab {
                        E2ETab::Install => Some(AppMsg::ToggleE2EInstall),
                        E2ETab::Update => Some(AppMsg::ToggleE2EUpdate),
                        E2ETab::Remove => Some(AppMsg::ToggleE2ERemove),
                        E2ETab::Execute => Some(AppMsg::None),
                    }
                } else {
                    Some(AppMsg::None)
                }
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                modifiers: KeyModifiers::NONE,
            }) => {
                if self.can_execute() {
                    Some(AppMsg::ExecuteE2E)
                } else {
                    Some(AppMsg::None)
                }
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('1'),
                modifiers: KeyModifiers::NONE,
            }) => {
                if self.software.is_none() {
                    Some(AppMsg::SelectE2ESoftware(SoftwareId::Neovim))
                } else {
                    Some(AppMsg::None)
                }
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('2'),
                modifiers: KeyModifiers::NONE,
            }) => {
                if self.software.is_none() {
                    Some(AppMsg::SelectE2ESoftware(SoftwareId::Btop))
                } else {
                    Some(AppMsg::None)
                }
            }
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
