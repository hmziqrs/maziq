# MazIQ

A powerful macOS development environment manager built in Rust that automates the setup and maintenance of your developer workstation.

## Overview

MazIQ streamlines the onboarding experience for macOS developers by providing both a command-line interface (CLI) and an interactive terminal user interface (TUI) to install, update, uninstall, and monitor development tools and applications. Whether you're setting up a fresh Mac or maintaining your existing environment, MazIQ handles dependency resolution, version detection, and installation tracking automatically.

## Key Features

- **69+ Development Tools**: Comprehensive catalog of essential development software including editors, browsers, CLI tools, language runtimes, and utilities
- **Dual Interface**: Choose between CLI commands for automation or an interactive TUI for visual feedback
- **Smart Dependency Resolution**: Automatically installs prerequisites (e.g., Homebrew before packages, Rust before `just`)
- **Template-Based Workflows**: Define reproducible environment setups with TOML templates
- **Multi-Source Installation**: Supports Homebrew, direct downloads, npm, cargo, and other installation methods with automatic fallback
- **Intelligent Version Detection**: Uses `mdls` for macOS apps, CLI commands for tools, and Homebrew metadata
- **Installation History**: Tracks all operations with timestamps and source information in JSONL format
- **Dry-Run Mode**: Preview actions without executing them using the `--dry-run` flag
- **Async Task Execution**: Background processing with live progress feedback in the TUI
- **E2E Testing**: Built-in end-to-end testing workflow for validating installations

## Installation

### Prerequisites

- macOS (primary target platform)
- Rust toolchain (for building from source)

### Build from Source

```bash
git clone https://github.com/yourusername/maziq.git
cd maziq
cargo build --release
```

The binary will be available at `target/release/maziq`.

### Install to System

```bash
cargo install --path .
```

## Usage

### Interactive TUI Mode (Default)

Simply run the binary without arguments to launch the interactive terminal interface:

```bash
maziq
```

The TUI provides six main workflows:

1. **Onboard (Fresh Install)** - Install all software from a template
2. **Update Template** - Update all software in your template
3. **Configurator** (Experimental) - Configure Git/SSH/GPG settings
4. **Software Catalog** - Browse and manage individual software
5. **Versions** - View all detected software versions
6. **Brew End-to-End Test** - Test Homebrew installation workflow

**Navigation**:
- `j`/`k` or Arrow keys: Navigate
- `Enter`: Select/Execute
- `Space`: Toggle selection (where applicable)
- `Esc` or `q`: Go back/Quit
- `Ctrl+C`: Exit application

### CLI Mode

#### Software Management

List all available software:
```bash
maziq software list
```

Show details for specific software:
```bash
maziq software show rust
```

Install software:
```bash
maziq software install rust
```

Update software:
```bash
maziq software update rust
```

Uninstall software:
```bash
maziq software uninstall rust
```

Check software status:
```bash
maziq software status rust
```

#### Onboarding Workflows

Fresh installation from template:
```bash
maziq onboard fresh
```

Update all software in template:
```bash
maziq onboard update
```

List available templates:
```bash
maziq onboard templates
```

#### Configuration Management (Experimental)

List configuration profiles:
```bash
maziq config list
```

Apply configuration:
```bash
maziq config apply <profile>
```

#### Version Detection

Show all detected software versions:
```bash
maziq versions
```

#### Global Options

Preview actions without executing (dry-run):
```bash
maziq --dry-run software install rust
```

## Software Catalog

MazIQ supports 69+ development tools across multiple categories:

### System Essentials
- Homebrew
- Xcode Command-Line Tools

### Browsers
- Brave, Firefox, Chrome

### Editors & IDEs
- Cursor, Windsurf, VS Code, Zed
- JetBrains Toolbox
- Android Studio

### Terminal & Shell
- iTerm2, Warp, Alacritty
- Fish shell

### Desktop Utilities
- Raycast, Docker Desktop

### Communication
- Slack, Discord, Zoom

### Design & Media
- Figma, VLC

### API & Testing Tools
- Postman, Yaak, Bruno

### Database Tools
- TablePlus, DBeaver
- MongoDB Compass, RedisInsight

### Language Runtimes & Tools

**Rust Stack**:
- Rustup, Rust toolchains
- Cargo tools (just, etc.)

**Python Stack**:
- Pyenv, Poetry, Uv

**JavaScript & Node**:
- NVM, Bun
- Electron Forge

**Other Languages**:
- Go, Flutter

### DevOps & Cloud
- Kubectl, Terraform
- AWS CLI, Azure CLI

### CLI Utilities
- jq, ripgrep, fd, bat, fzf, btop, neovim

### AI Assistant CLIs
- Codex, Claude, Claude Multi
- Kimi, Gemini, Qwen, Opencode

## Templates

Templates define sets of software to install together. Create custom templates as TOML files in the `templates/` directory.

### Default Template

The included `hmziq.toml` template contains 38 commonly-used development tools:

```toml
[[software]]
id = "rust"

[[software]]
id = "node"

[[software]]
id = "cursor"

# ... more entries
```

### Using Templates

Templates are automatically loaded from the `templates/` directory. To create a custom template:

1. Create a new `.toml` file in `templates/`
2. Define software entries using the `[[software]]` section
3. Reference software by their catalog ID

## Installation History

MazIQ tracks all installation operations in `install_history.jsonl`:

```json
{"software_id":"rust","action":"install","version":"1.75.0","source":"rustup","timestamp":1704067200}
{"software_id":"cursor","action":"install","version":"0.20.0","source":"homebrew_cask","timestamp":1704070800}
```

This history helps you understand what was installed, when, and how—even if version detection fails later.

## Configuration

### Configurator (Experimental)

The configurator helps set up development environment configurations:

- Git (email, username, default branch, pull mode)
- SSH keys
- GPG keys

**Note**: This feature is experimental. Use with caution and consider backing up your configuration first.

## Development

### Project Structure

```
maziq/
├── src/
│   ├── main.rs          # Entry point and TUI event loop
│   ├── cli.rs           # CLI command definitions
│   ├── app.rs           # Application state and TUI logic
│   ├── catalog.rs       # Software catalog (69+ entries)
│   ├── manager.rs       # Software adapter and execution engine
│   ├── configurator.rs  # Configuration management
│   ├── tui.rs           # Terminal UI rendering
│   ├── templates.rs     # Template loading and parsing
│   ├── history.rs       # Installation history tracking
│   └── options.rs       # Global options (dry-run flag)
├── templates/
│   └── hmziq.toml       # Default template
├── doc/
│   ├── raw-plan.md      # Initial project goals
│   └── refined-plan.md  # Detailed implementation plan
├── Cargo.toml           # Rust package manifest
└── install_history.jsonl # Installation history log
```

### Dependencies

- **clap** - CLI argument parsing
- **ratatui** - Terminal UI framework
- **crossterm** - Terminal manipulation
- **serde** - Serialization/deserialization
- **serde_json** - JSON support
- **toml** - TOML parsing
- **once_cell** - Lazy static initialization
- **ctrlc** - Signal handling

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

## Version Detection Strategy

MazIQ uses a standardized approach to detect software versions:

1. **macOS .app bundles**: `mdls -name kMDItemVersion /Applications/App.app`
2. **CLI tools**: Execute `<tool> --version` and parse output
3. **Homebrew packages**: Query `brew list --versions <package>`
4. **Fallback**: Use `mdfind` + `mdls` for unknown app paths

## Roadmap

### Completed
- [x] Core CLI structure with subcommands
- [x] Software catalog with 69+ tools
- [x] Dependency-aware installation
- [x] Template-based onboarding
- [x] Interactive TUI with live feedback
- [x] Multi-source installation support
- [x] Installation history tracking
- [x] Version detection system
- [x] Basic configurator (experimental)

### Planned
- [ ] Global dry-run flag refinement
- [ ] Confirmation prompts for destructive operations
- [ ] Enhanced logging and summary reports
- [ ] Test harness for adapter validation
- [ ] JSON output mode for programmatic consumption
- [ ] Cross-platform support (Linux, Windows)
- [ ] Plugin system for custom software adapters

## Contributing

Contributions are welcome! To add new software to the catalog:

1. Add an entry to `src/catalog.rs`
2. Define installation, update, and uninstall strategies
3. Specify version detection method
4. List any dependencies
5. Test the adapter with dry-run mode

## License

This project is open source. Please see the LICENSE file for details.

## Acknowledgments

Built with Rust 🦀 and powered by:
- [ratatui](https://github.com/ratatui-org/ratatui) for the TUI
- [clap](https://github.com/clap-rs/clap) for CLI parsing
- The Rust community for excellent tooling

---

**Note**: MazIQ is designed for macOS development environments. While some tools may work on other platforms, full compatibility is not guaranteed.
