use std::{
    collections::HashMap,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use ratatui::widgets::ListState;

use crate::{
    catalog::{self, SoftwareHandle, SoftwareId},
    configurator::{self, ApplyOptions as ConfigApplyOptions},
    manager::{ActionKind, ExecutionEvent, SoftwareManager, StatusReport, StatusState},
    templates::{self, Template},
};

const MAX_LOG_ENTRIES: usize = 6;
const DEFAULT_TEMPLATE: &str = "hmziq";
const MAX_TASK_LOG_LINES: usize = 5;
const MAX_TASKS: usize = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Screen {
    Menu,
    Software,
    E2ETest,
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
    E2ETest,
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
    MenuEntry {
        label: "Brew End to End Test",
        description: "Test install, update, and remove flows for neovim or btop.",
        action: MenuAction::E2ETest,
    },
];

#[derive(Clone)]
struct TaskRequest {
    id: u64,
    label: String,
    action: TaskAction,
}

#[derive(Clone)]
enum TaskAction {
    TemplateFlow {
        action: ActionKind,
        template: Template,
        force: bool,
        dry_run: bool,
    },
    Versions,
    E2EFlow {
        software: SoftwareId,
        install_enabled: bool,
        update_enabled: bool,
        remove_enabled: bool,
    },
}

struct TaskEvent {
    id: u64,
    label: String,
    messages: Vec<String>,
    statuses: Option<Vec<StatusReport>>,
}

#[derive(Clone, Debug)]
pub struct TaskLog {
    pub id: u64,
    pub label: String,
    pub lines: Vec<String>,
}

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

pub struct App {
    handles: Vec<SoftwareHandle>,
    state: ListState,
    menu_state: ListState,
    statuses: HashMap<SoftwareId, StatusState>,
    manager: SoftwareManager,
    log: Vec<String>,
    screen: Screen,
    task_sender: Sender<TaskRequest>,
    event_receiver: Receiver<TaskEvent>,
    next_task_id: u64,
    menu_enabled: bool,
    show_tasks: bool,
    task_logs: Vec<TaskLog>,
    initial_task: Option<u64>,
    pub message: String,
    pub quit: bool,
    e2e_software: Option<SoftwareId>,
    e2e_tab: E2ETab,
    e2e_install_enabled: bool,
    e2e_update_enabled: bool,
    e2e_remove_enabled: bool,
    e2e_install_status: E2EStepStatus,
    e2e_update_status: E2EStepStatus,
    e2e_remove_status: E2EStepStatus,
    e2e_executing: bool,
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

        let (task_tx, task_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();
        spawn_worker(task_rx, event_tx);

        let mut app = Self {
            handles,
            state,
            menu_state,
            statuses: HashMap::new(),
            manager: SoftwareManager::new(),
            log: Vec::new(),
            screen: Screen::Menu,
            task_sender: task_tx,
            event_receiver: event_rx,
            next_task_id: 1,
            menu_enabled: false,
            show_tasks: true,
            task_logs: Vec::new(),
            initial_task: None,
            message: "Select a workflow from the menu (Onboard/Update/Config/Catalog).".into(),
            quit: false,
            e2e_software: None,
            e2e_tab: E2ETab::Install,
            e2e_install_enabled: true,
            e2e_update_enabled: true,
            e2e_remove_enabled: true,
            e2e_install_status: E2EStepStatus::Pending,
            e2e_update_status: E2EStepStatus::Pending,
            e2e_remove_status: E2EStepStatus::Pending,
            e2e_executing: false,
        };
        if crate::options::global_dry_run() {
            app.push_log_line("Global dry-run mode enabled.".into());
            app.message = "Dry-run: browse or preview workflows; no commands will execute.".into();
        }
        app.queue_initial_status_refresh();
        app
    }

    fn queue_initial_status_refresh(&mut self) {
        let request = TaskRequest {
            id: self.next_task_id,
            label: "Initial status refresh".into(),
            action: TaskAction::Versions,
        };
        self.next_task_id += 1;
        let queued_id = request.id;
        let queued_label = request.label.clone();
        self.initial_task = Some(queued_id);
        if self.task_sender.send(request).is_ok() {
            self.message = "Loading statuses...".into();
            self.add_task_log(queued_id, queued_label, "queued".into());
            self.show_tasks = true;
        } else {
            self.message = "Failed to queue status refresh.".into();
        }
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

    pub fn show_tasks(&self) -> bool {
        self.show_tasks
    }

    pub fn task_logs(&self) -> &[TaskLog] {
        &self.task_logs
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
        if self.menu_enabled {
            self.message = "Select a workflow from the menu.".into();
        } else {
            self.message = "Loading statuses...".into();
        }
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
            Screen::E2ETest => {
                if self.e2e_software.is_some() && !self.e2e_executing {
                    self.e2e_tab = match self.e2e_tab {
                        E2ETab::Install => E2ETab::Update,
                        E2ETab::Update => E2ETab::Remove,
                        E2ETab::Remove => E2ETab::Execute,
                        E2ETab::Execute => E2ETab::Install,
                    };
                }
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
            Screen::E2ETest => {
                if self.e2e_software.is_some() && !self.e2e_executing {
                    self.e2e_tab = match self.e2e_tab {
                        E2ETab::Install => E2ETab::Execute,
                        E2ETab::Update => E2ETab::Install,
                        E2ETab::Remove => E2ETab::Update,
                        E2ETab::Execute => E2ETab::Remove,
                    };
                }
            }
        }
    }

    pub fn activate_menu(&mut self) {
        if let Some(index) = self.menu_state.selected() {
            self.activate_menu_index(index);
        }
    }

    pub fn activate_menu_index(&mut self, index: usize) {
        if !self.menu_enabled {
            self.message = "Initial status refresh running; please wait.".into();
            return;
        }
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
                MenuAction::Versions => self.run_versions_check(),
                MenuAction::E2ETest => {
                    self.screen = Screen::E2ETest;
                    self.e2e_software = None;
                    self.e2e_tab = E2ETab::Install;
                    self.e2e_install_enabled = true;
                    self.e2e_update_enabled = true;
                    self.e2e_remove_enabled = true;
                    self.e2e_install_status = E2EStepStatus::Pending;
                    self.e2e_update_status = E2EStepStatus::Pending;
                    self.e2e_remove_status = E2EStepStatus::Pending;
                    self.e2e_executing = false;
                    self.message = "Select a software: 1=Neovim, 2=btop".into();
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

    pub fn toggle_task_view(&mut self) {
        self.show_tasks = !self.show_tasks;
        self.message = if self.show_tasks {
            "Showing task logs (press t to switch back).".into()
        } else {
            "Showing action log.".into()
        };
    }

    fn run_on_selected(&mut self, action: ActionKind) {
        if self.screen != Screen::Software {
            self.message = "Open the software catalog to manage individual entries.".into();
            return;
        }
        if !self.menu_enabled {
            self.message = "Initial status refresh running; please wait.".into();
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
            ActionKind::Install => self.manager.install_many(&ids, None),
            ActionKind::Update => self.manager.update_many(&ids, None),
            ActionKind::Uninstall => self.manager.uninstall_many(&ids, None),
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

    pub fn poll_task_events(&mut self) {
        while let Ok(event) = self.event_receiver.try_recv() {
            self.append_task_messages(event.id, event.label.clone(), event.messages);
            if let Some(statuses) = event.statuses {
                for report in statuses {
                    self.statuses.insert(report.id, report.state);
                }
                if let Some(task) = self.task_logs.iter_mut().find(|t| t.id == event.id) {
                    task.lines.push("done".into());
                    trim_lines(&mut task.lines);
                }
                if self.initial_task == Some(event.id) {
                    self.menu_enabled = true;
                    self.initial_task = None;
                    self.message = "Status refresh complete. Select a workflow.".into();
                }
            } else if self.initial_task == Some(event.id) {
                self.menu_enabled = true;
                self.initial_task = None;
                self.message = "Status refresh complete. Select a workflow.".into();
            }
        }
    }

    fn run_template_flow(&mut self, action: ActionKind) {
        match templates::load_named(DEFAULT_TEMPLATE) {
            Ok(template) => {
                let label = format!("{} `{}` template", action.label(), template.name);
                let request = TaskRequest {
                    id: self.next_task_id,
                    label: label.clone(),
                    action: TaskAction::TemplateFlow {
                        action,
                        template,
                        force: false,
                        dry_run: crate::options::global_dry_run(),
                    },
                };
                self.next_task_id += 1;
                let queued_id = request.id;
                let queued_label = request.label.clone();
                if self.task_sender.send(request).is_ok() {
                    self.message = format!("Queued {label}. Progress shown below.");
                    self.add_task_log(queued_id, queued_label, "queued".into());
                    self.show_tasks = true;
                } else {
                    self.message = "Failed to queue template task.".into();
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

    fn run_versions_check(&mut self) {
        let request = TaskRequest {
            id: self.next_task_id,
            label: "Versions refresh".into(),
            action: TaskAction::Versions,
        };
        self.next_task_id += 1;
        let queued_id = request.id;
        let queued_label = request.label.clone();
        if self.task_sender.send(request).is_ok() {
            self.message = "Queued version check; results will appear below.".into();
            self.add_task_log(queued_id, queued_label, "queued".into());
            self.show_tasks = true;
        } else {
            self.message = "Failed to queue version task.".into();
        }
    }

    pub fn e2e_software(&self) -> Option<SoftwareId> {
        self.e2e_software
    }

    pub fn e2e_tab(&self) -> E2ETab {
        self.e2e_tab
    }

    pub fn e2e_install_status(&self) -> E2EStepStatus {
        self.e2e_install_status
    }

    pub fn e2e_update_status(&self) -> E2EStepStatus {
        self.e2e_update_status
    }

    pub fn e2e_remove_status(&self) -> E2EStepStatus {
        self.e2e_remove_status
    }

    pub fn e2e_executing(&self) -> bool {
        self.e2e_executing
    }

    pub fn e2e_install_enabled(&self) -> bool {
        self.e2e_install_enabled
    }

    pub fn e2e_update_enabled(&self) -> bool {
        self.e2e_update_enabled
    }

    pub fn e2e_remove_enabled(&self) -> bool {
        self.e2e_remove_enabled
    }

    pub fn toggle_e2e_step(&mut self) {
        if self.e2e_executing {
            return;
        }
        match self.e2e_tab {
            E2ETab::Install => {
                self.e2e_install_enabled = !self.e2e_install_enabled;
                self.message = format!("Install step {}", if self.e2e_install_enabled { "enabled" } else { "disabled" });
            }
            E2ETab::Update => {
                self.e2e_update_enabled = !self.e2e_update_enabled;
                self.message = format!("Update step {}", if self.e2e_update_enabled { "enabled" } else { "disabled" });
            }
            E2ETab::Remove => {
                self.e2e_remove_enabled = !self.e2e_remove_enabled;
                self.message = format!("Remove step {}", if self.e2e_remove_enabled { "enabled" } else { "disabled" });
            }
            E2ETab::Execute => {
                // No toggle on Execute tab
            }
        }
    }

    pub fn select_e2e_software(&mut self, software: SoftwareId) {
        self.e2e_software = Some(software);
        self.message = format!("Selected {}. Configure steps, then navigate to Execute tab.", software.name());
    }

    pub fn execute_e2e(&mut self) {
        if let Some(software) = self.e2e_software {
            if !self.e2e_executing {
                self.e2e_executing = true;
                self.e2e_install_status = E2EStepStatus::Pending;
                self.e2e_update_status = E2EStepStatus::Pending;
                self.e2e_remove_status = E2EStepStatus::Pending;

                let request = TaskRequest {
                    id: self.next_task_id,
                    label: format!("E2E Test: {}", software.name()),
                    action: TaskAction::E2EFlow {
                        software,
                        install_enabled: self.e2e_install_enabled,
                        update_enabled: self.e2e_update_enabled,
                        remove_enabled: self.e2e_remove_enabled,
                    },
                };
                self.next_task_id += 1;
                let queued_id = request.id;
                let queued_label = request.label.clone();

                if self.task_sender.send(request).is_ok() {
                    self.message = "E2E test executing...".into();
                    self.add_task_log(queued_id, queued_label, "queued".into());
                    self.show_tasks = true;
                } else {
                    self.message = "Failed to queue E2E task.".into();
                    self.e2e_executing = false;
                }
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

    fn add_task_log(&mut self, id: u64, label: String, line: String) {
        if let Some(existing) = self.task_logs.iter_mut().find(|t| t.id == id) {
            existing.lines.push(line);
            trim_lines(&mut existing.lines);
            return;
        }
        if self.task_logs.len() >= MAX_TASKS {
            self.task_logs.remove(0);
        }
        let mut entry = TaskLog {
            id,
            label,
            lines: vec![line],
        };
        trim_lines(&mut entry.lines);
        self.task_logs.push(entry);
    }

    fn append_task_messages(&mut self, id: u64, label: String, messages: Vec<String>) {
        if messages.is_empty() {
            return;
        }
        if !self.task_logs.iter().any(|t| t.id == id) {
            self.add_task_log(id, label.clone(), format!("task queued: {}", label));
        }
        if let Some(task) = self.task_logs.iter_mut().find(|t| t.id == id) {
            for line in messages {
                task.lines.push(line);
            }
            trim_lines(&mut task.lines);
        }
    }
}

fn trim_lines(lines: &mut Vec<String>) {
    if lines.len() > MAX_TASK_LOG_LINES {
        let excess = lines.len() - MAX_TASK_LOG_LINES;
        lines.drain(0..excess);
    }
}

fn spawn_worker(request_rx: Receiver<TaskRequest>, event_tx: Sender<TaskEvent>) {
    thread::spawn(move || {
        while let Ok(request) = request_rx.recv() {
            match request.action {
                TaskAction::TemplateFlow {
                    action,
                    template,
                    force,
                    dry_run,
                } => {
                    let manager = SoftwareManager::with_flags(dry_run, force);
                    let mut plan_messages = vec![format!("{} (task #{})", request.label, request.id)];
                    match manager.plan(&template.software, action) {
                        Ok(plan) => {
                            plan_messages.push(format!("Plan includes {} steps", plan.len()));
                            for (idx, id) in plan.iter().take(MAX_TASK_LOG_LINES).enumerate() {
                                plan_messages.push(format!("  {:>2}. {}", idx + 1, id.name()));
                            }
                            if plan.len() > MAX_TASK_LOG_LINES {
                                plan_messages.push("  ...".into());
                            }

                            // Send plan immediately
                            send_event(&event_tx, request.id, &request.label, plan_messages, None);

                            if dry_run {
                                send_event(&event_tx, request.id, &request.label, vec!["Dry run complete.".into(), "done".into()], None);
                                continue;
                            }

                            // Execute with progress callback
                            let event_tx_clone = event_tx.clone();
                            let request_id = request.id;
                            let request_label = request.label.clone();
                            let mut callback = |event: &ExecutionEvent| {
                                send_event(
                                    &event_tx_clone,
                                    request_id,
                                    &request_label,
                                    vec![event.summary()],
                                    None,
                                );
                            };

                            let exec_result = match action {
                                ActionKind::Install => manager.install_many(&template.software, Some(&mut callback)),
                                ActionKind::Update => manager.update_many(&template.software, Some(&mut callback)),
                                ActionKind::Uninstall => manager.uninstall_many(&template.software, Some(&mut callback)),
                                ActionKind::Test => Ok(Vec::new()),
                            };

                            match exec_result {
                                Ok(_events) => {
                                    let statuses = manager.status_all();
                                    send_event(
                                        &event_tx,
                                        request.id,
                                        &request.label,
                                        vec!["done".into()],
                                        Some(statuses),
                                    );
                                }
                                Err(err) => {
                                    send_event(
                                        &event_tx,
                                        request.id,
                                        &request.label,
                                        vec![format!("Error: {}", err), "done".into()],
                                        None,
                                    );
                                }
                            }
                        }
                        Err(err) => {
                            send_event(&event_tx, request.id, &request.label, vec![format!("Failed to plan: {}", err), "done".into()], None);
                        }
                    }
                }
                TaskAction::Versions => {
                    let manager = SoftwareManager::new();
                    let reports = manager.status_all();
                    let mut messages =
                        vec![format!("Versions refreshed for {} entries", reports.len())];
                    messages.extend(summarize_reports(&reports));
                    messages.push("done".into());
                    send_event(
                        &event_tx,
                        request.id,
                        &request.label,
                        messages,
                        Some(reports),
                    );
                }
                TaskAction::E2EFlow { software, install_enabled, update_enabled, remove_enabled } => {
                    let dry_run = crate::options::global_dry_run();
                    let manager = SoftwareManager::with_flags(dry_run, false);
                    let software_vec = vec![software];

                    let event_tx_clone = event_tx.clone();
                    let request_id = request.id;
                    let request_label = request.label.clone();
                    let mut callback = |event: &ExecutionEvent| {
                        let mut messages = vec![format!("  {}", event.summary())];
                        if let Some(output) = &event.output {
                            // Split output into lines and indent each
                            for line in output.lines() {
                                messages.push(format!("    {}", line));
                            }
                        }
                        send_event(&event_tx_clone, request_id, &request_label, messages, None);
                    };

                    // Step 1: Install
                    if install_enabled {
                        send_event(&event_tx, request.id, &request.label, vec!["Step 1: Installing...".into()], None);
                        if !dry_run {
                            match manager.install_many(&software_vec, Some(&mut callback)) {
                                Ok(_) => {
                                    send_event(&event_tx, request.id, &request.label, vec!["  Install complete".into()], None);
                                }
                                Err(err) => {
                                    send_event(&event_tx, request.id, &request.label, vec![format!("  Install failed: {}", err)], None);
                                }
                            }
                        } else {
                            send_event(&event_tx, request.id, &request.label, vec!["  Install skipped (dry-run)".into()], None);
                        }
                    } else {
                        send_event(&event_tx, request.id, &request.label, vec!["Step 1: Install skipped (disabled)".into()], None);
                    }

                    // Step 2: Update
                    if update_enabled {
                        send_event(&event_tx, request.id, &request.label, vec!["Step 2: Updating...".into()], None);
                        if !dry_run {
                            match manager.update_many(&software_vec, Some(&mut callback)) {
                                Ok(_) => {
                                    send_event(&event_tx, request.id, &request.label, vec!["  Update complete".into()], None);
                                }
                                Err(err) => {
                                    send_event(&event_tx, request.id, &request.label, vec![format!("  Update failed: {}", err)], None);
                                }
                            }
                        } else {
                            send_event(&event_tx, request.id, &request.label, vec!["  Update skipped (dry-run)".into()], None);
                        }
                    } else {
                        send_event(&event_tx, request.id, &request.label, vec!["Step 2: Update skipped (disabled)".into()], None);
                    }

                    // Step 3: Remove
                    if remove_enabled {
                        send_event(&event_tx, request.id, &request.label, vec!["Step 3: Removing...".into()], None);
                        if !dry_run {
                            match manager.uninstall_many(&software_vec, Some(&mut callback)) {
                                Ok(_) => {
                                    send_event(&event_tx, request.id, &request.label, vec!["  Remove complete".into()], None);
                                }
                                Err(err) => {
                                    send_event(&event_tx, request.id, &request.label, vec![format!("  Remove failed: {}", err)], None);
                                }
                            }
                        } else {
                            send_event(&event_tx, request.id, &request.label, vec!["  Remove skipped (dry-run)".into()], None);
                        }
                    } else {
                        send_event(&event_tx, request.id, &request.label, vec!["Step 3: Remove skipped (disabled)".into()], None);
                    }

                    // Done
                    send_event(&event_tx, request.id, &request.label, vec!["E2E test complete".into(), "done".into()], None);
                }
            }
        }
    });
}

fn summarize_reports(reports: &[StatusReport]) -> Vec<String> {
    let mut lines = Vec::new();
    for report in reports.iter().take(MAX_TASK_LOG_LINES) {
        let summary = match &report.state {
            StatusState::Installed { version } => format!(
                "{} -> {}",
                report.id.name(),
                version.clone().unwrap_or_else(|| "installed".into())
            ),
            StatusState::NotInstalled => format!("{} -> missing", report.id.name()),
            StatusState::ManualCheck(note) => {
                format!("{} -> manual ({})", report.id.name(), note)
            }
            StatusState::Unknown(note) => {
                format!("{} -> unknown ({})", report.id.name(), note)
            }
        };
        lines.push(format!("versions -> {}", summary));
    }
    if reports.len() > MAX_TASK_LOG_LINES {
        lines.push("versions -> ...".into());
    }
    lines
}

fn send_event(
    sender: &Sender<TaskEvent>,
    id: u64,
    label: &str,
    messages: Vec<String>,
    statuses: Option<Vec<StatusReport>>,
) {
    let _ = sender.send(TaskEvent {
        id,
        label: label.to_string(),
        messages,
        statuses,
    });
}
