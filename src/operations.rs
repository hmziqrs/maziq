use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    process::{Command, Stdio},
};

use crate::catalog::SoftwareId;

pub fn check_installed(id: SoftwareId) -> bool {
    match id {
        SoftwareId::Homebrew => check_command("brew"),
        SoftwareId::XcodeCommandLineTools => check_command("gcc"),
        SoftwareId::Brave => check_gui_app("Brave Browser"),
        SoftwareId::Firefox => check_gui_app("Firefox"),
        SoftwareId::Chrome => check_gui_app("Google Chrome"),
        SoftwareId::JetBrainsToolbox => check_gui_app("JetBrains Toolbox"),
        SoftwareId::ZedStable => check_gui_app("Zed"),
        SoftwareId::ZedPreview => check_gui_app("Zed Preview"),
        SoftwareId::Raycast => check_gui_app("Raycast"),
        SoftwareId::Rustup => check_command("rustup"),
        SoftwareId::RustStable => {
            check_command("rustc")
                && Command::new("rustup")
                    .arg("default")
                    .output()
                    .map_or(false, |output| {
                        String::from_utf8_lossy(&output.stdout)
                            .to_lowercase()
                            .contains("stable")
                    })
        }
        SoftwareId::RustNightly => Command::new("rustup")
            .arg("toolchain")
            .arg("list")
            .output()
            .map_or(false, |output| {
                String::from_utf8_lossy(&output.stdout)
                    .to_lowercase()
                    .contains("nightly")
            }),
        SoftwareId::CargoJust => check_command("just"),
        SoftwareId::CargoBinstall => check_command("cargo-binstall"),
        SoftwareId::CargoWatch => check_command("cargo-watch"),
        SoftwareId::SimpleHttpServer => check_command("simple-http-server"),
        SoftwareId::Nvm => env::var("HOME")
            .ok()
            .map(|home| Path::new(&(home + "/.nvm")).exists())
            .unwrap_or(false),
        SoftwareId::Bun => check_command("bun"),
        SoftwareId::Go => check_command("go"),
        SoftwareId::Flutter => check_command("flutter"),
        SoftwareId::ReactNativeCli => check_command("react-native"),
        SoftwareId::DioxusCli => check_command("dioxus"),
    }
}

pub fn install(id: SoftwareId) -> Result<(), Box<dyn Error>> {
    match id {
        SoftwareId::Homebrew => run_shell(
            "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"",
        ),
        SoftwareId::XcodeCommandLineTools => run_shell("xcode-select --install"),
        SoftwareId::Brave => run_brew_cask("brave-browser"),
        SoftwareId::Firefox => run_brew_cask("firefox"),
        SoftwareId::Chrome => run_brew_cask("google-chrome"),
        SoftwareId::JetBrainsToolbox => run_brew_cask("jetbrains-toolbox"),
        SoftwareId::ZedStable => run_brew_cask("zed"),
        SoftwareId::ZedPreview => run_brew_cask("zed@preview"),
        SoftwareId::Raycast => run_brew_cask("raycast"),
        SoftwareId::Rustup => {
            run_shell("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y")?;
            source_cargo_env();
            Ok(())
        }
        SoftwareId::RustStable => run_shell("rustup default stable"),
        SoftwareId::RustNightly => run_shell("rustup toolchain install nightly"),
        SoftwareId::CargoJust => run_shell("cargo install just"),
        SoftwareId::CargoBinstall => run_shell("cargo install cargo-binstall"),
        SoftwareId::CargoWatch => run_shell("cargo install cargo-watch"),
        SoftwareId::SimpleHttpServer => run_shell("cargo +nightly install simple-http-server"),
        SoftwareId::Nvm => {
            run_shell(
                "curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash",
            )?;
            run_shell(
                "export NVM_DIR=\"$HOME/.nvm\" && [ -s \"$NVM_DIR/nvm.sh\" ] \
                 && . \"$NVM_DIR/nvm.sh\" && nvm install --lts",
            )
        }
        SoftwareId::Bun => run_shell("curl -fsSL https://bun.sh/install | bash"),
        SoftwareId::Go => run_brew_pkg("go"),
        SoftwareId::Flutter => run_brew_cask("flutter"),
        SoftwareId::ReactNativeCli => run_shell("npm install -g react-native-cli")
            .or_else(|_| Err("npm (Node.js) is required for React Native CLI.".into())),
        SoftwareId::DioxusCli => {
            source_cargo_env();
            run_shell("cargo install dioxus-cli")
        }
    }
}

fn run_brew_cask(cask: &str) -> Result<(), Box<dyn Error>> {
    run_shell(&format!("brew install --cask {cask}"))
}

fn run_brew_pkg(pkg: &str) -> Result<(), Box<dyn Error>> {
    run_shell(&format!("brew install {pkg}"))
}

fn run_shell(cmd: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Command `{cmd}` failed: {}", stderr.trim()).into())
    }
}

fn check_command(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()
        .map_or(false, |status| status.success())
}

fn check_gui_app(app_name: &str) -> bool {
    Command::new("open")
        .arg("-Ra")
        .arg(app_name)
        .status()
        .map_or(false, |status| status.success())
}

fn source_cargo_env() {
    if let Ok(home) = env::var("HOME") {
        let path = format!("{home}/.cargo/env");
        if let Ok(file) = File::open(&path) {
            let reader = BufReader::new(file);
            for line in reader.lines().flatten() {
                if let Some(rest) = line.strip_prefix("export ") {
                    if let Some((key, value)) = rest.split_once('=') {
                        let cleaned = value.trim_matches('"');
                        unsafe {
                            std::env::set_var(key, cleaned);
                        }
                    }
                }
            }
        }
    }
}
