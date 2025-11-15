use std::error::Error;

use clap::{Args, Parser, Subcommand};

use crate::{
    catalog::{self, CommandSource, SoftwareEntry, SoftwareId},
    configurator::{self, ApplyOptions as ConfigApplyOptions},
    manager::{ActionKind, ExecutionEvent, SoftwareManager, StatusReport, StatusState},
    templates,
};

#[derive(Parser, Debug)]
#[command(name = "maziq", about = "CLI for managing macOS development setups.")]
pub struct Cli {
    /// Global dry-run (applies to software installs, onboarding, configurator).
    #[arg(long)]
    pub dry_run: bool,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Inspect software definitions and actions.
    #[command(subcommand)]
    Software(SoftwareCommand),
    /// Template-driven onboarding flows.
    #[command(subcommand)]
    Onboard(OnboardCommand),
    /// Experimental workstation configuration helpers.
    #[command(subcommand)]
    Config(ConfigCommand),
    /// Detect installed versions across all software.
    Versions,
}

#[derive(Subcommand, Debug)]
pub enum SoftwareCommand {
    /// List all known software entries.
    List,
    /// Show detail for a specific software id (use the key, e.g. `rustup`).
    Show { id: String },
    /// Install a specific software entry (resolves dependencies automatically).
    Install(SoftwareActionArgs),
    /// Update a specific software entry.
    Update(SoftwareActionArgs),
    /// Uninstall a specific software entry.
    Uninstall(SoftwareActionArgs),
    /// Show current status/version for a software entry.
    Status { id: String },
}

#[derive(Args, Debug)]
pub struct SoftwareActionArgs {
    /// Software id (see `maziq software list` for options).
    id: String,
    /// Preview actions without modifying the system.
    #[arg(long)]
    dry_run: bool,
    /// Force action even if MazIQ detects the target is already installed.
    #[arg(long)]
    force: bool,
}

#[derive(Subcommand, Debug)]
pub enum OnboardCommand {
    /// Install every item defined in a template.
    Fresh(OnboardFlowArgs),
    /// Update every item defined in a template.
    Update(OnboardFlowArgs),
    /// List available templates.
    Templates,
}

#[derive(Args, Debug)]
pub struct OnboardFlowArgs {
    /// Template slug or name (defaults to hmziq).
    #[arg(short, long, default_value = "hmziq")]
    template: String,
    /// Do not execute anything; show the plan instead.
    #[arg(long)]
    dry_run: bool,
    /// Force reinstall/update even if targets appear installed.
    #[arg(long)]
    force: bool,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommand {
    /// List configuration profiles shipped with MazIQ.
    List,
    /// Apply a configuration profile (experimental).
    Apply {
        /// Profile id to apply.
        profile: String,
        /// Preview the actions without modifying the system.
        #[arg(long)]
        dry_run: bool,
        /// Confirm experimental configurator usage.
        #[arg(long = "experimental-config")]
        experimental: bool,
    },
}

pub fn run(command: Commands) -> Result<(), Box<dyn Error>> {
    match command {
        Commands::Software(cmd) => handle_software(cmd)?,
        Commands::Onboard(cmd) => handle_onboard(cmd)?,
        Commands::Config(cmd) => handle_config(cmd)?,
        Commands::Versions => handle_versions(),
    }
    Ok(())
}

fn handle_software(cmd: SoftwareCommand) -> Result<(), Box<dyn Error>> {
    match cmd {
        SoftwareCommand::List => {
            println!("{:<20} {:<26} {:<8} {}", "Key", "Name", "Kind", "Category");
            println!("{}", "-".repeat(90));
            for entry in catalog::all_entries() {
                println!(
                    "{:<20} {:<26} {:<8} {}",
                    entry.id.key(),
                    entry.display_name,
                    entry.kind.label(),
                    entry.category
                );
            }
        }
        SoftwareCommand::Show { id } => {
            if let Some(entry) = catalog::find_by_key(&id) {
                print_entry(&entry);
            } else {
                eprintln!("Unknown software id `{id}`.");
            }
        }
        SoftwareCommand::Install(args) => run_action_for_id(ActionKind::Install, args)?,
        SoftwareCommand::Update(args) => run_action_for_id(ActionKind::Update, args)?,
        SoftwareCommand::Uninstall(args) => run_action_for_id(ActionKind::Uninstall, args)?,
        SoftwareCommand::Status { id } => {
            if let Some(id) = SoftwareId::from_key(&id) {
                let manager = SoftwareManager::new();
                let status = manager.status(id)?;
                print_status(&status);
            } else {
                eprintln!("Unknown software id `{id}`.");
            }
        }
    }
    Ok(())
}

fn run_action_for_id(action: ActionKind, args: SoftwareActionArgs) -> Result<(), Box<dyn Error>> {
    let Some(id) = SoftwareId::from_key(&args.id) else {
        eprintln!("Unknown software id `{}`.", args.id);
        return Ok(());
    };
    let manager = SoftwareManager::with_flags(args.dry_run, args.force);
    let events = match action {
        ActionKind::Install => manager.install(id)?,
        ActionKind::Update => manager.update(id)?,
        ActionKind::Uninstall => manager.uninstall(id)?,
        ActionKind::Test => manager.install(id)?, // placeholder
    };
    render_events(&events);
    Ok(())
}

fn handle_onboard(cmd: OnboardCommand) -> Result<(), Box<dyn Error>> {
    match cmd {
        OnboardCommand::Templates => {
            let templates = templates::load_all()?;
            if templates.is_empty() {
                println!("No templates found in the `templates/` directory.");
            } else {
                println!("Available templates:");
                for template in templates {
                    println!(
                        "- {} ({}) [{}]",
                        template.name,
                        template
                            .description
                            .as_deref()
                            .unwrap_or("no description provided"),
                        template.path.display()
                    );
                }
            }
        }
        OnboardCommand::Fresh(args) => run_onboard_flow(ActionKind::Install, args)?,
        OnboardCommand::Update(args) => run_onboard_flow(ActionKind::Update, args)?,
    }
    Ok(())
}

fn run_onboard_flow(action: ActionKind, args: OnboardFlowArgs) -> Result<(), Box<dyn Error>> {
    let template = templates::load_named(&args.template)?;
    println!(
        "{} template `{}`{}",
        match action {
            ActionKind::Install => "Installing",
            ActionKind::Update => "Updating",
            ActionKind::Uninstall => "Uninstalling",
            ActionKind::Test => "Testing",
        },
        template.name,
        if args.dry_run { " (dry run)" } else { "" }
    );
    if let Some(desc) = &template.description {
        println!("Description: {desc}");
    }
    let manager = SoftwareManager::with_flags(args.dry_run, args.force);
    let plan = manager.plan(&template.software, action)?;
    println!("\nExecution order:");
    for (index, id) in plan.iter().enumerate() {
        println!(
            "{:>2}. {:<24} deps: {}",
            index + 1,
            id.name(),
            format_dependencies(catalog::entry(*id).dependencies)
        );
    }

    if args.dry_run {
        println!("\nDry run mode enabled; no commands executed.");
        return Ok(());
    }

    println!();
    let events = match action {
        ActionKind::Install => manager.install_many(&template.software)?,
        ActionKind::Update => manager.update_many(&template.software)?,
        ActionKind::Uninstall => manager.uninstall_many(&template.software)?,
        ActionKind::Test => manager.install_many(&template.software)?,
    };
    render_events(&events);
    Ok(())
}

fn handle_config(cmd: ConfigCommand) -> Result<(), Box<dyn Error>> {
    match cmd {
        ConfigCommand::List => {
            println!("Configurator profiles (experimental):\n");
            for profile in configurator::profiles() {
                println!(
                    "- {} ({}): {}",
                    profile.id, profile.display_name, profile.description
                );
            }
            println!(
                "\nUse `maziq config apply <profile> --dry-run --experimental-config` to preview."
            );
        }
        ConfigCommand::Apply {
            profile,
            dry_run,
            experimental,
        } => {
            match configurator::apply_profile(
                &profile,
                ConfigApplyOptions {
                    dry_run,
                    experimental,
                },
            ) {
                Ok(result) => {
                    if let Some(path) = result.backup_path {
                        println!("Existing gitconfig backed up to {}", path.display());
                    }
                    println!(
                        "{} profile `{}`:",
                        if result.dry_run { "Planned" } else { "Applied" },
                        result.profile_id
                    );
                    for action in result.actions {
                        println!("- {action}");
                    }
                }
                Err(err) => eprintln!("Configurator error: {err}"),
            }
        }
    }
    Ok(())
}

fn handle_versions() {
    let manager = SoftwareManager::new();
    println!("Detected software versions:");
    for report in manager.status_all() {
        println!("{}", summarize_status(&report));
    }
}

fn print_entry(entry: &SoftwareEntry) {
    println!("Key: {}", entry.id.key());
    println!("Name: {}", entry.display_name);
    println!("Category: {}", entry.category);
    println!("Kind: {}", entry.kind.label());
    println!("Summary: {}", entry.summary);
    if entry.dependencies.is_empty() {
        println!("Dependencies: none");
    } else {
        println!(
            "Dependencies: {}",
            entry
                .dependencies
                .iter()
                .map(|id| id.name())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    println!("Version check: {}", entry.version_probe.description());
    print_sources("Install", entry.install_sources());
    print_sources("Update", entry.update_sources());
    print_sources("Uninstall", entry.uninstall_sources());
}

fn print_sources(title: &str, sources: Vec<CommandSource>) {
    if sources.is_empty() {
        println!("{title}: none");
        return;
    }
    println!("{title} sources:");
    for source in sources {
        println!("  - {} -> {}", source.label, source.recipe.description());
    }
}

fn render_events(events: &[ExecutionEvent]) {
    if events.is_empty() {
        println!("No actions executed.");
        return;
    }
    println!("Action log:");
    for event in events {
        println!("- {}", event.summary());
    }
}

fn print_status(report: &StatusReport) {
    println!("{}", summarize_status(report));
}

fn summarize_status(report: &StatusReport) -> String {
    match &report.state {
        StatusState::Installed { version } => {
            if let Some(version) = version {
                format!("{} ({}) -> {}", report.id.name(), report.id.key(), version)
            } else {
                format!("{} ({}) -> installed", report.id.name(), report.id.key())
            }
        }
        StatusState::NotInstalled => {
            format!(
                "{} ({}) -> not installed",
                report.id.name(),
                report.id.key()
            )
        }
        StatusState::ManualCheck(note) => format!(
            "{} ({}) -> manual check required: {}",
            report.id.name(),
            report.id.key(),
            note
        ),
        StatusState::Unknown(note) => format!(
            "{} ({}) -> unknown: {}",
            report.id.name(),
            report.id.key(),
            note
        ),
    }
}

fn format_dependencies(ids: &[SoftwareId]) -> String {
    if ids.is_empty() {
        "none".into()
    } else {
        ids.iter()
            .map(|id| id.key().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }
}
