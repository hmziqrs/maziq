#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SoftwareId {
    Homebrew,
    XcodeCommandLineTools,
    Brave,
    Firefox,
    Chrome,
    JetBrainsToolbox,
    ZedStable,
    ZedPreview,
    Raycast,
    Rustup,
    RustStable,
    RustNightly,
    CargoJust,
    CargoBinstall,
    CargoWatch,
    SimpleHttpServer,
    Nvm,
    Bun,
    Go,
    Flutter,
    ReactNativeCli,
    DioxusCli,
}

pub struct SoftwareCategory {
    pub name: &'static str,
    pub entries: &'static [SoftwareId],
}

#[derive(Clone, Copy)]
pub struct SoftwareHandle {
    pub category: &'static str,
    pub id: SoftwareId,
}

pub fn flattened_handles() -> Vec<SoftwareHandle> {
    CATEGORIES
        .iter()
        .flat_map(|category| {
            category.entries.iter().map(move |id| SoftwareHandle {
                category: category.name,
                id: *id,
            })
        })
        .collect()
}

impl SoftwareId {
    pub fn key(&self) -> &'static str {
        match self {
            Self::Homebrew => "homebrew",
            Self::XcodeCommandLineTools => "xcode_clt",
            Self::Brave => "brave",
            Self::Firefox => "firefox",
            Self::Chrome => "chrome",
            Self::JetBrainsToolbox => "jetbrains_toolbox",
            Self::ZedStable => "zed_stable",
            Self::ZedPreview => "zed_preview",
            Self::Raycast => "raycast",
            Self::Rustup => "rustup",
            Self::RustStable => "rust_stable",
            Self::RustNightly => "rust_nightly",
            Self::CargoJust => "cargo_just",
            Self::CargoBinstall => "cargo_binstall",
            Self::CargoWatch => "cargo_watch",
            Self::SimpleHttpServer => "simple_http_server",
            Self::Nvm => "nvm",
            Self::Bun => "bun",
            Self::Go => "go",
            Self::Flutter => "flutter",
            Self::ReactNativeCli => "react_native_cli",
            Self::DioxusCli => "dioxus_cli",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Homebrew => "Homebrew",
            Self::XcodeCommandLineTools => "Xcode Command-Line Tools",
            Self::Brave => "Brave Browser",
            Self::Firefox => "Firefox",
            Self::Chrome => "Google Chrome",
            Self::JetBrainsToolbox => "JetBrains Toolbox",
            Self::ZedStable => "Zed (Stable)",
            Self::ZedPreview => "Zed (Preview)",
            Self::Raycast => "Raycast",
            Self::Rustup => "Rustup",
            Self::RustStable => "Rust Toolchain (Stable)",
            Self::RustNightly => "Rust Toolchain (Nightly)",
            Self::CargoJust => "cargo-just",
            Self::CargoBinstall => "cargo-binstall",
            Self::CargoWatch => "cargo-watch",
            Self::SimpleHttpServer => "simple-http-server (nightly)",
            Self::Nvm => "Node Version Manager (nvm)",
            Self::Bun => "Bun Runtime",
            Self::Go => "Go Toolchain",
            Self::Flutter => "Flutter SDK",
            Self::ReactNativeCli => "React Native CLI",
            Self::DioxusCli => "Dioxus CLI",
        }
    }

    pub fn summary(&self) -> &'static str {
        match self {
            Self::Homebrew => "Package manager foundation for macOS.",
            Self::XcodeCommandLineTools => "Compilers and SDK headers from Apple.",
            Self::Brave => "Privacy-focused Chromium browser.",
            Self::Firefox => "Mozilla’s versatile browser.",
            Self::Chrome => "Google’s mainstream browser.",
            Self::JetBrainsToolbox => "Launcher for JetBrains IDEs (Android Studio).",
            Self::ZedStable => "Zed editor stable channel.",
            Self::ZedPreview => "Zed editor preview channel.",
            Self::Raycast => "Productivity command palette for macOS.",
            Self::Rustup => "Rust toolchain installer.",
            Self::RustStable => "Sets Rust stable as default toolchain.",
            Self::RustNightly => "Installs nightly Rust toolchain.",
            Self::CargoJust => "Handy task runner for Rust projects.",
            Self::CargoBinstall => "Fast binary installer for cargo packages.",
            Self::CargoWatch => "Auto compile/test watcher for Rust.",
            Self::SimpleHttpServer => "Nightly-only HTTP dev server.",
            Self::Nvm => "Manage multiple Node.js versions.",
            Self::Bun => "All-in-one JS runtime/bundler.",
            Self::Go => "Google’s Go language toolchain.",
            Self::Flutter => "Google’s UI SDK for mobile & desktop.",
            Self::ReactNativeCli => "React Native project scaffolding CLI.",
            Self::DioxusCli => "Rust-based cross-platform UI tooling.",
        }
    }
}

pub const CATEGORIES: &[SoftwareCategory] = &[
    SoftwareCategory {
        name: "System Essentials",
        entries: &[SoftwareId::Homebrew, SoftwareId::XcodeCommandLineTools],
    },
    SoftwareCategory {
        name: "Browsers",
        entries: &[SoftwareId::Brave, SoftwareId::Firefox, SoftwareId::Chrome],
    },
    SoftwareCategory {
        name: "Editors",
        entries: &[
            SoftwareId::JetBrainsToolbox,
            SoftwareId::ZedStable,
            SoftwareId::ZedPreview,
        ],
    },
    SoftwareCategory {
        name: "Desktop Utilities",
        entries: &[SoftwareId::Raycast],
    },
    SoftwareCategory {
        name: "Rust Stack",
        entries: &[
            SoftwareId::Rustup,
            SoftwareId::RustStable,
            SoftwareId::RustNightly,
            SoftwareId::CargoJust,
            SoftwareId::CargoBinstall,
            SoftwareId::CargoWatch,
            SoftwareId::SimpleHttpServer,
            SoftwareId::DioxusCli,
        ],
    },
    SoftwareCategory {
        name: "JavaScript & Node",
        entries: &[SoftwareId::Nvm, SoftwareId::Bun, SoftwareId::ReactNativeCli],
    },
    SoftwareCategory {
        name: "Mobile / Cross-Platform",
        entries: &[SoftwareId::Flutter],
    },
    SoftwareCategory {
        name: "Languages",
        entries: &[SoftwareId::Go],
    },
];
