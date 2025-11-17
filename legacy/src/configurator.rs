use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug)]
pub enum ConfigError {
    NotFound(String),
    ExperimentalGate,
    MissingHome,
    Io(String, io::Error),
    Command { command: String, stderr: String },
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::NotFound(id) => write!(f, "Unknown configurator profile `{}`", id),
            ConfigError::ExperimentalGate => {
                write!(
                    f,
                    "Configurator is experimental. Pass --experimental-config to proceed."
                )
            }
            ConfigError::MissingHome => write!(f, "HOME environment variable not set"),
            ConfigError::Io(path, err) => write!(f, "IO error for {}: {}", path, err),
            ConfigError::Command { command, stderr } => {
                write!(f, "Command `{}` failed: {}", command, stderr)
            }
        }
    }
}

impl std::error::Error for ConfigError {}

#[derive(Clone, Debug)]
pub struct ConfigProfile {
    pub id: &'static str,
    pub display_name: &'static str,
    pub description: &'static str,
    pub git_user_name: &'static str,
    pub git_email: &'static str,
    pub gpg_key: Option<&'static str>,
    pub default_branch: &'static str,
    pub pull_rebase: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ApplyOptions {
    pub dry_run: bool,
    pub experimental: bool,
}

#[derive(Debug)]
pub struct ApplyResult {
    pub profile_id: String,
    pub dry_run: bool,
    pub actions: Vec<String>,
    pub backup_path: Option<PathBuf>,
}

pub fn profiles() -> &'static [ConfigProfile] {
    &PROFILES
}

pub fn find_profile(id: &str) -> Option<&'static ConfigProfile> {
    PROFILES
        .iter()
        .find(|profile| profile.id.eq_ignore_ascii_case(id))
}

pub fn apply_profile(id: &str, opts: ApplyOptions) -> Result<ApplyResult, ConfigError> {
    if !opts.experimental {
        return Err(ConfigError::ExperimentalGate);
    }
    let profile = find_profile(id).ok_or_else(|| ConfigError::NotFound(id.to_string()))?;

    let home = env::var("HOME").map_err(|_| ConfigError::MissingHome)?;
    let git_config = Path::new(&home).join(".gitconfig");
    let backup = if git_config.exists() {
        Some(backup_file(&git_config)?)
    } else {
        None
    };

    let mut actions = Vec::new();
    actions.push(format!("Set git user.name = {}", profile.git_user_name));
    actions.push(format!("Set git user.email = {}", profile.git_email));
    if let Some(key) = profile.gpg_key {
        actions.push(format!("Set git user.signingkey = {}", key));
        actions.push("Enable commit.gpgsign true".into());
    }
    actions.push(format!(
        "Set init.defaultBranch = {}",
        profile.default_branch
    ));
    actions.push(format!(
        "Set pull.rebase = {}",
        if profile.pull_rebase { "true" } else { "false" }
    ));
    actions
        .push("Ensure SSH keys exist (~/.ssh/id_ed25519) and agent is configured (manual).".into());

    if opts.dry_run {
        return Ok(ApplyResult {
            profile_id: profile.id.to_string(),
            dry_run: true,
            actions,
            backup_path: backup,
        });
    }

    run_git_config("user.name", profile.git_user_name)?;
    run_git_config("user.email", profile.git_email)?;
    if let Some(key) = profile.gpg_key {
        run_git_config("user.signingkey", key)?;
        run_git_config("commit.gpgsign", "true")?;
    }
    run_git_config("init.defaultBranch", profile.default_branch)?;
    let pull_value = if profile.pull_rebase { "true" } else { "false" };
    run_git_config("pull.rebase", pull_value)?;

    Ok(ApplyResult {
        profile_id: profile.id.to_string(),
        dry_run: false,
        actions,
        backup_path: backup,
    })
}

fn run_git_config(key: &str, value: &str) -> Result<(), ConfigError> {
    let output = Command::new("git")
        .args(["config", "--global", key, value])
        .output()
        .map_err(|err| ConfigError::Io("git".into(), err))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(ConfigError::Command {
            command: format!("git config --global {} {}", key, value),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        })
    }
}

fn backup_file(path: &Path) -> Result<PathBuf, ConfigError> {
    let backup_dir = path
        .parent()
        .map(|p| p.join(".maziq/backups"))
        .unwrap_or_else(|| PathBuf::from(".maziq/backups"));
    fs::create_dir_all(&backup_dir)
        .map_err(|err| ConfigError::Io(backup_dir.display().to_string(), err))?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let file_name = format!(
        "{}.backup.{}",
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("gitconfig"),
        timestamp
    );
    let backup_path = backup_dir.join(file_name);
    fs::copy(path, &backup_path)
        .map_err(|err| ConfigError::Io(backup_path.display().to_string(), err))?;
    Ok(backup_path)
}

const PROFILES: &[ConfigProfile] = &[
    ConfigProfile {
        id: "hmziq-default",
        display_name: "hmziq Default",
        description: "Git + GPG defaults (master branch, rebase pulls).",
        git_user_name: "hmziq",
        git_email: "hmziq@example.com",
        gpg_key: Some("ABCDE12345ABCDE"),
        default_branch: "master",
        pull_rebase: true,
    },
    ConfigProfile {
        id: "git-basics",
        display_name: "Git Basics",
        description: "Prompt for user identity (set after editing profile).",
        git_user_name: "Your Name",
        git_email: "you@example.com",
        gpg_key: None,
        default_branch: "master",
        pull_rebase: true,
    },
];
