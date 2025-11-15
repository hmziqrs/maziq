#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SoftwareId {
    Homebrew,
    XcodeCommandLineTools,
    Brave,
    Firefox,
    Chrome,
    JetBrainsToolbox,
    Cursor,
    Windsurf,
    VisualStudioCode,
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
    AndroidStudio,
    ReactNativeCli,
    DioxusCli,
    YewCli,
    LeptosCli,
    ElectronForge,
    DockerDesktop,
    Postman,
    Yaak,
    Bruno,
    CodexCli,
    ClaudeCli,
    ClaudeMultiCli,
    KimiCli,
    GeminiCli,
    QwenCli,
    OpencodeCli,
}

pub const ALL_SOFTWARE: &[SoftwareId] = &[
    SoftwareId::Homebrew,
    SoftwareId::XcodeCommandLineTools,
    SoftwareId::Brave,
    SoftwareId::Firefox,
    SoftwareId::Chrome,
    SoftwareId::JetBrainsToolbox,
    SoftwareId::Cursor,
    SoftwareId::Windsurf,
    SoftwareId::VisualStudioCode,
    SoftwareId::ZedStable,
    SoftwareId::ZedPreview,
    SoftwareId::Raycast,
    SoftwareId::Rustup,
    SoftwareId::RustStable,
    SoftwareId::RustNightly,
    SoftwareId::CargoJust,
    SoftwareId::CargoBinstall,
    SoftwareId::CargoWatch,
    SoftwareId::SimpleHttpServer,
    SoftwareId::Nvm,
    SoftwareId::Bun,
    SoftwareId::Go,
    SoftwareId::Flutter,
    SoftwareId::AndroidStudio,
    SoftwareId::ReactNativeCli,
    SoftwareId::DioxusCli,
    SoftwareId::YewCli,
    SoftwareId::LeptosCli,
    SoftwareId::ElectronForge,
    SoftwareId::DockerDesktop,
    SoftwareId::Postman,
    SoftwareId::Yaak,
    SoftwareId::Bruno,
    SoftwareId::CodexCli,
    SoftwareId::ClaudeCli,
    SoftwareId::ClaudeMultiCli,
    SoftwareId::KimiCli,
    SoftwareId::GeminiCli,
    SoftwareId::QwenCli,
    SoftwareId::OpencodeCli,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SoftwareKind {
    GuiApplication,
    CliTool,
    Sdk,
}

impl SoftwareKind {
    pub fn label(&self) -> &'static str {
        match self {
            SoftwareKind::GuiApplication => "GUI",
            SoftwareKind::CliTool => "CLI",
            SoftwareKind::Sdk => "SDK",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum VersionProbe {
    Mdls {
        path: &'static str,
    },
    Command {
        program: &'static str,
        args: &'static [&'static str],
    },
    BrewList {
        package: &'static str,
    },
    Manual(&'static str),
}

impl VersionProbe {
    pub fn description(&self) -> String {
        match self {
            VersionProbe::Mdls { path } => {
                format!("mdls -name kMDItemVersion {}", path.replace(' ', "\\ "))
            }
            VersionProbe::Command { program, args } => {
                let joined = if args.is_empty() {
                    String::new()
                } else {
                    format!(" {}", args.join(" "))
                };
                format!("{program}{joined}")
            }
            VersionProbe::BrewList { package } => format!("brew list --versions {package}"),
            VersionProbe::Manual(note) => format!("Manual check: {note}"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CommandRecipe {
    Shell(&'static str),
    Manual(&'static str),
}

impl CommandRecipe {
    pub fn description(&self) -> &'static str {
        match self {
            CommandRecipe::Shell(cmd) => cmd,
            CommandRecipe::Manual(note) => note,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SoftwareEntry {
    pub id: SoftwareId,
    pub display_name: &'static str,
    pub category: &'static str,
    pub summary: &'static str,
    pub kind: SoftwareKind,
    pub dependencies: &'static [SoftwareId],
    pub version_probe: VersionProbe,
    pub install: CommandRecipe,
    pub update: CommandRecipe,
    pub uninstall: CommandRecipe,
}

impl SoftwareEntry {
    pub fn from_id(id: SoftwareId) -> Self {
        Self {
            id,
            display_name: id.name(),
            category: id.category_name(),
            summary: id.summary(),
            kind: id.kind(),
            dependencies: id.dependencies(),
            version_probe: id.version_probe(),
            install: id.install_recipe(),
            update: id.update_recipe(),
            uninstall: id.uninstall_recipe(),
        }
    }
}

impl SoftwareId {
    pub fn all() -> &'static [SoftwareId] {
        ALL_SOFTWARE
    }

    pub fn key(&self) -> &'static str {
        match self {
            Self::Homebrew => "homebrew",
            Self::XcodeCommandLineTools => "xcode_clt",
            Self::Brave => "brave",
            Self::Firefox => "firefox",
            Self::Chrome => "chrome",
            Self::JetBrainsToolbox => "jetbrains_toolbox",
            Self::Cursor => "cursor",
            Self::Windsurf => "windsurf",
            Self::VisualStudioCode => "visual_studio_code",
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
            Self::AndroidStudio => "android_studio",
            Self::ReactNativeCli => "react_native_cli",
            Self::DioxusCli => "dioxus_cli",
            Self::YewCli => "yew_cli",
            Self::LeptosCli => "leptos_cli",
            Self::ElectronForge => "electron_forge",
            Self::DockerDesktop => "docker_desktop",
            Self::Postman => "postman",
            Self::Yaak => "yaak",
            Self::Bruno => "bruno",
            Self::CodexCli => "codex_cli",
            Self::ClaudeCli => "claude_cli",
            Self::ClaudeMultiCli => "claude_multi_cli",
            Self::KimiCli => "kimi_cli",
            Self::GeminiCli => "gemini_cli",
            Self::QwenCli => "qwen_cli",
            Self::OpencodeCli => "opencode_cli",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        ALL_SOFTWARE.iter().copied().find(|id| id.key() == key)
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Homebrew => "Homebrew",
            Self::XcodeCommandLineTools => "Xcode Command-Line Tools",
            Self::Brave => "Brave Browser",
            Self::Firefox => "Firefox",
            Self::Chrome => "Google Chrome",
            Self::JetBrainsToolbox => "JetBrains Toolbox",
            Self::Cursor => "Cursor",
            Self::Windsurf => "Windsurf",
            Self::VisualStudioCode => "Visual Studio Code",
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
            Self::AndroidStudio => "Android Studio",
            Self::ReactNativeCli => "React Native CLI",
            Self::DioxusCli => "Dioxus CLI",
            Self::YewCli => "Yew CLI",
            Self::LeptosCli => "Leptos CLI",
            Self::ElectronForge => "Electron Forge",
            Self::DockerDesktop => "Docker Desktop",
            Self::Postman => "Postman",
            Self::Yaak => "Yaak",
            Self::Bruno => "Bruno",
            Self::CodexCli => "Codex CLI",
            Self::ClaudeCli => "Claude CLI",
            Self::ClaudeMultiCli => "Claude Multi CLI",
            Self::KimiCli => "Kimi CLI",
            Self::GeminiCli => "Gemini CLI",
            Self::QwenCli => "Qwen CLI",
            Self::OpencodeCli => "Opencode CLI",
        }
    }

    pub fn summary(&self) -> &'static str {
        match self {
            Self::Homebrew => "Package manager foundation for macOS.",
            Self::XcodeCommandLineTools => "Compilers and SDK headers from Apple.",
            Self::Brave => "Privacy-focused Chromium browser.",
            Self::Firefox => "Mozilla’s versatile browser.",
            Self::Chrome => "Google’s mainstream browser.",
            Self::JetBrainsToolbox => "Launcher for JetBrains IDEs (including Android Studio).",
            Self::Cursor => "AI-native editor focused on flow with built-in agent.",
            Self::Windsurf => "Codeium’s AI pair-programming IDE.",
            Self::VisualStudioCode => "Microsoft’s extensible editor and IDE.",
            Self::ZedStable => "Zed editor stable channel.",
            Self::ZedPreview => "Zed editor preview channel.",
            Self::Raycast => "Productivity command palette for macOS.",
            Self::Rustup => "Rust toolchain installer and updater.",
            Self::RustStable => "Sets Rust stable toolchain as default.",
            Self::RustNightly => "Installs nightly Rust toolchain.",
            Self::CargoJust => "Handy task runner for Rust projects.",
            Self::CargoBinstall => "Fast binary installer for cargo packages.",
            Self::CargoWatch => "Auto compile/test watcher for Rust.",
            Self::SimpleHttpServer => "Nightly-only HTTP dev server.",
            Self::Nvm => "Manage multiple Node.js versions.",
            Self::Bun => "All-in-one JS runtime/bundler.",
            Self::Go => "Google’s Go language toolchain.",
            Self::Flutter => "Google’s UI SDK for mobile & desktop.",
            Self::AndroidStudio => "Google’s official IDE for Android development.",
            Self::ReactNativeCli => "React Native project scaffolding CLI.",
            Self::DioxusCli => "Rust-based cross-platform UI tooling.",
            Self::YewCli => "CLI helpers for Yew web apps.",
            Self::LeptosCli => "Full-stack Leptos project manager.",
            Self::ElectronForge => "Electron scaffolding, packaging, and release tooling.",
            Self::DockerDesktop => "GUI and runtime for local Docker containers.",
            Self::Postman => "API design, testing, and collaboration suite.",
            Self::Yaak => "Lightweight REST and GraphQL API client.",
            Self::Bruno => "Text-based API collections and testing tool.",
            Self::CodexCli => "Command-line interface for Codex-style AI coding.",
            Self::ClaudeCli => "Anthropic Claude assistant from the terminal.",
            Self::ClaudeMultiCli => "Manage multiple Claude sessions and keys via CLI.",
            Self::KimiCli => "Terminal gateway to Kimi AI assistant.",
            Self::GeminiCli => "Interact with Google Gemini models from the CLI.",
            Self::QwenCli => "CLI helper for Alibaba Cloud’s Qwen assistants.",
            Self::OpencodeCli => "Dispatch open-source code LLMs via CLI workflows.",
        }
    }

    pub fn category_name(&self) -> &'static str {
        match self {
            Self::Homebrew | Self::XcodeCommandLineTools => "System Essentials",
            Self::Brave | Self::Firefox | Self::Chrome => "Browsers",
            Self::JetBrainsToolbox
            | Self::Cursor
            | Self::Windsurf
            | Self::VisualStudioCode
            | Self::ZedStable
            | Self::ZedPreview => "Editors & IDEs",
            Self::Raycast | Self::DockerDesktop => "Desktop Utilities",
            Self::Postman | Self::Yaak | Self::Bruno => "API & Testing",
            Self::Rustup
            | Self::RustStable
            | Self::RustNightly
            | Self::CargoJust
            | Self::CargoBinstall
            | Self::CargoWatch
            | Self::SimpleHttpServer
            | Self::DioxusCli
            | Self::YewCli
            | Self::LeptosCli => "Rust Stack",
            Self::Nvm
            | Self::Bun
            | Self::ElectronForge
            | Self::CodexCli
            | Self::ClaudeCli
            | Self::ClaudeMultiCli
            | Self::KimiCli
            | Self::GeminiCli
            | Self::QwenCli
            | Self::OpencodeCli => "JavaScript & AI CLIs",
            Self::Go => "Languages",
            Self::Flutter | Self::AndroidStudio | Self::ReactNativeCli => "Mobile / Cross-Platform",
        }
    }

    pub fn kind(&self) -> SoftwareKind {
        match self {
            Self::Brave
            | Self::Firefox
            | Self::Chrome
            | Self::JetBrainsToolbox
            | Self::Cursor
            | Self::Windsurf
            | Self::VisualStudioCode
            | Self::ZedStable
            | Self::ZedPreview
            | Self::Raycast
            | Self::AndroidStudio
            | Self::DockerDesktop
            | Self::Postman
            | Self::Yaak
            | Self::Bruno => SoftwareKind::GuiApplication,
            Self::Rustup
            | Self::RustStable
            | Self::RustNightly
            | Self::Go
            | Self::Flutter
            | Self::XcodeCommandLineTools => SoftwareKind::Sdk,
            _ => SoftwareKind::CliTool,
        }
    }

    pub fn dependencies(&self) -> &'static [SoftwareId] {
        match self {
            Self::Homebrew => &[],
            Self::XcodeCommandLineTools => &[],
            Self::Brave
            | Self::Firefox
            | Self::Chrome
            | Self::JetBrainsToolbox
            | Self::Cursor
            | Self::Windsurf
            | Self::VisualStudioCode
            | Self::ZedStable
            | Self::ZedPreview
            | Self::Raycast
            | Self::DockerDesktop
            | Self::Postman
            | Self::Yaak
            | Self::Bruno
            | Self::AndroidStudio => &[SoftwareId::Homebrew],
            Self::Rustup => &[],
            Self::RustStable | Self::RustNightly => &[SoftwareId::Rustup],
            Self::CargoJust
            | Self::CargoBinstall
            | Self::CargoWatch
            | Self::SimpleHttpServer
            | Self::DioxusCli
            | Self::YewCli
            | Self::LeptosCli => &[SoftwareId::Rustup],
            Self::Nvm => &[],
            Self::Bun => &[],
            Self::Go => &[SoftwareId::Homebrew],
            Self::Flutter => &[SoftwareId::Homebrew],
            Self::ReactNativeCli
            | Self::ElectronForge
            | Self::CodexCli
            | Self::ClaudeCli
            | Self::ClaudeMultiCli
            | Self::KimiCli
            | Self::GeminiCli
            | Self::QwenCli
            | Self::OpencodeCli => &[SoftwareId::Nvm],
        }
    }

    pub fn version_probe(&self) -> VersionProbe {
        match self {
            Self::Homebrew => VersionProbe::Command {
                program: "brew",
                args: &["--version"],
            },
            Self::XcodeCommandLineTools => VersionProbe::Command {
                program: "xcodebuild",
                args: &["-version"],
            },
            Self::Brave => VersionProbe::Mdls {
                path: "/Applications/Brave Browser.app",
            },
            Self::Firefox => VersionProbe::Mdls {
                path: "/Applications/Firefox.app",
            },
            Self::Chrome => VersionProbe::Mdls {
                path: "/Applications/Google Chrome.app",
            },
            Self::JetBrainsToolbox => VersionProbe::Mdls {
                path: "/Applications/JetBrains Toolbox.app",
            },
            Self::Cursor => VersionProbe::Mdls {
                path: "/Applications/Cursor.app",
            },
            Self::Windsurf => VersionProbe::Mdls {
                path: "/Applications/Windsurf.app",
            },
            Self::VisualStudioCode => VersionProbe::Mdls {
                path: "/Applications/Visual Studio Code.app",
            },
            Self::ZedStable => VersionProbe::Mdls {
                path: "/Applications/Zed.app",
            },
            Self::ZedPreview => VersionProbe::Mdls {
                path: "/Applications/Zed Preview.app",
            },
            Self::Raycast => VersionProbe::Mdls {
                path: "/Applications/Raycast.app",
            },
            Self::Rustup => VersionProbe::Command {
                program: "rustup",
                args: &["--version"],
            },
            Self::RustStable => VersionProbe::Command {
                program: "rustup",
                args: &["run", "stable", "rustc", "--version"],
            },
            Self::RustNightly => VersionProbe::Command {
                program: "rustup",
                args: &["run", "nightly", "rustc", "--version"],
            },
            Self::CargoJust => VersionProbe::Command {
                program: "just",
                args: &["--version"],
            },
            Self::CargoBinstall => VersionProbe::Command {
                program: "cargo-binstall",
                args: &["--version"],
            },
            Self::CargoWatch => VersionProbe::Command {
                program: "cargo-watch",
                args: &["--version"],
            },
            Self::SimpleHttpServer => VersionProbe::Command {
                program: "simple-http-server",
                args: &["--version"],
            },
            Self::Nvm => {
                VersionProbe::Manual("Run `nvm --version` after sourcing your shell profile.")
            }
            Self::Bun => VersionProbe::Command {
                program: "bun",
                args: &["--version"],
            },
            Self::Go => VersionProbe::BrewList { package: "go" },
            Self::Flutter => VersionProbe::BrewList { package: "flutter" },
            Self::AndroidStudio => VersionProbe::Mdls {
                path: "/Applications/Android Studio.app",
            },
            Self::ReactNativeCli => VersionProbe::Command {
                program: "react-native",
                args: &["--version"],
            },
            Self::DioxusCli => VersionProbe::Command {
                program: "dioxus",
                args: &["--version"],
            },
            Self::YewCli => VersionProbe::Command {
                program: "trunk",
                args: &["--version"],
            },
            Self::LeptosCli => VersionProbe::Command {
                program: "cargo-leptos",
                args: &["--version"],
            },
            Self::ElectronForge => VersionProbe::Command {
                program: "electron-forge",
                args: &["--version"],
            },
            Self::DockerDesktop => VersionProbe::Mdls {
                path: "/Applications/Docker.app",
            },
            Self::Postman => VersionProbe::Mdls {
                path: "/Applications/Postman.app",
            },
            Self::Yaak => VersionProbe::Mdls {
                path: "/Applications/Yaak.app",
            },
            Self::Bruno => VersionProbe::Mdls {
                path: "/Applications/Bruno.app",
            },
            Self::CodexCli => VersionProbe::Command {
                program: "codex",
                args: &["--version"],
            },
            Self::ClaudeCli => VersionProbe::Command {
                program: "claude",
                args: &["--version"],
            },
            Self::ClaudeMultiCli => VersionProbe::Command {
                program: "claude-multi",
                args: &["--version"],
            },
            Self::KimiCli => VersionProbe::Command {
                program: "kimi",
                args: &["--version"],
            },
            Self::GeminiCli => VersionProbe::Command {
                program: "gemini",
                args: &["--version"],
            },
            Self::QwenCli => VersionProbe::Command {
                program: "qwen",
                args: &["--version"],
            },
            Self::OpencodeCli => VersionProbe::Command {
                program: "opencode",
                args: &["--version"],
            },
        }
    }

    pub fn install_recipe(&self) -> CommandRecipe {
        match self {
            Self::Homebrew => CommandRecipe::Shell(
                "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"",
            ),
            Self::XcodeCommandLineTools => CommandRecipe::Shell("xcode-select --install"),
            Self::Brave => CommandRecipe::Shell("brew install --cask brave-browser"),
            Self::Firefox => CommandRecipe::Shell("brew install --cask firefox"),
            Self::Chrome => CommandRecipe::Shell("brew install --cask google-chrome"),
            Self::JetBrainsToolbox => CommandRecipe::Shell("brew install --cask jetbrains-toolbox"),
            Self::Cursor => CommandRecipe::Shell("brew install --cask cursor"),
            Self::Windsurf => CommandRecipe::Shell("brew install --cask windsurf"),
            Self::VisualStudioCode => {
                CommandRecipe::Shell("brew install --cask visual-studio-code")
            }
            Self::ZedStable => CommandRecipe::Shell("brew install --cask zed"),
            Self::ZedPreview => CommandRecipe::Shell("brew install --cask zed@preview"),
            Self::Raycast => CommandRecipe::Shell("brew install --cask raycast"),
            Self::Rustup => CommandRecipe::Shell(
                "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y",
            ),
            Self::RustStable => {
                CommandRecipe::Shell("rustup toolchain install stable && rustup default stable")
            }
            Self::RustNightly => CommandRecipe::Shell("rustup toolchain install nightly"),
            Self::CargoJust => CommandRecipe::Shell("cargo install just"),
            Self::CargoBinstall => CommandRecipe::Shell("cargo install cargo-binstall"),
            Self::CargoWatch => CommandRecipe::Shell("cargo install cargo-watch"),
            Self::SimpleHttpServer => {
                CommandRecipe::Shell("cargo +nightly install simple-http-server")
            }
            Self::Nvm => CommandRecipe::Shell(
                "curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash",
            ),
            Self::Bun => CommandRecipe::Shell("curl -fsSL https://bun.sh/install | bash"),
            Self::Go => CommandRecipe::Shell("brew install go"),
            Self::Flutter => CommandRecipe::Shell("brew install --cask flutter"),
            Self::AndroidStudio => CommandRecipe::Shell("brew install --cask android-studio"),
            Self::ReactNativeCli => CommandRecipe::Shell("npm install -g react-native-cli"),
            Self::DioxusCli => CommandRecipe::Shell("cargo install dioxus-cli"),
            Self::YewCli => CommandRecipe::Shell(
                "rustup target add wasm32-unknown-unknown && cargo install trunk",
            ),
            Self::LeptosCli => CommandRecipe::Shell(
                "rustup target add wasm32-unknown-unknown && cargo install cargo-leptos",
            ),
            Self::ElectronForge => CommandRecipe::Shell("npm install -g @electron-forge/cli"),
            Self::DockerDesktop => CommandRecipe::Shell("brew install --cask docker"),
            Self::Postman => CommandRecipe::Shell("brew install --cask postman"),
            Self::Yaak => CommandRecipe::Shell("brew install --cask yaak"),
            Self::Bruno => CommandRecipe::Shell("brew install --cask bruno"),
            Self::CodexCli => CommandRecipe::Shell("npm install -g codex-cli"),
            Self::ClaudeCli => CommandRecipe::Shell("npm install -g claude-cli"),
            Self::ClaudeMultiCli => CommandRecipe::Shell("npm install -g claude-multi-cli"),
            Self::KimiCli => CommandRecipe::Shell("npm install -g kimi-cli"),
            Self::GeminiCli => CommandRecipe::Shell("npm install -g gemini-cli"),
            Self::QwenCli => CommandRecipe::Shell("npm install -g qwen-cli"),
            Self::OpencodeCli => CommandRecipe::Shell("npm install -g opencode-cli"),
        }
    }

    pub fn update_recipe(&self) -> CommandRecipe {
        match self {
            Self::Homebrew => CommandRecipe::Shell("brew update && brew upgrade"),
            Self::XcodeCommandLineTools => CommandRecipe::Manual(
                "Use Software Update or run `softwareupdate --all --install --force`.",
            ),
            Self::Brave => CommandRecipe::Shell("brew upgrade --cask brave-browser"),
            Self::Firefox => CommandRecipe::Shell("brew upgrade --cask firefox"),
            Self::Chrome => CommandRecipe::Shell("brew upgrade --cask google-chrome"),
            Self::JetBrainsToolbox => CommandRecipe::Shell("brew upgrade --cask jetbrains-toolbox"),
            Self::Cursor => CommandRecipe::Shell("brew upgrade --cask cursor"),
            Self::Windsurf => CommandRecipe::Shell("brew upgrade --cask windsurf"),
            Self::VisualStudioCode => {
                CommandRecipe::Shell("brew upgrade --cask visual-studio-code")
            }
            Self::ZedStable => CommandRecipe::Shell("brew upgrade --cask zed"),
            Self::ZedPreview => CommandRecipe::Shell("brew upgrade --cask zed@preview"),
            Self::Raycast => CommandRecipe::Shell("brew upgrade --cask raycast"),
            Self::Rustup => CommandRecipe::Shell("rustup self update && rustup update"),
            Self::RustStable => CommandRecipe::Shell("rustup update stable"),
            Self::RustNightly => CommandRecipe::Shell("rustup update nightly"),
            Self::CargoJust => CommandRecipe::Shell("cargo install just --force"),
            Self::CargoBinstall => CommandRecipe::Shell("cargo install cargo-binstall --force"),
            Self::CargoWatch => CommandRecipe::Shell("cargo install cargo-watch --force"),
            Self::SimpleHttpServer => {
                CommandRecipe::Shell("cargo +nightly install simple-http-server --force")
            }
            Self::Nvm => {
                CommandRecipe::Manual("Pull the latest nvm via git or rerun the installer script.")
            }
            Self::Bun => CommandRecipe::Shell("bun upgrade"),
            Self::Go => CommandRecipe::Shell("brew upgrade go"),
            Self::Flutter => CommandRecipe::Shell("flutter upgrade"),
            Self::AndroidStudio => CommandRecipe::Shell("brew upgrade --cask android-studio"),
            Self::ReactNativeCli => CommandRecipe::Shell("npm update -g react-native-cli"),
            Self::DioxusCli => CommandRecipe::Shell("cargo install dioxus-cli --force"),
            Self::YewCli => CommandRecipe::Shell("cargo install trunk --force"),
            Self::LeptosCli => CommandRecipe::Shell("cargo install cargo-leptos --force"),
            Self::ElectronForge => CommandRecipe::Shell("npm update -g @electron-forge/cli"),
            Self::DockerDesktop => CommandRecipe::Shell("brew upgrade --cask docker"),
            Self::Postman => CommandRecipe::Shell("brew upgrade --cask postman"),
            Self::Yaak => CommandRecipe::Shell("brew upgrade --cask yaak"),
            Self::Bruno => CommandRecipe::Shell("brew upgrade --cask bruno"),
            Self::CodexCli => CommandRecipe::Shell("npm update -g codex-cli"),
            Self::ClaudeCli => CommandRecipe::Shell("npm update -g claude-cli"),
            Self::ClaudeMultiCli => CommandRecipe::Shell("npm update -g claude-multi-cli"),
            Self::KimiCli => CommandRecipe::Shell("npm update -g kimi-cli"),
            Self::GeminiCli => CommandRecipe::Shell("npm update -g gemini-cli"),
            Self::QwenCli => CommandRecipe::Shell("npm update -g qwen-cli"),
            Self::OpencodeCli => CommandRecipe::Shell("npm update -g opencode-cli"),
        }
    }

    pub fn uninstall_recipe(&self) -> CommandRecipe {
        match self {
            Self::Homebrew => {
                CommandRecipe::Manual("Follow https://docs.brew.sh/FAQ#how-do-i-uninstall-homebrew")
            }
            Self::XcodeCommandLineTools => {
                CommandRecipe::Shell("sudo rm -rf /Library/Developer/CommandLineTools")
            }
            Self::Brave => CommandRecipe::Shell("brew uninstall --cask brave-browser"),
            Self::Firefox => CommandRecipe::Shell("brew uninstall --cask firefox"),
            Self::Chrome => CommandRecipe::Shell("brew uninstall --cask google-chrome"),
            Self::JetBrainsToolbox => {
                CommandRecipe::Shell("brew uninstall --cask jetbrains-toolbox")
            }
            Self::Cursor => CommandRecipe::Shell("brew uninstall --cask cursor"),
            Self::Windsurf => CommandRecipe::Shell("brew uninstall --cask windsurf"),
            Self::VisualStudioCode => {
                CommandRecipe::Shell("brew uninstall --cask visual-studio-code")
            }
            Self::ZedStable => CommandRecipe::Shell("brew uninstall --cask zed"),
            Self::ZedPreview => CommandRecipe::Shell("brew uninstall --cask zed@preview"),
            Self::Raycast => CommandRecipe::Shell("brew uninstall --cask raycast"),
            Self::Rustup => CommandRecipe::Shell("rustup self uninstall -y"),
            Self::RustStable => CommandRecipe::Shell("rustup toolchain uninstall stable"),
            Self::RustNightly => CommandRecipe::Shell("rustup toolchain uninstall nightly"),
            Self::CargoJust => CommandRecipe::Shell("cargo uninstall just"),
            Self::CargoBinstall => CommandRecipe::Shell("cargo uninstall cargo-binstall"),
            Self::CargoWatch => CommandRecipe::Shell("cargo uninstall cargo-watch"),
            Self::SimpleHttpServer => CommandRecipe::Shell("cargo uninstall simple-http-server"),
            Self::Nvm => CommandRecipe::Manual("Remove ~/.nvm and related shell profile entries."),
            Self::Bun => CommandRecipe::Manual("Remove ~/.bun and PATH exports."),
            Self::Go => CommandRecipe::Shell("brew uninstall go"),
            Self::Flutter => CommandRecipe::Shell("brew uninstall --cask flutter"),
            Self::AndroidStudio => CommandRecipe::Shell("brew uninstall --cask android-studio"),
            Self::ReactNativeCli => CommandRecipe::Shell("npm uninstall -g react-native-cli"),
            Self::DioxusCli => CommandRecipe::Shell("cargo uninstall dioxus-cli"),
            Self::YewCli => CommandRecipe::Shell("cargo uninstall trunk"),
            Self::LeptosCli => CommandRecipe::Shell("cargo uninstall cargo-leptos"),
            Self::ElectronForge => CommandRecipe::Shell("npm uninstall -g @electron-forge/cli"),
            Self::DockerDesktop => CommandRecipe::Shell("brew uninstall --cask docker"),
            Self::Postman => CommandRecipe::Shell("brew uninstall --cask postman"),
            Self::Yaak => CommandRecipe::Shell("brew uninstall --cask yaak"),
            Self::Bruno => CommandRecipe::Shell("brew uninstall --cask bruno"),
            Self::CodexCli => CommandRecipe::Shell("npm uninstall -g codex-cli"),
            Self::ClaudeCli => CommandRecipe::Shell("npm uninstall -g claude-cli"),
            Self::ClaudeMultiCli => CommandRecipe::Shell("npm uninstall -g claude-multi-cli"),
            Self::KimiCli => CommandRecipe::Shell("npm uninstall -g kimi-cli"),
            Self::GeminiCli => CommandRecipe::Shell("npm uninstall -g gemini-cli"),
            Self::QwenCli => CommandRecipe::Shell("npm uninstall -g qwen-cli"),
            Self::OpencodeCli => CommandRecipe::Shell("npm uninstall -g opencode-cli"),
        }
    }
}

pub fn all_entries() -> Vec<SoftwareEntry> {
    SoftwareId::all()
        .iter()
        .copied()
        .map(SoftwareEntry::from_id)
        .collect()
}

pub fn find_by_key(key: &str) -> Option<SoftwareEntry> {
    SoftwareId::from_key(key).map(SoftwareEntry::from_id)
}

pub fn entry(id: SoftwareId) -> SoftwareEntry {
    SoftwareEntry::from_id(id)
}
