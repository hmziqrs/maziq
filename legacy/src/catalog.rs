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
    ITerm2,
    Warp,
    Alacritty,
    Fish,
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
    Pyenv,
    Poetry,
    Uv,
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
    TablePlus,
    DBeaver,
    MongoDbCompass,
    RedisInsight,
    Slack,
    Discord,
    Zoom,
    Figma,
    Vlc,
    Kubectl,
    Terraform,
    AwsCli,
    AzureCli,
    Jq,
    Ripgrep,
    Fd,
    Bat,
    Fzf,
    Btop,
    Neovim,
    CodexCli,
    ClaudeCli,
    ClaudeMultiCli,
    KimiCli,
    GeminiCli,
    QwenCli,
    OpencodeCli,
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
    SoftwareId::ITerm2,
    SoftwareId::Warp,
    SoftwareId::Alacritty,
    SoftwareId::Fish,
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
    SoftwareId::Pyenv,
    SoftwareId::Poetry,
    SoftwareId::Uv,
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
    SoftwareId::TablePlus,
    SoftwareId::DBeaver,
    SoftwareId::MongoDbCompass,
    SoftwareId::RedisInsight,
    SoftwareId::Slack,
    SoftwareId::Discord,
    SoftwareId::Zoom,
    SoftwareId::Figma,
    SoftwareId::Vlc,
    SoftwareId::Kubectl,
    SoftwareId::Terraform,
    SoftwareId::AwsCli,
    SoftwareId::AzureCli,
    SoftwareId::Jq,
    SoftwareId::Ripgrep,
    SoftwareId::Fd,
    SoftwareId::Bat,
    SoftwareId::Fzf,
    SoftwareId::Btop,
    SoftwareId::Neovim,
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

    pub fn default_label(&self) -> &'static str {
        match self {
            CommandRecipe::Shell(cmd) => infer_label(cmd),
            CommandRecipe::Manual(_) => "manual",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CommandSource {
    pub label: &'static str,
    pub recipe: CommandRecipe,
}

impl CommandSource {
    pub fn from_recipe(recipe: CommandRecipe) -> Self {
        Self {
            label: recipe.default_label(),
            recipe,
        }
    }

    pub const fn shell(label: &'static str, command: &'static str) -> Self {
        Self {
            label,
            recipe: CommandRecipe::Shell(command),
        }
    }

    pub const fn manual(label: &'static str, note: &'static str) -> Self {
        Self {
            label,
            recipe: CommandRecipe::Manual(note),
        }
    }
}

fn infer_label(cmd: &str) -> &'static str {
    if cmd.contains("brew install --cask") {
        "homebrew cask"
    } else if cmd.starts_with("brew install") {
        "homebrew formula"
    } else if cmd.contains("brew upgrade --cask") {
        "homebrew cask upgrade"
    } else if cmd.starts_with("brew upgrade") {
        "homebrew upgrade"
    } else if cmd.starts_with("npm install -g") {
        "npm global"
    } else if cmd.starts_with("npm update -g") {
        "npm global update"
    } else if cmd.starts_with("cargo install") {
        "cargo install"
    } else if cmd.starts_with("cargo uninstall") {
        "cargo uninstall"
    } else if cmd.starts_with("curl --proto") || cmd.starts_with("curl -fsSL") {
        "curl script"
    } else if cmd.starts_with("xcode-select") {
        "xcode-select"
    } else if cmd.starts_with("rustup") {
        "rustup"
    } else if cmd.starts_with("go ") {
        "go"
    } else if cmd.starts_with("flutter") {
        "flutter"
    } else {
        "shell"
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

    pub fn install_sources(&self) -> Vec<CommandSource> {
        build_sources(self.install, self.id.extra_install_sources())
    }

    pub fn update_sources(&self) -> Vec<CommandSource> {
        build_sources(self.update, self.id.extra_update_sources())
    }

    pub fn uninstall_sources(&self) -> Vec<CommandSource> {
        build_sources(self.uninstall, self.id.extra_uninstall_sources())
    }
}

fn build_sources(primary: CommandRecipe, extras: &'static [CommandRecipe]) -> Vec<CommandSource> {
    let mut sources = Vec::with_capacity(1 + extras.len());
    sources.push(CommandSource::from_recipe(primary));
    for recipe in extras {
        sources.push(CommandSource::from_recipe(*recipe));
    }
    sources
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
            Self::ITerm2 => "iterm2",
            Self::Warp => "warp",
            Self::Alacritty => "alacritty",
            Self::Fish => "fish",
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
            Self::Pyenv => "pyenv",
            Self::Poetry => "poetry",
            Self::Uv => "uv",
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
            Self::TablePlus => "tableplus",
            Self::DBeaver => "dbeaver",
            Self::MongoDbCompass => "mongodb_compass",
            Self::RedisInsight => "redis_insight",
            Self::Slack => "slack",
            Self::Discord => "discord",
            Self::Zoom => "zoom",
            Self::Figma => "figma",
            Self::Vlc => "vlc",
            Self::Kubectl => "kubectl",
            Self::Terraform => "terraform",
            Self::AwsCli => "aws_cli",
            Self::AzureCli => "azure_cli",
            Self::Jq => "jq",
            Self::Ripgrep => "ripgrep",
            Self::Fd => "fd",
            Self::Bat => "bat",
            Self::Fzf => "fzf",
            Self::Btop => "btop",
            Self::Neovim => "neovim",
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
            Self::ITerm2 => "iTerm2",
            Self::Warp => "Warp",
            Self::Alacritty => "Alacritty",
            Self::Fish => "Fish Shell",
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
            Self::Pyenv => "pyenv",
            Self::Poetry => "Poetry",
            Self::Uv => "uv",
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
            Self::TablePlus => "TablePlus",
            Self::DBeaver => "DBeaver",
            Self::MongoDbCompass => "MongoDB Compass",
            Self::RedisInsight => "RedisInsight",
            Self::Slack => "Slack",
            Self::Discord => "Discord",
            Self::Zoom => "Zoom",
            Self::Figma => "Figma",
            Self::Vlc => "VLC Media Player",
            Self::Kubectl => "kubectl",
            Self::Terraform => "Terraform",
            Self::AwsCli => "AWS CLI",
            Self::AzureCli => "Azure CLI",
            Self::Jq => "jq",
            Self::Ripgrep => "ripgrep",
            Self::Fd => "fd",
            Self::Bat => "bat",
            Self::Fzf => "fzf",
            Self::Btop => "btop",
            Self::Neovim => "Neovim",
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
            Self::Firefox => "Mozilla's versatile browser.",
            Self::Chrome => "Google's mainstream browser.",
            Self::JetBrainsToolbox => "Launcher for JetBrains IDEs (including Android Studio).",
            Self::Cursor => "AI-native editor focused on flow with built-in agent.",
            Self::Windsurf => "Codeium's AI pair-programming IDE.",
            Self::VisualStudioCode => "Microsoft's extensible editor and IDE.",
            Self::ZedStable => "Zed editor stable channel.",
            Self::ZedPreview => "Zed editor preview channel.",
            Self::Raycast => "Productivity command palette for macOS.",
            Self::ITerm2 => "Popular macOS terminal replacement with advanced features.",
            Self::Warp => "Modern terminal with AI features and collaborative editing.",
            Self::Alacritty => "GPU-accelerated terminal emulator.",
            Self::Fish => "User-friendly shell with autosuggestions and syntax highlighting.",
            Self::Rustup => "Rust toolchain installer and updater.",
            Self::RustStable => "Sets Rust stable toolchain as default.",
            Self::RustNightly => "Installs nightly Rust toolchain.",
            Self::CargoJust => "Handy task runner for Rust projects.",
            Self::CargoBinstall => "Fast binary installer for cargo packages.",
            Self::CargoWatch => "Auto compile/test watcher for Rust.",
            Self::SimpleHttpServer => "Nightly-only HTTP dev server.",
            Self::Nvm => "Manage multiple Node.js versions.",
            Self::Bun => "All-in-one JS runtime/bundler.",
            Self::Go => "Google's Go language toolchain.",
            Self::Pyenv => "Python version manager for switching between Python versions.",
            Self::Poetry => "Python dependency management and packaging tool.",
            Self::Uv => "Extremely fast Python package installer and resolver.",
            Self::Flutter => "Google's UI SDK for mobile & desktop.",
            Self::AndroidStudio => "Google's official IDE for Android development.",
            Self::ReactNativeCli => "React Native project scaffolding CLI.",
            Self::DioxusCli => "Rust-based cross-platform UI tooling.",
            Self::YewCli => "CLI helpers for Yew web apps.",
            Self::LeptosCli => "Full-stack Leptos project manager.",
            Self::ElectronForge => "Electron scaffolding, packaging, and release tooling.",
            Self::DockerDesktop => "GUI and runtime for local Docker containers.",
            Self::Postman => "API design, testing, and collaboration suite.",
            Self::Yaak => "Lightweight REST and GraphQL API client.",
            Self::Bruno => "Text-based API collections and testing tool.",
            Self::TablePlus => "Modern database GUI for MySQL, PostgreSQL, SQLite, and more.",
            Self::DBeaver => "Free universal database tool and SQL client.",
            Self::MongoDbCompass => "GUI for MongoDB with visual query builder.",
            Self::RedisInsight => "Redis GUI with browser and CLI.",
            Self::Slack => "Team communication and collaboration platform.",
            Self::Discord => "Voice, video, and text communication platform.",
            Self::Zoom => "Video conferencing and online meetings.",
            Self::Figma => "Collaborative interface design tool.",
            Self::Vlc => "Open-source multimedia player.",
            Self::Kubectl => "Kubernetes command-line tool.",
            Self::Terraform => "Infrastructure as code tool for cloud provisioning.",
            Self::AwsCli => "Amazon Web Services command-line interface.",
            Self::AzureCli => "Microsoft Azure command-line interface.",
            Self::Jq => "Lightweight command-line JSON processor.",
            Self::Ripgrep => "Extremely fast grep alternative.",
            Self::Fd => "Simple, fast alternative to find.",
            Self::Bat => "Cat clone with syntax highlighting and Git integration.",
            Self::Fzf => "General-purpose command-line fuzzy finder.",
            Self::Btop => "Resource monitor with modern interface.",
            Self::Neovim => "Hyperextensible Vim-based text editor.",
            Self::CodexCli => "Command-line interface for Codex-style AI coding.",
            Self::ClaudeCli => "Anthropic Claude assistant from the terminal.",
            Self::ClaudeMultiCli => "Manage multiple Claude sessions and keys via CLI.",
            Self::KimiCli => "Terminal gateway to Kimi AI assistant.",
            Self::GeminiCli => "Interact with Google Gemini models from the CLI.",
            Self::QwenCli => "CLI helper for Alibaba Cloud's Qwen assistants.",
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
            Self::ITerm2 | Self::Warp | Self::Alacritty | Self::Fish => "Terminal & Shell",
            Self::Raycast | Self::DockerDesktop => "Desktop Utilities",
            Self::Slack | Self::Discord | Self::Zoom => "Communication",
            Self::Figma | Self::Vlc => "Design & Media",
            Self::Postman | Self::Yaak | Self::Bruno => "API & Testing",
            Self::TablePlus | Self::DBeaver | Self::MongoDbCompass | Self::RedisInsight => "Database Tools",
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
            Self::Pyenv | Self::Poetry | Self::Uv => "Python Stack",
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
            Self::Kubectl | Self::Terraform | Self::AwsCli | Self::AzureCli => "DevOps & Cloud",
            Self::Jq | Self::Ripgrep | Self::Fd | Self::Bat | Self::Fzf | Self::Btop | Self::Neovim => "CLI Utilities",
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
            | Self::ITerm2
            | Self::Warp
            | Self::Alacritty
            | Self::AndroidStudio
            | Self::DockerDesktop
            | Self::Postman
            | Self::Yaak
            | Self::Bruno
            | Self::TablePlus
            | Self::DBeaver
            | Self::MongoDbCompass
            | Self::RedisInsight
            | Self::Slack
            | Self::Discord
            | Self::Zoom
            | Self::Figma
            | Self::Vlc => SoftwareKind::GuiApplication,
            Self::Rustup
            | Self::RustStable
            | Self::RustNightly
            | Self::Go
            | Self::Pyenv
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
            | Self::ITerm2
            | Self::Warp
            | Self::Alacritty
            | Self::Fish
            | Self::DockerDesktop
            | Self::Postman
            | Self::Yaak
            | Self::Bruno
            | Self::TablePlus
            | Self::DBeaver
            | Self::MongoDbCompass
            | Self::RedisInsight
            | Self::Slack
            | Self::Discord
            | Self::Zoom
            | Self::Figma
            | Self::Vlc
            | Self::AndroidStudio
            | Self::Kubectl
            | Self::Terraform
            | Self::AwsCli
            | Self::AzureCli
            | Self::Jq
            | Self::Ripgrep
            | Self::Fd
            | Self::Bat
            | Self::Fzf
            | Self::Btop
            | Self::Neovim => &[SoftwareId::Homebrew],
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
            Self::Pyenv => &[],
            Self::Poetry | Self::Uv => &[SoftwareId::Pyenv],
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
            Self::ITerm2 => VersionProbe::Mdls {
                path: "/Applications/iTerm.app",
            },
            Self::Warp => VersionProbe::Mdls {
                path: "/Applications/Warp.app",
            },
            Self::Alacritty => VersionProbe::Mdls {
                path: "/Applications/Alacritty.app",
            },
            Self::Fish => VersionProbe::Command {
                program: "fish",
                args: &["--version"],
            },
            Self::TablePlus => VersionProbe::Mdls {
                path: "/Applications/TablePlus.app",
            },
            Self::DBeaver => VersionProbe::Mdls {
                path: "/Applications/DBeaver.app",
            },
            Self::MongoDbCompass => VersionProbe::Mdls {
                path: "/Applications/MongoDB Compass.app",
            },
            Self::RedisInsight => VersionProbe::Mdls {
                path: "/Applications/RedisInsight.app",
            },
            Self::Slack => VersionProbe::Mdls {
                path: "/Applications/Slack.app",
            },
            Self::Discord => VersionProbe::Mdls {
                path: "/Applications/Discord.app",
            },
            Self::Zoom => VersionProbe::Mdls {
                path: "/Applications/zoom.us.app",
            },
            Self::Figma => VersionProbe::Mdls {
                path: "/Applications/Figma.app",
            },
            Self::Vlc => VersionProbe::Mdls {
                path: "/Applications/VLC.app",
            },
            Self::Pyenv => VersionProbe::Command {
                program: "pyenv",
                args: &["--version"],
            },
            Self::Poetry => VersionProbe::Command {
                program: "poetry",
                args: &["--version"],
            },
            Self::Uv => VersionProbe::Command {
                program: "uv",
                args: &["--version"],
            },
            Self::Kubectl => VersionProbe::Command {
                program: "kubectl",
                args: &["version", "--client", "--short"],
            },
            Self::Terraform => VersionProbe::Command {
                program: "terraform",
                args: &["--version"],
            },
            Self::AwsCli => VersionProbe::Command {
                program: "aws",
                args: &["--version"],
            },
            Self::AzureCli => VersionProbe::Command {
                program: "az",
                args: &["--version"],
            },
            Self::Jq => VersionProbe::Command {
                program: "jq",
                args: &["--version"],
            },
            Self::Ripgrep => VersionProbe::Command {
                program: "rg",
                args: &["--version"],
            },
            Self::Fd => VersionProbe::Command {
                program: "fd",
                args: &["--version"],
            },
            Self::Bat => VersionProbe::Command {
                program: "bat",
                args: &["--version"],
            },
            Self::Fzf => VersionProbe::Command {
                program: "fzf",
                args: &["--version"],
            },
            Self::Btop => VersionProbe::Command {
                program: "btop",
                args: &["--version"],
            },
            Self::Neovim => VersionProbe::Command {
                program: "nvim",
                args: &["--version"],
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
            Self::ITerm2 => CommandRecipe::Shell("brew install --cask iterm2"),
            Self::Warp => CommandRecipe::Shell("brew install --cask warp"),
            Self::Alacritty => CommandRecipe::Shell("brew install --cask alacritty"),
            Self::Fish => CommandRecipe::Shell("brew install fish"),
            Self::TablePlus => CommandRecipe::Shell("brew install --cask tableplus"),
            Self::DBeaver => CommandRecipe::Shell("brew install --cask dbeaver-community"),
            Self::MongoDbCompass => CommandRecipe::Shell("brew install --cask mongodb-compass"),
            Self::RedisInsight => CommandRecipe::Shell("brew install --cask redisinsight"),
            Self::Slack => CommandRecipe::Shell("brew install --cask slack"),
            Self::Discord => CommandRecipe::Shell("brew install --cask discord"),
            Self::Zoom => CommandRecipe::Shell("brew install --cask zoom"),
            Self::Figma => CommandRecipe::Shell("brew install --cask figma"),
            Self::Vlc => CommandRecipe::Shell("brew install --cask vlc"),
            Self::Pyenv => CommandRecipe::Shell("brew install pyenv"),
            Self::Poetry => CommandRecipe::Shell("curl -sSL https://install.python-poetry.org | python3 -"),
            Self::Uv => CommandRecipe::Shell("curl -LsSf https://astral.sh/uv/install.sh | sh"),
            Self::Kubectl => CommandRecipe::Shell("brew install kubectl"),
            Self::Terraform => CommandRecipe::Shell("brew install terraform"),
            Self::AwsCli => CommandRecipe::Shell("brew install awscli"),
            Self::AzureCli => CommandRecipe::Shell("brew install azure-cli"),
            Self::Jq => CommandRecipe::Shell("brew install jq"),
            Self::Ripgrep => CommandRecipe::Shell("brew install ripgrep"),
            Self::Fd => CommandRecipe::Shell("brew install fd"),
            Self::Bat => CommandRecipe::Shell("brew install bat"),
            Self::Fzf => CommandRecipe::Shell("brew install fzf"),
            Self::Btop => CommandRecipe::Shell("brew install btop"),
            Self::Neovim => CommandRecipe::Shell("brew install neovim"),
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
            Self::ITerm2 => CommandRecipe::Shell("brew upgrade --cask iterm2"),
            Self::Warp => CommandRecipe::Shell("brew upgrade --cask warp"),
            Self::Alacritty => CommandRecipe::Shell("brew upgrade --cask alacritty"),
            Self::Fish => CommandRecipe::Shell("brew upgrade fish"),
            Self::TablePlus => CommandRecipe::Shell("brew upgrade --cask tableplus"),
            Self::DBeaver => CommandRecipe::Shell("brew upgrade --cask dbeaver-community"),
            Self::MongoDbCompass => CommandRecipe::Shell("brew upgrade --cask mongodb-compass"),
            Self::RedisInsight => CommandRecipe::Shell("brew upgrade --cask redisinsight"),
            Self::Slack => CommandRecipe::Shell("brew upgrade --cask slack"),
            Self::Discord => CommandRecipe::Shell("brew upgrade --cask discord"),
            Self::Zoom => CommandRecipe::Shell("brew upgrade --cask zoom"),
            Self::Figma => CommandRecipe::Shell("brew upgrade --cask figma"),
            Self::Vlc => CommandRecipe::Shell("brew upgrade --cask vlc"),
            Self::Pyenv => CommandRecipe::Shell("brew upgrade pyenv"),
            Self::Poetry => CommandRecipe::Shell("poetry self update"),
            Self::Uv => CommandRecipe::Shell("curl -LsSf https://astral.sh/uv/install.sh | sh"),
            Self::Kubectl => CommandRecipe::Shell("brew upgrade kubectl"),
            Self::Terraform => CommandRecipe::Shell("brew upgrade terraform"),
            Self::AwsCli => CommandRecipe::Shell("brew upgrade awscli"),
            Self::AzureCli => CommandRecipe::Shell("brew upgrade azure-cli"),
            Self::Jq => CommandRecipe::Shell("brew upgrade jq"),
            Self::Ripgrep => CommandRecipe::Shell("brew upgrade ripgrep"),
            Self::Fd => CommandRecipe::Shell("brew upgrade fd"),
            Self::Bat => CommandRecipe::Shell("brew upgrade bat"),
            Self::Fzf => CommandRecipe::Shell("brew upgrade fzf"),
            Self::Btop => CommandRecipe::Shell("brew upgrade btop"),
            Self::Neovim => CommandRecipe::Shell("brew upgrade neovim"),
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
            Self::ITerm2 => CommandRecipe::Shell("brew uninstall --cask iterm2"),
            Self::Warp => CommandRecipe::Shell("brew uninstall --cask warp"),
            Self::Alacritty => CommandRecipe::Shell("brew uninstall --cask alacritty"),
            Self::Fish => CommandRecipe::Shell("brew uninstall fish"),
            Self::TablePlus => CommandRecipe::Shell("brew uninstall --cask tableplus"),
            Self::DBeaver => CommandRecipe::Shell("brew uninstall --cask dbeaver-community"),
            Self::MongoDbCompass => CommandRecipe::Shell("brew uninstall --cask mongodb-compass"),
            Self::RedisInsight => CommandRecipe::Shell("brew uninstall --cask redisinsight"),
            Self::Slack => CommandRecipe::Shell("brew uninstall --cask slack"),
            Self::Discord => CommandRecipe::Shell("brew uninstall --cask discord"),
            Self::Zoom => CommandRecipe::Shell("brew uninstall --cask zoom"),
            Self::Figma => CommandRecipe::Shell("brew uninstall --cask figma"),
            Self::Vlc => CommandRecipe::Shell("brew uninstall --cask vlc"),
            Self::Pyenv => CommandRecipe::Shell("brew uninstall pyenv"),
            Self::Poetry => CommandRecipe::Manual("Run: curl -sSL https://install.python-poetry.org | python3 - --uninstall"),
            Self::Uv => CommandRecipe::Manual("Remove ~/.cargo/bin/uv and related files."),
            Self::Kubectl => CommandRecipe::Shell("brew uninstall kubectl"),
            Self::Terraform => CommandRecipe::Shell("brew uninstall terraform"),
            Self::AwsCli => CommandRecipe::Shell("brew uninstall awscli"),
            Self::AzureCli => CommandRecipe::Shell("brew uninstall azure-cli"),
            Self::Jq => CommandRecipe::Shell("brew uninstall jq"),
            Self::Ripgrep => CommandRecipe::Shell("brew uninstall ripgrep"),
            Self::Fd => CommandRecipe::Shell("brew uninstall fd"),
            Self::Bat => CommandRecipe::Shell("brew uninstall bat"),
            Self::Fzf => CommandRecipe::Shell("brew uninstall fzf"),
            Self::Btop => CommandRecipe::Shell("brew uninstall btop"),
            Self::Neovim => CommandRecipe::Shell("brew uninstall neovim"),
            Self::CodexCli => CommandRecipe::Shell("npm uninstall -g codex-cli"),
            Self::ClaudeCli => CommandRecipe::Shell("npm uninstall -g claude-cli"),
            Self::ClaudeMultiCli => CommandRecipe::Shell("npm uninstall -g claude-multi-cli"),
            Self::KimiCli => CommandRecipe::Shell("npm uninstall -g kimi-cli"),
            Self::GeminiCli => CommandRecipe::Shell("npm uninstall -g gemini-cli"),
            Self::QwenCli => CommandRecipe::Shell("npm uninstall -g qwen-cli"),
            Self::OpencodeCli => CommandRecipe::Shell("npm uninstall -g opencode-cli"),
        }
    }

    pub fn extra_install_sources(&self) -> &'static [CommandRecipe] {
        match self {
            Self::AndroidStudio => &[CommandRecipe::Manual(
                "Download Android Studio dmg from developer.android.com and move it to /Applications.",
            )],
            Self::VisualStudioCode => &[CommandRecipe::Manual(
                "Download VS Code from code.visualstudio.com and drag it into /Applications.",
            )],
            Self::Cursor => &[CommandRecipe::Manual(
                "Download Cursor from cursor.sh and drag it into /Applications.",
            )],
            Self::Flutter => &[CommandRecipe::Manual(
                "Download Flutter SDK archive from flutter.dev/docs/get-started/install and add it manually.",
            )],
            _ => &[],
        }
    }

    pub fn extra_update_sources(&self) -> &'static [CommandRecipe] {
        &[]
    }

    pub fn extra_uninstall_sources(&self) -> &'static [CommandRecipe] {
        &[]
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
        name: "Editors & IDEs",
        entries: &[
            SoftwareId::JetBrainsToolbox,
            SoftwareId::Cursor,
            SoftwareId::Windsurf,
            SoftwareId::VisualStudioCode,
            SoftwareId::ZedStable,
            SoftwareId::ZedPreview,
        ],
    },
    SoftwareCategory {
        name: "Terminal & Shell",
        entries: &[SoftwareId::ITerm2, SoftwareId::Warp, SoftwareId::Alacritty, SoftwareId::Fish],
    },
    SoftwareCategory {
        name: "Desktop Utilities",
        entries: &[SoftwareId::Raycast, SoftwareId::DockerDesktop],
    },
    SoftwareCategory {
        name: "Communication",
        entries: &[SoftwareId::Slack, SoftwareId::Discord, SoftwareId::Zoom],
    },
    SoftwareCategory {
        name: "Design & Media",
        entries: &[SoftwareId::Figma, SoftwareId::Vlc],
    },
    SoftwareCategory {
        name: "API & Testing",
        entries: &[SoftwareId::Postman, SoftwareId::Yaak, SoftwareId::Bruno],
    },
    SoftwareCategory {
        name: "Database Tools",
        entries: &[SoftwareId::TablePlus, SoftwareId::DBeaver, SoftwareId::MongoDbCompass, SoftwareId::RedisInsight],
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
            SoftwareId::YewCli,
            SoftwareId::LeptosCli,
        ],
    },
    SoftwareCategory {
        name: "Python Stack",
        entries: &[SoftwareId::Pyenv, SoftwareId::Poetry, SoftwareId::Uv],
    },
    SoftwareCategory {
        name: "JavaScript & Node",
        entries: &[SoftwareId::Nvm, SoftwareId::Bun, SoftwareId::ElectronForge],
    },
    SoftwareCategory {
        name: "DevOps & Cloud",
        entries: &[SoftwareId::Kubectl, SoftwareId::Terraform, SoftwareId::AwsCli, SoftwareId::AzureCli],
    },
    SoftwareCategory {
        name: "Mobile / Cross-Platform",
        entries: &[
            SoftwareId::Flutter,
            SoftwareId::AndroidStudio,
            SoftwareId::ReactNativeCli,
        ],
    },
    SoftwareCategory {
        name: "CLI Utilities",
        entries: &[SoftwareId::Jq, SoftwareId::Ripgrep, SoftwareId::Fd, SoftwareId::Bat, SoftwareId::Fzf, SoftwareId::Btop, SoftwareId::Neovim],
    },
    SoftwareCategory {
        name: "AI Assistant CLIs",
        entries: &[
            SoftwareId::CodexCli,
            SoftwareId::ClaudeCli,
            SoftwareId::ClaudeMultiCli,
            SoftwareId::KimiCli,
            SoftwareId::GeminiCli,
            SoftwareId::QwenCli,
            SoftwareId::OpencodeCli,
        ],
    },
    SoftwareCategory {
        name: "Languages",
        entries: &[SoftwareId::Go],
    },
];
