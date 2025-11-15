use std::collections::HashMap;

use ratatui::widgets::ListState;

use crate::{
    catalog::{self, SoftwareHandle, SoftwareId},
    manager::{ActionKind, ExecutionEvent, SoftwareManager, StatusState},
};

const MAX_LOG_ENTRIES: usize = 6;

pub struct App {
    handles: Vec<SoftwareHandle>,
    state: ListState,
    statuses: HashMap<SoftwareId, StatusState>,
    manager: SoftwareManager,
    log: Vec<String>,
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

        let mut app = Self {
            handles,
            state,
            statuses: HashMap::new(),
            manager: SoftwareManager::new(),
            log: Vec::new(),
            message: "Select software and use Enter/U/X to install/update/uninstall.".into(),
            quit: false,
        };
        app.refresh_statuses();
        app
    }

    pub fn handles(&self) -> &[SoftwareHandle] {
        &self.handles
    }

    pub fn list_state_mut(&mut self) -> &mut ListState {
        &mut self.state
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

    pub fn next(&mut self) {
        if self.handles.is_empty() {
            return;
        }
        let current = self.state.selected().unwrap_or(0);
        let next = (current + 1) % self.handles.len();
        self.state.select(Some(next));
    }

    pub fn previous(&mut self) {
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
