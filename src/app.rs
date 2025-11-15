use std::collections::{HashMap, HashSet};

use ratatui::widgets::ListState;

use crate::{
    catalog::{self, SoftwareHandle, SoftwareId},
    operations, persistence,
};

#[derive(Clone, Default)]
pub struct SoftwareStatus {
    pub installed: bool,
    pub error: Option<String>,
}

pub struct App {
    handles: Vec<SoftwareHandle>,
    state: ListState,
    statuses: HashMap<SoftwareId, SoftwareStatus>,
    progress: HashSet<String>,
    pub message: String,
    pub quit: bool,
}

impl App {
    pub fn new() -> Self {
        let handles = catalog::flattened_handles();
        let progress = persistence::load_progress();
        let mut state = ListState::default();
        if !handles.is_empty() {
            state.select(Some(0));
        }

        let mut app = Self {
            handles,
            state,
            statuses: HashMap::new(),
            progress,
            message: "Select software and press Enter to install.".into(),
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

    pub fn status_for(&self, id: SoftwareId) -> SoftwareStatus {
        self.statuses.get(&id).cloned().unwrap_or_default()
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
        if let Some(index) = self.state.selected() {
            let handle = self.handles[index];
            let status = self.status_for(handle.id);
            if status.installed {
                self.message = format!("{} is already installed.", handle.id.name());
                return;
            }
            self.message = format!("Installing {}...", handle.id.name());
            match operations::install(handle.id) {
                Ok(_) => {
                    self.mark_installed(handle.id);
                    self.message = format!("{} installed successfully.", handle.id.name());
                }
                Err(err) => {
                    self.statuses.insert(
                        handle.id,
                        SoftwareStatus {
                            installed: false,
                            error: Some(err.to_string()),
                        },
                    );
                    self.message = format!("Failed to install {}: {}", handle.id.name(), err);
                }
            }
        }
    }

    pub fn install_all_missing(&mut self) {
        self.message = "Installing all missing software...".into();
        let handles_snapshot = self.handles.clone();
        for handle in handles_snapshot {
            let status = self.status_for(handle.id);
            if status.installed {
                continue;
            }
            match operations::install(handle.id) {
                Ok(_) => self.mark_installed(handle.id),
                Err(err) => {
                    self.statuses.insert(
                        handle.id,
                        SoftwareStatus {
                            installed: false,
                            error: Some(err.to_string()),
                        },
                    );
                    self.message = format!(
                        "Paused on {} due to error (see list). Continue later.",
                        handle.id.name()
                    );
                }
            }
        }
        self.message = "Batch installation complete. Review statuses.".into();
    }

    pub fn refresh_statuses_with_feedback(&mut self) {
        self.refresh_statuses();
        self.message = "Statuses refreshed.".into();
    }

    pub fn refresh_statuses(&mut self) {
        for handle in &self.handles {
            let installed =
                operations::check_installed(handle.id) || self.progress.contains(handle.id.key());
            self.statuses.insert(
                handle.id,
                SoftwareStatus {
                    installed,
                    error: None,
                },
            );
        }
    }

    fn mark_installed(&mut self, id: SoftwareId) {
        self.statuses.insert(
            id,
            SoftwareStatus {
                installed: true,
                error: None,
            },
        );
        self.progress.insert(id.key().to_string());
        if let Err(err) = persistence::save_progress(&self.progress) {
            self.message = format!(
                "{} installed, but failed to write progress: {}",
                id.name(),
                err
            );
        }
    }
}
