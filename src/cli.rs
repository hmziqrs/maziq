use std::error::Error;

use clap::{Args, Parser, Subcommand};

use crate::{
    catalog::{self, SoftwareEntry, SoftwareId},
    templates,
};

#[derive(Parser, Debug)]
#[command(name = "maziq", about = "CLI for managing macOS development setups.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Inspect software definitions and actions.
    #[command(subcommand)]
    Software(SoftwareCommand),
    /// Template-driven onboarding flows.
    #[command(subcommand)]
    Onboard(OnboardCommand),
    /// Experimental workstation configuration helpers.
    #[command(subcommand)]
    Config(ConfigCommand),
    /// Show version detection commands for every software entry.
    Versions,
}

#[derive(Subcommand, Debug)]
enum SoftwareCommand {
    /// List all known software entries.
    List,
    /// Show detail for a specific software id (use the key, e.g. `rustup`).
    Show { id: String },
}

#[derive(Subcommand, Debug)]
enum OnboardCommand {
    /// Preview or dry-run the first-time install sequence.
    Fresh(OnboardFlowArgs),
    /// Preview or dry-run the update workflow for an existing install.
    Update(OnboardFlowArgs),
    /// List available templates.
    Templates,
}

#[derive(Args, Debug)]
struct OnboardFlowArgs {
    /// Template slug or name (defaults to hmziq).
    #[arg(short, long, default_value = "hmziq")]
    template: String,
    /// Do not execute anything; show the plan instead.
    #[arg(long)]
    dry_run: bool,
}

#[derive(Subcommand, Debug)]
enum ConfigCommand {
    /// List configuration profiles shipped with MazIQ.
    List,
    /// Apply a configuration profile (experimental).
    Apply {
        /// Profile id to apply.
        profile: String,
        /// Preview the actions without modifying the system.
        #[arg(long)]
        dry_run: bool,
    },
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
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
            println!("{:<20} {:<25} {:<8} {}", "Key", "Name", "Kind", "Category");
            println!("{}", "-".repeat(80));
            for entry in catalog::all_entries() {
                println!(
                    "{:<20} {:<25} {:<8} {}",
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
    }
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
        OnboardCommand::Fresh(args) => preview_template("Fresh installation", args)?,
        OnboardCommand::Update(args) => preview_template("Update workflow", args)?,
    }
    Ok(())
}

fn preview_template(label: &str, args: OnboardFlowArgs) -> Result<(), Box<dyn Error>> {
    let template = templates::load_named(&args.template)?;
    println!("{label} for template `{}`", template.name);
    if let Some(desc) = &template.description {
        println!("Description: {desc}");
    }
    println!(
        "Total software entries: {}{}",
        template.software.len(),
        if args.dry_run { " (dry run)" } else { "" }
    );
    for (index, id) in template.software.iter().enumerate() {
        let entry = catalog::entry(*id);
        println!(
            "{:>2}. {:<24} {:<6} deps: {}",
            index + 1,
            entry.display_name,
            entry.kind.label(),
            format_dependencies(entry.dependencies)
        );
    }
    if !args.dry_run {
        println!(
            "\nExecution engine is not wired yet. Add `--dry-run` to preview without this notice."
        );
    }
    Ok(())
}

fn handle_config(cmd: ConfigCommand) -> Result<(), Box<dyn Error>> {
    match cmd {
        ConfigCommand::List => {
            println!("Configurator profiles (experimental):");
            println!("- git-basics (Git identity, SSH, GPG)");
            println!("- hmziq-default (workstation defaults from hmziq)");
            println!("\nUse `maziq config apply <profile> --dry-run` to preview.");
        }
        ConfigCommand::Apply { profile, dry_run } => {
            println!(
                "Configurator `{profile}` is experimental. {}",
                if dry_run {
                    "Dry run requested; showing preview only."
                } else {
                    "No actions executed yet."
                }
            );
            println!("Planned steps:");
            println!("- Backup existing git/ssh/gpg configs.");
            println!("- Apply profile-specific settings.");
            println!("- Set git default branch to master and pull.rebase true.");
            if !dry_run {
                println!(
                    "Execution pipeline not implemented. Re-run with --dry-run to inspect steps."
                );
            }
        }
    }
    Ok(())
}

fn handle_versions() {
    println!("Version detection commands:");
    for entry in catalog::all_entries() {
        println!(
            "- {:<22}: {}",
            entry.display_name,
            entry.version_probe.description()
        );
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
    println!("Install: {}", entry.install.description());
    println!("Update: {}", entry.update.description());
    println!("Uninstall: {}", entry.uninstall.description());
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
