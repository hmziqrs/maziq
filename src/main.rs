use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{self, BufRead, BufReader, Write},
    path::Path,
    process::{Command, Stdio},
    time::{Duration, Instant},
};

const PROGRESS_FILE: &str = "install_progress.txt";

#[derive(Clone, PartialEq)]
enum InstallItem {
    Homebrew,
    Brave,
    Firefox,
    Chrome,
    XcodeCLT,
    JetBrainsToolbox,
    Rustup,
    RustStable,
    RustNightly,
    CargoJust,
    CargoBinstall,
    CargoWatch,
    SimpleHttpServer,
    Nvm,
    Bun,
    Raycast,
    ZedStable,
    ZedPreview,
}

impl InstallItem {
    fn all() -> Vec<Self> {
        vec![
            Self::Homebrew,
            Self::Brave,
            Self::Firefox,
            Self::Chrome,
            Self::XcodeCLT,
            Self::JetBrainsToolbox,
            Self::Rustup,
            Self::RustStable,
            Self::RustNightly,
            Self::CargoJust,
            Self::CargoBinstall,
            Self::CargoWatch,
            Self::SimpleHttpServer,
            Self::Nvm,
            Self::Bun,
            Self::Raycast,
            Self::ZedStable,
            Self::ZedPreview,
        ]
    }

    fn name(&self) -> &str {
        match self {
            Self::Homebrew => "Homebrew",
            Self::Brave => "Brave Browser",
            Self::Firefox => "Firefox",
            Self::Chrome => "Google Chrome",
            Self::XcodeCLT => "Xcode Command-Line Tools",
            Self::JetBrainsToolbox => "JetBrains Toolbox (for Android Studio)",
            Self::Rustup => "Rustup",
            Self::RustStable => "Rust Stable",
            Self::RustNightly => "Rust Nightly",
            Self::CargoJust => "Cargo: just",
            Self::CargoBinstall => "Cargo: cargo-binstall",
            Self::CargoWatch => "Cargo: cargo-watch",
            Self::SimpleHttpServer => "Cargo: simple-http-server (nightly)",
            Self::Nvm => "nvm (Node Version Manager)",
            Self::Bun => "Bun",
            Self::Raycast => "Raycast",
            Self::ZedStable => "Zed Stable",
            Self::ZedPreview => "Zed Preview",
        }
    }

    fn check_installed(&self) -> bool {
        match self {
            Self::Homebrew => check_command("brew"),
            Self::Brave => {
                check_command("open")
                    && Command::new("open")
                        .arg("-a")
                        .arg("Brave Browser")
                        .status()
                        .map_or(false, |s| s.success())
            }
            Self::Firefox => {
                check_command("open")
                    && Command::new("open")
                        .arg("-a")
                        .arg("Firefox")
                        .status()
                        .map_or(false, |s| s.success())
            }
            Self::Chrome => {
                check_command("open")
                    && Command::new("open")
                        .arg("-a")
                        .arg("Google Chrome")
                        .status()
                        .map_or(false, |s| s.success())
            }
            Self::XcodeCLT => check_command("gcc"),
            Self::JetBrainsToolbox => {
                check_command("open")
                    && Command::new("open")
                        .arg("-a")
                        .arg("JetBrains Toolbox")
                        .status()
                        .map_or(false, |s| s.success())
            }
            Self::Rustup => check_command("rustup"),
            Self::RustStable => {
                check_command("rustc")
                    && Command::new("rustup")
                        .arg("show")
                        .output()
                        .map_or(false, |o| {
                            String::from_utf8_lossy(&o.stdout).contains("stable")
                        })
            }
            Self::RustNightly => Command::new("rustup")
                .arg("toolchain")
                .arg("list")
                .output()
                .map_or(false, |o| {
                    String::from_utf8_lossy(&o.stdout).contains("nightly")
                }),
            Self::CargoJust => check_command("just"),
            Self::CargoBinstall => check_command("cargo-binstall"),
            Self::CargoWatch => check_command("cargo-watch"),
            Self::SimpleHttpServer => check_command("simple-http-server"),
            Self::Nvm => env::var("HOME")
                .map(|home| Path::new(&(home + "/.nvm")).exists())
                .unwrap_or(false),
            Self::Bun => check_command("bun"),
            Self::Raycast => {
                check_command("open")
                    && Command::new("open")
                        .arg("-a")
                        .arg("Raycast")
                        .status()
                        .map_or(false, |s| s.success())
            }
            Self::ZedStable => {
                check_command("open")
                    && Command::new("open")
                        .arg("-a")
                        .arg("Zed")
                        .status()
                        .map_or(false, |s| s.success())
            }
            Self::ZedPreview => {
                check_command("open")
                    && Command::new("open")
                        .arg("-a")
                        .arg("Zed Preview")
                        .status()
                        .map_or(false, |s| s.success())
            }
        }
    }

    fn install_command(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Homebrew => {
                run_shell(
                    "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"",
                )?;
            }
            Self::Brave => run_brew_cask("brave-browser")?,
            Self::Firefox => run_brew_cask("firefox")?,
            Self::Chrome => run_brew_cask("google-chrome")?,
            Self::XcodeCLT => {
                run_shell("xcode-select --install")?;
            }
            Self::JetBrainsToolbox => run_brew_cask("jetbrains-toolbox")?,
            Self::Rustup => {
                run_shell(
                    "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y",
                )?;
                source_cargo_env();
            }
            Self::RustStable => run_shell("rustup default stable")?,
            Self::RustNightly => run_shell("rustup toolchain install nightly")?,
            Self::CargoJust => run_shell("cargo install just")?,
            Self::CargoBinstall => run_shell("cargo install cargo-binstall")?,
            Self::CargoWatch => run_shell("cargo install cargo-watch")?,
            Self::SimpleHttpServer => run_shell("cargo +nightly install simple-http-server")?,
            Self::Nvm => {
                run_shell(
                    "curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash",
                )?;
                if let Ok(home) = env::var("HOME") {
                    let nvm_dir = format!("{}/.nvm", home);
                    unsafe {
                        env::set_var("NVM_DIR", nvm_dir);
                    }
                    run_shell("nvm install --lts")?;
                }
            }
            Self::Bun => run_shell("curl -fsSL https://bun.sh/install | bash")?,
            Self::Raycast => run_brew_cask("raycast")?,
            Self::ZedStable => run_brew_cask("zed")?,
            Self::ZedPreview => run_brew_cask("zed@preview")?,
        }
        Ok(())
    }
}

fn check_command(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .status()
        .map_or(false, |s| s.success())
}

fn run_brew_cask(cask: &str) -> Result<(), Box<dyn Error>> {
    run_shell(&format!("brew install --cask {}", cask))
}

fn run_shell(cmd: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output()?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Command failed: {}\nError: {}", cmd, err).into());
    }
    Ok(())
}

fn source_cargo_env() {
    if let Ok(home) = env::var("HOME") {
        let cargo_env = format!("{}/.cargo/env", home);
        if Path::new(&cargo_env).exists() {
            if let Ok(file) = File::open(&cargo_env) {
                let reader = BufReader::new(file);
                for line in reader.lines().flatten() {
                    if let Some(stripped) = line.strip_prefix("export ") {
                        let parts: Vec<&str> = stripped.splitn(2, '=').collect();
                        if parts.len() == 2 {
                            unsafe {
                                env::set_var(parts[0], parts[1].trim_matches('"'));
                            }
                        }
                    }
                }
            }
        }
    }
}

struct App {
    items: Vec<InstallItem>,
    state: ListState,
    statuses: Vec<(String, bool, Option<String>)>,
    progress: Vec<String>,
    message: String,
    quit: bool,
}

impl App {
    fn new() -> Self {
        let items = InstallItem::all();
        let mut app = Self {
            items: items.clone(),
            state: ListState::default(),
            statuses: vec![],
            progress: load_progress(),
            message: String::new(),
            quit: false,
        };
        app.state.select(Some(0));
        app.refresh_statuses();
        app
    }

    fn refresh_statuses(&mut self) {
        self.statuses = self
            .items
            .iter()
            .map(|item| {
                let installed =
                    item.check_installed() || self.progress.contains(&item.name().to_string());
                (item.name().to_string(), installed, None)
            })
            .collect();
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) if i >= self.items.len() - 1 => 0,
            Some(i) => i + 1,
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(0) | None => self.items.len() - 1,
            Some(i) => i - 1,
        };
        self.state.select(Some(i));
    }

    fn install_selected(&mut self) {
        if let Some(i) = self.state.selected() {
            let item = &self.items[i];
            if self.statuses.get(i).map(|status| status.1).unwrap_or(false) {
                self.message = format!("{} already installed.", item.name());
                return;
            }
            self.message = format!("Installing {}...", item.name());
            match item.install_command() {
                Ok(_) => {
                    self.message = format!("{} installed successfully.", item.name());
                    self.progress.push(item.name().to_string());
                    save_progress(&self.progress);
                    if let Some(status) = self.statuses.get_mut(i) {
                        status.1 = true;
                        status.2 = None;
                    }
                }
                Err(e) => {
                    self.message = format!("Error installing {}: {}", item.name(), e);
                    if let Some(status) = self.statuses.get_mut(i) {
                        status.2 = Some(e.to_string());
                    }
                }
            }
        }
    }

    fn install_all_missing(&mut self) {
        self.message = "Installing all missing items...".to_string();
        for (i, item) in self.items.iter().enumerate() {
            let installed = self.statuses.get(i).map(|status| status.1).unwrap_or(false);
            if installed {
                continue;
            }
            self.message = format!("Installing {}...", item.name());
            if let Err(e) = item.install_command() {
                self.message = format!("Error on {}: {}. Resumable later.", item.name(), e);
                if let Some(status) = self.statuses.get_mut(i) {
                    status.2 = Some(e.to_string());
                }
            } else {
                self.progress.push(item.name().to_string());
                save_progress(&self.progress);
                if let Some(status) = self.statuses.get_mut(i) {
                    status.1 = true;
                    status.2 = None;
                }
            }
        }
        self.message = "Installation loop complete. Check statuses.".to_string();
    }
}

fn load_progress() -> Vec<String> {
    fs::read_to_string(PROGRESS_FILE)
        .map(|contents| contents.lines().map(|s| s.to_string()).collect())
        .unwrap_or_default()
}

fn save_progress(progress: &[String]) {
    if let Ok(mut file) = File::create(PROGRESS_FILE) {
        for p in progress {
            let _ = writeln!(file, "{}", p);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(250);
    let mut app = App::new();
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => app.quit = true,
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Enter => app.install_selected(),
                    KeyCode::Char('a') => app.install_all_missing(),
                    KeyCode::Char('r') => {
                        app.refresh_statuses();
                        app.message = "Statuses refreshed.".to_string();
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        if app.quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame<'_>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.size());

    let items: Vec<ListItem> = app
        .statuses
        .iter()
        .map(|(name, installed, err)| {
            let status = if *installed {
                "✅ Installed"
            } else {
                "❌ Missing"
            };
            let err_span = err
                .as_ref()
                .map(|e| format!(" (Error: {})", e))
                .unwrap_or_default();
            ListItem::new(Line::from(vec![
                Span::styled(
                    name.clone(),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" - "),
                Span::styled(
                    status,
                    Style::default().fg(if *installed { Color::Green } else { Color::Red }),
                ),
                Span::raw(err_span),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Installation Status")
                .borders(Borders::ALL),
        )
        .highlight_style(Style::default().bg(Color::LightBlue).fg(Color::Black))
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[0], &mut app.state);

    let lower_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(chunks[1]);

    let message = Paragraph::new(app.message.as_str())
        .block(Block::default().title("Messages").borders(Borders::ALL));
    f.render_widget(message, lower_chunks[0]);

    let controls_text = "Controls: j/k or Up/Down: Navigate | Enter: Install Selected | a: Install All Missing | r: Refresh | q: Quit";
    let controls = Paragraph::new(controls_text)
        .block(Block::default().title("Controls").borders(Borders::ALL));
    f.render_widget(controls, lower_chunks[1]);
}
