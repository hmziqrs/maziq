use std::{
    collections::HashSet,
    error::Error,
    fmt,
    process::{Command, Output, Stdio},
};

use crate::catalog::{self, CommandRecipe, SoftwareEntry, SoftwareId, VersionProbe};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionKind {
    Install,
    Update,
    Uninstall,
    Test,
}

impl ActionKind {
    pub fn label(&self) -> &'static str {
        match self {
            ActionKind::Install => "install",
            ActionKind::Update => "update",
            ActionKind::Uninstall => "uninstall",
            ActionKind::Test => "test",
        }
    }
}

#[derive(Debug)]
pub struct ExecutionEvent {
    pub id: SoftwareId,
    pub action: ActionKind,
    pub command: Option<String>,
    pub note: Option<String>,
    pub skipped: bool,
}

impl ExecutionEvent {
    pub fn summary(&self) -> String {
        let prefix = format!("[{}] {}", self.action.label(), self.id.name());
        if let Some(note) = &self.note {
            format!("{prefix}: {note}")
        } else if let Some(cmd) = &self.command {
            if self.skipped {
                format!("{prefix}: dry-run -> {cmd}")
            } else {
                format!("{prefix}: {cmd}")
            }
        } else if self.skipped {
            format!("{prefix}: skipped")
        } else {
            format!("{prefix}: completed")
        }
    }
}

#[derive(Clone, Debug)]
pub struct StatusReport {
    pub id: SoftwareId,
    pub state: StatusState,
}

#[derive(Clone, Debug)]
pub enum StatusState {
    Installed { version: Option<String> },
    NotInstalled,
    ManualCheck(String),
    Unknown(String),
}

#[derive(Debug)]
pub enum ManagerError {
    CycleDetected(SoftwareId),
    CommandFailed {
        command: String,
        stderr: String,
    },
    Spawn(std::io::Error),
    UnsafeGuiCommand {
        id: SoftwareId,
        command: &'static str,
    },
}

impl fmt::Display for ManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ManagerError::CycleDetected(id) => {
                write!(f, "Dependency cycle detected while resolving {}", id.name())
            }
            ManagerError::CommandFailed { command, stderr } => {
                write!(f, "Command `{command}` failed: {stderr}")
            }
            ManagerError::Spawn(err) => write!(f, "Failed to spawn command: {err}"),
            ManagerError::UnsafeGuiCommand { id, command } => write!(
                f,
                "GUI application `{}` attempted to run unsafe command `{}`. Only application binaries should be managed.",
                id.name(),
                command
            ),
        }
    }
}

impl Error for ManagerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ManagerError::Spawn(err) => Some(err),
            _ => None,
        }
    }
}

pub trait SoftwareAdapter {
    fn id(&self) -> SoftwareId;
    fn install(&self, exec: &CommandExecutor) -> Result<ExecutionEvent, ManagerError>;
    fn update(&self, exec: &CommandExecutor) -> Result<ExecutionEvent, ManagerError>;
    fn uninstall(&self, exec: &CommandExecutor) -> Result<ExecutionEvent, ManagerError>;
    fn status(&self, exec: &CommandExecutor) -> Result<StatusReport, ManagerError>;
    fn test(&self, _exec: &CommandExecutor) -> Result<ExecutionEvent, ManagerError> {
        Ok(ExecutionEvent {
            id: self.id(),
            action: ActionKind::Test,
            command: None,
            note: Some("Test command not implemented".into()),
            skipped: true,
        })
    }
}

struct CatalogAdapter {
    entry: SoftwareEntry,
}

impl CatalogAdapter {
    fn new(entry: SoftwareEntry) -> Self {
        Self { entry }
    }

    fn run_recipe(
        &self,
        exec: &CommandExecutor,
        action: ActionKind,
        recipe: &CommandRecipe,
    ) -> Result<ExecutionEvent, ManagerError> {
        match recipe {
            CommandRecipe::Shell(cmd) => {
                if self.entry.kind == catalog::SoftwareKind::GuiApplication
                    && !(cmd.starts_with("brew install --cask")
                        || cmd.starts_with("brew upgrade --cask")
                        || cmd.starts_with("brew uninstall --cask"))
                {
                    return Err(ManagerError::UnsafeGuiCommand {
                        id: self.entry.id,
                        command: cmd,
                    });
                }
                exec.run_shell(cmd)?;
                Ok(ExecutionEvent {
                    id: self.entry.id,
                    action,
                    command: Some((*cmd).to_string()),
                    note: None,
                    skipped: exec.dry_run(),
                })
            }
            CommandRecipe::Manual(note) => Ok(ExecutionEvent {
                id: self.entry.id,
                action,
                command: None,
                note: Some((*note).to_string()),
                skipped: true,
            }),
        }
    }
}

impl SoftwareAdapter for CatalogAdapter {
    fn id(&self) -> SoftwareId {
        self.entry.id
    }

    fn install(&self, exec: &CommandExecutor) -> Result<ExecutionEvent, ManagerError> {
        self.run_recipe(exec, ActionKind::Install, &self.entry.install)
    }

    fn update(&self, exec: &CommandExecutor) -> Result<ExecutionEvent, ManagerError> {
        self.run_recipe(exec, ActionKind::Update, &self.entry.update)
    }

    fn uninstall(&self, exec: &CommandExecutor) -> Result<ExecutionEvent, ManagerError> {
        self.run_recipe(exec, ActionKind::Uninstall, &self.entry.uninstall)
    }

    fn status(&self, exec: &CommandExecutor) -> Result<StatusReport, ManagerError> {
        let state = exec.detect_version(&self.entry.version_probe);
        Ok(StatusReport {
            id: self.entry.id,
            state,
        })
    }
}

pub struct SoftwareManager {
    dry_run: bool,
    force: bool,
}

impl Default for SoftwareManager {
    fn default() -> Self {
        Self {
            dry_run: false,
            force: false,
        }
    }
}

impl SoftwareManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_flags(dry_run: bool, force: bool) -> Self {
        Self { dry_run, force }
    }

    pub fn install(&self, id: SoftwareId) -> Result<Vec<ExecutionEvent>, ManagerError> {
        self.run_for_ids(&[id], ActionKind::Install)
    }

    pub fn update(&self, id: SoftwareId) -> Result<Vec<ExecutionEvent>, ManagerError> {
        self.run_for_ids(&[id], ActionKind::Update)
    }

    pub fn uninstall(&self, id: SoftwareId) -> Result<Vec<ExecutionEvent>, ManagerError> {
        self.run_for_ids(&[id], ActionKind::Uninstall)
    }

    pub fn install_many(&self, ids: &[SoftwareId]) -> Result<Vec<ExecutionEvent>, ManagerError> {
        self.run_for_ids(ids, ActionKind::Install)
    }

    pub fn update_many(&self, ids: &[SoftwareId]) -> Result<Vec<ExecutionEvent>, ManagerError> {
        self.run_for_ids(ids, ActionKind::Update)
    }

    pub fn uninstall_many(&self, ids: &[SoftwareId]) -> Result<Vec<ExecutionEvent>, ManagerError> {
        self.run_for_ids(ids, ActionKind::Uninstall)
    }

    pub fn plan(
        &self,
        ids: &[SoftwareId],
        action: ActionKind,
    ) -> Result<Vec<SoftwareId>, ManagerError> {
        let mut order = self.resolve_order(ids)?;
        if matches!(action, ActionKind::Uninstall) {
            order.reverse();
        }
        Ok(order)
    }

    pub fn status(&self, id: SoftwareId) -> Result<StatusReport, ManagerError> {
        let adapter = CatalogAdapter::new(catalog::entry(id));
        adapter.status(&CommandExecutor::for_status())
    }

    pub fn status_all(&self) -> Vec<StatusReport> {
        catalog::all_entries()
            .into_iter()
            .map(|entry| {
                let id = entry.id;
                let adapter = CatalogAdapter::new(entry);
                adapter
                    .status(&CommandExecutor::for_status())
                    .unwrap_or_else(|err| StatusReport {
                        id,
                        state: StatusState::Unknown(err.to_string()),
                    })
            })
            .collect()
    }

    fn run_for_ids(
        &self,
        ids: &[SoftwareId],
        action: ActionKind,
    ) -> Result<Vec<ExecutionEvent>, ManagerError> {
        let mut order = self.resolve_order(ids)?;
        if matches!(action, ActionKind::Uninstall) {
            order.reverse();
        }
        let exec = CommandExecutor::new(self.dry_run);
        let mut events = Vec::new();
        for id in order {
            let adapter = CatalogAdapter::new(catalog::entry(id));
            if matches!(action, ActionKind::Install)
                && !self.force
                && adapter.entry.kind == catalog::SoftwareKind::GuiApplication
            {
                if let Ok(report) = adapter.status(&exec) {
                    if matches!(report.state, StatusState::Installed { .. }) {
                        events.push(ExecutionEvent {
                            id,
                            action,
                            command: None,
                            note: Some(
                                "Already installed; run install --force to reinstall.".into(),
                            ),
                            skipped: true,
                        });
                        continue;
                    }
                }
            }
            let event = match action {
                ActionKind::Install => adapter.install(&exec)?,
                ActionKind::Update => adapter.update(&exec)?,
                ActionKind::Uninstall => adapter.uninstall(&exec)?,
                ActionKind::Test => adapter.test(&exec)?,
            };
            events.push(event);
        }
        Ok(events)
    }

    fn resolve_order(&self, roots: &[SoftwareId]) -> Result<Vec<SoftwareId>, ManagerError> {
        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();
        let mut order = Vec::new();
        for id in roots {
            self.visit(*id, &mut visiting, &mut visited, &mut order)?;
        }
        Ok(order)
    }

    fn visit(
        &self,
        id: SoftwareId,
        visiting: &mut HashSet<SoftwareId>,
        visited: &mut HashSet<SoftwareId>,
        order: &mut Vec<SoftwareId>,
    ) -> Result<(), ManagerError> {
        if visited.contains(&id) {
            return Ok(());
        }
        if !visiting.insert(id) {
            return Err(ManagerError::CycleDetected(id));
        }
        let entry = catalog::entry(id);
        for dep in entry.dependencies {
            self.visit(*dep, visiting, visited, order)?;
        }
        visiting.remove(&id);
        visited.insert(id);
        order.push(id);
        Ok(())
    }
}

pub struct CommandExecutor {
    dry_run: bool,
}

impl CommandExecutor {
    fn new(dry_run: bool) -> Self {
        Self { dry_run }
    }

    fn for_status() -> Self {
        Self { dry_run: false }
    }

    fn dry_run(&self) -> bool {
        self.dry_run
    }

    fn run_shell(&self, command: &str) -> Result<(), ManagerError> {
        if self.dry_run {
            return Ok(());
        }
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(ManagerError::Spawn)?;
        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(ManagerError::CommandFailed {
                command: command.to_string(),
                stderr,
            })
        }
    }

    fn detect_version(&self, probe: &VersionProbe) -> StatusState {
        match probe {
            VersionProbe::Mdls { path } => self.detect_mdls(path),
            VersionProbe::Command { program, args } => self.detect_command(program, args),
            VersionProbe::BrewList { package } => {
                self.detect_command("brew", &["list", "--versions", package])
            }
            VersionProbe::Manual(note) => StatusState::ManualCheck((*note).to_string()),
        }
    }

    fn detect_mdls(&self, path: &str) -> StatusState {
        match Command::new("mdls")
            .arg("-name")
            .arg("kMDItemVersion")
            .arg(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    parse_mdls_version_output(output)
                } else {
                    StatusState::NotInstalled
                }
            }
            Err(_) => StatusState::Unknown("failed to run mdls".into()),
        }
    }

    fn detect_command(&self, program: &str, args: &[&str]) -> StatusState {
        let mut cmd = Command::new(program);
        cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());
        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    StatusState::Installed {
                        version: if stdout.is_empty() {
                            None
                        } else {
                            Some(stdout)
                        },
                    }
                } else {
                    StatusState::NotInstalled
                }
            }
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    StatusState::NotInstalled
                } else {
                    StatusState::Unknown(err.to_string())
                }
            }
        }
    }
}

fn parse_mdls_version_output(output: Output) -> StatusState {
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if let Some(value) = line.split('=').nth(1) {
            let cleaned = value.trim().trim_matches('"').to_string();
            if cleaned.is_empty() {
                return StatusState::Installed { version: None };
            } else {
                return StatusState::Installed {
                    version: Some(cleaned),
                };
            }
        }
    }
    StatusState::Installed { version: None }
}
