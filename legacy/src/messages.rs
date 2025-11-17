use crate::catalog::SoftwareId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppMsg {
    // Navigation messages
    GoToMenu,
    GoToSoftwareCatalog,
    GoToE2ETest,

    // Menu screen messages
    ActivateMenuItem(usize),

    // Software catalog messages
    InstallSoftware(SoftwareId),
    UpdateSoftware(SoftwareId),
    UninstallSoftware(SoftwareId),
    InstallAllMissing,
    RefreshStatuses,

    // E2E test messages
    SelectE2ESoftware(SoftwareId),
    ToggleE2EInstall,
    ToggleE2EUpdate,
    ToggleE2ERemove,
    ExecuteE2E,

    // Task management
    ToggleTaskView,

    // Application control
    Quit,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComponentId {
    Menu,
    Software,
    E2ETest,
    StatusBar,
    LogPanel,
}
