use std::collections::HashMap;

use ratatui::widgets::ListState;

use crate::{
    catalog::{self, SoftwareHandle, SoftwareId},
    configurator::{self, ApplyOptions as ConfigApplyOptions},
    manager::{ActionKind, ExecutionEvent, SoftwareManager, StatusState},
    templates,
};

const MAX_LOG_ENTRIES: usize = 6;
const DEFAULT_TEMPLATE: &str = "hmziq";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Screen {
    Menu,
    Software,
}

#[derive(Clone, Copy, Debug)]
pub struct MenuEntry {
    pub label: &'static str,
    pub description: &'static str,
    action: MenuAction,
}

#[derive(Clone, Copy, Debug)]
enum MenuAction {
    OnboardFresh,
    OnboardUpdate,
    Config,
    SoftwareCatalog,
    Versions,
}

const MENU_ENTRIES: &[MenuEntry] = &[
    MenuEntry {
        label: "Onboard (Fresh install)",
        description: "Install the default hmziq template in dependency order.",
        action: MenuAction::OnboardFresh,
    },
    MenuEntry {
        label: "Update template",
        description: "Update everything defined in the hmziq template.",
        action: MenuAction::OnboardUpdate,
    },
    MenuEntry {
        label: "Configurator (experimental)",
        description: "Preview Git/SSH/GPG config helpers.",
        action: MenuAction::Config,
    },
    MenuEntry {
        label: "Software catalog",
        description: "Browse and manage individual software entries.",
        action: MenuAction::SoftwareCatalog,
    },
    MenuEntry {
        label: "Versions",
        description: "Refresh and display detected versions/statuses.",
        action: MenuAction::Versions,
    },
];

pub struct App {
    handles: Vec<SoftwareHandle>,
    state: ListState,
    menu_state: ListState,
    statuses: HashMap<SoftwareId, StatusState>,
    manager: SoftwareManager,
    log: Vec<String>,
    screen: Screen,
    pub message: String,
    pub quit: bool,
}

impl App {
    pub fn new() -> Self {
        let handles = catalog::flattened_handles();
        let mut state = ListState::default();
        if !handles.is_empty() {
            state.select(Some(0));
        }
        let mut menu_state = ListState::default();
        if !MENU_ENTRIES.is_empty() {
            menu_state.select(Some(0));
        }

        let mut app = Self {
            handles,
            state,
            menu_state,
            statuses: HashMap::new(),
            manager: SoftwareManager::new(),
            log: Vec::new(),
            screen: Screen::Menu,
            message: "Select a workflow from the menu (Onboard/Update/Config/Catalog).".into(),
            quit: false,
        };
        app.refresh_statuses();
        if crate::options::global_dry_run() {
            app.push_log_line("Global dry-run mode enabled.".into());
            app.message = "Dry-run: browse or preview workflows; no commands will execute.".into();
        }
        app
    }

    pub fn handles(&self) -> &[SoftwareHandle] {
        &self.handles
    }

    pub fn menu_items(&self) -> &[MenuEntry] {
        MENU_ENTRIES
    }

    pub fn list_state_mut(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn menu_state_mut(&mut self) -> &mut ListState {
        &mut self.menu_state
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn log(&self) -> &[String] {
        &self.log
    }

    pub fn status_for(&self, id: SoftwareId) -> StatusState {
        self.statuses
            .get(&id)
            .cloned()
            .unwrap_or(StatusState::NotInstalled)
    }

    pub fn screen(&self) -> Screen {
        self.screen
    }

    pub fn enter_menu(&mut self) {
        self.screen = Screen::Menu;
        self.message = "Select a workflow from the menu.".into();
    }

    pub fn next(&mut self) {
        match self.screen {
            Screen::Menu => {
                if MENU_ENTRIES.is_empty() {
                    return;
                }
                let current = self.menu_state.selected().unwrap_or(0);
                let next = (current + 1) % MENU_ENTRIES.len();
                self.menu_state.select(Some(next));
            }
            Screen::Software => {
                if self.handles.is_empty() {
                    return;
                }
                let current = self.state.selected().unwrap_or(0);
                let next = (current + 1) % self.handles.len();
                self.state.select(Some(next));
            }
        }
    }

    pub fn previous(&mut self) {
        match self.screen {
            Screen::Menu => {
                if MENU_ENTRIES.is_empty() {
                    return;
                }
                let current = self.menu_state.selected().unwrap_or(0);
                let prev = if current == 0 {
                    MENU_ENTRIES.len() - 1
                } else {
                    current - 1
                };
                self.menu_state.select(Some(prev));
            }
            Screen::Software => {
                if self.handles.is_empty() {
                    return;
                }
                let current = self.state.selected().unwrap_or(0);
                let prev = if current == 0 {
                    self.handles.len() - 1
                } else {
                    current - 1
                };
                self.state.select(Some(prev));
            }
        }
    }

    pub fn activate_menu(&mut self) {
        if let Some(index) = self.menu_state.selected() {
            self.activate_menu_index(index);
        }
    }

    pub fn activate_menu_index(&mut self, index: usize) {
        if let Some(entry) = MENU_ENTRIES.get(index) {
            self.message = format!("Selected {}", entry.label);
            self.push_log_line(format!("menu -> {}", entry.label));
            match entry.action {
                MenuAction::OnboardFresh => self.run_template_flow(ActionKind::Install),
                MenuAction::OnboardUpdate => self.run_template_flow(ActionKind::Update),
                MenuAction::Config => self.show_config_preview(),
                MenuAction::SoftwareCatalog => {
                    self.screen = Screen::Software;
                    self.message =
                        "Browse the catalog. Use Enter/u/x to install/update/uninstall.".into();
                }
                MenuAction::Versions => {
                    self.refresh_statuses();
                    self.message = "Versions refreshed via status probes.".into();
                }
            }
        }
    }

    pub fn install_selected(&mut self) {
        self.run_on_selected(ActionKind::Install);
    }

    pub fn update_selected(&mut self) {
        self.run_on_selected(ActionKind::Update);
    }

    pub fn uninstall_selected(&mut self) {
        self.run_on_selected(ActionKind::Uninstall);
    }

    pub fn install_all_missing(&mut self) {
        let targets: Vec<_> = self
            .handles
            .iter()
            .filter_map(|handle| match self.status_for(handle.id) {
                StatusState::NotInstalled
                | StatusState::ManualCheck(_)
                | StatusState::Unknown(_) => Some(handle.id),
                _ => None,
            })
            .collect();
        if targets.is_empty() {
            self.message = "All software appears installed.".into();
            return;
        }
        self.message = format!("Installing {} items...", targets.len());
        self.run_action(ActionKind::Install, targets);
    }

    pub fn refresh_statuses_with_feedback(&mut self) {
        self.refresh_statuses();
        self.message = "Statuses refreshed.".into();
    }

    pub fn refresh_statuses(&mut self) {
        for handle in &self.handles {
            let status = self
                .manager
                .status(handle.id)
                .map(|report| report.state)
                .unwrap_or_else(|err| StatusState::Unknown(err.to_string()));
            self.statuses.insert(handle.id, status);
        }
    }

    fn run_on_selected(&mut self, action: ActionKind) {
        if self.screen != Screen::Software {
            self.message = "Open the software catalog to manage individual entries.".into();
            return;
        }
        if let Some(index) = self.state.selected() {
            let id = self.handles[index].id;
            self.run_action(action, vec![id]);
        }
    }

    fn run_action(&mut self, action: ActionKind, ids: Vec<SoftwareId>) {
        if ids.is_empty() {
            return;
        }
        let label = if ids.len() == 1 {
            catalog::entry(ids[0]).display_name.to_string()
        } else {
            format!("{} selections", ids.len())
        };
        let result = match action {
            ActionKind::Install => self.manager.install_many(&ids),
            ActionKind::Update => self.manager.update_many(&ids),
            ActionKind::Uninstall => self.manager.uninstall_many(&ids),
            ActionKind::Test => Ok(Vec::new()),
        };
        match result {
            Ok(events) => {
                if events.is_empty() {
                    self.message = format!("No actions performed for {label}.");
                } else {
                    self.message = format!(
                        "{} complete for {} ({} event{})",
                        action.label(),
                        label,
                        events.len(),
                        if events.len() == 1 { "" } else { "s" }
                    );
                    self.append_log(&events);
                }
                self.refresh_statuses();
            }
            Err(err) => {
                self.message = format!("{} failed for {}: {}", action.label(), label, err);
                self.push_log_line(format!("error: {}", err));
            }
        }
    }

    fn run_template_flow(&mut self, action: ActionKind) {
        match templates::load_named(DEFAULT_TEMPLATE) {
            Ok(template) => {
                self.message = format!(
                    "{} `{}` template...",
                    match action {
                        ActionKind::Install => "Installing",
                        ActionKind::Update => "Updating",
                        ActionKind::Uninstall => "Uninstalling",
                        ActionKind::Test => "Testing",
                    },
                    template.name
                );
                match self.manager.plan(&template.software, action) {
                    Ok(plan) => {
                        self.push_log_line(format!(
                            "{} plan ({} steps)",
                            action.label(),
                            plan.len()
                        ));
                        for (idx, id) in plan.iter().enumerate() {
                            self.push_log_line(format!("{:>2}. {}", idx + 1, id.name()));
                        }
                        let result = match action {
                            ActionKind::Install => self.manager.install_many(&template.software),
                            ActionKind::Update => self.manager.update_many(&template.software),
                            ActionKind::Uninstall => {
                                self.manager.uninstall_many(&template.software)
                            }
                            ActionKind::Test => Ok(Vec::new()),
                        };
                        match result {
                            Ok(events) => {
                                self.append_log(&events);
                                self.message = format!(
                                    "{} template `{}` complete.",
                                    action.label(),
                                    template.name
                                );
                                self.refresh_statuses();
                            }
                            Err(err) => {
                                self.message = format!(
                                    "{} template `{}` failed: {}",
                                    action.label(),
                                    template.name,
                                    err
                                );
                            }
                        }
                    }
                    Err(err) => {
                        self.message =
                            format!("Failed to plan template `{}`: {}", template.name, err);
                    }
                }
            }
            Err(err) => {
                self.message = format!("Unable to load `{DEFAULT_TEMPLATE}` template: {}", err);
            }
        }
    }

    fn show_config_preview(&mut self) {
        match configurator::apply_profile(
            "hmziq-default",
            ConfigApplyOptions {
                dry_run: true,
                experimental: true,
            },
        ) {
            Ok(result) => {
                self.message = "Configurator preview (hmziq-default). Use CLI with --experimental-config to apply.".into();
                for action in result.actions {
                    self.push_log_line(format!("config preview -> {action}"));
                }
            }
            Err(err) => {
                self.message = format!("Configurator unavailable: {}", err);
            }
        }
    }

    fn append_log(&mut self, events: &[ExecutionEvent]) {
        for event in events {
            self.push_log_line(event.summary());
        }
    }

    fn push_log_line(&mut self, line: String) {
        self.log.push(line);
        if self.log.len() > MAX_LOG_ENTRIES {
            let excess = self.log.len() - MAX_LOG_ENTRIES;
            self.log.drain(0..excess);
        }
    }
}
