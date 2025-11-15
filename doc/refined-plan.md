**Goal**
- Provide a CLI to set up and manage a fresh macOS development environment.

**Software Manager**
- Fetch latest versions of a defined software list at compile time and store them in a JSON manifest.
- Define per-software adapters that know how to install, update, and uninstall each tool (commands can differ per software).
- Check status of each software (not installed / outdated / up to date).
- Support dependency-aware operations (e.g., install Rust before `just`, install Bun/Node before `codex`).
- Standardize version detection per software: use `mdls -name kMDItemVersion` for `.app` bundles (Android Studio, Xcode, VS Code, Cursor), `<tool> --version` for CLI/SDKs (Flutter, Rust, Xcode tools, etc.), and `brew list --versions` for Homebrew packages, with `mdfind` + `mdls` as a fallback for unknown app paths.
- Provide a single command/script that prints all detected versions in one run for quick diagnostics.

**Testing**
- Provide a way to run install/update/tests for each software individually.
- Add a simple test harness to validate per-software adapters and dependency chains.

**Onboarding**
- Support a “fresh installation” workflow driven by a template (software list + config).
- Allow multiple templates; ship with a recommended `hmziq` template.
- Provide a manager command to install and update software from the selected template.

**Configurator**
- List available and currently applied configuration profiles.
- Install configuration (initially: Git, SSH, GPG key, email, username, default branch `master`, pull mode `rebase`).
- Mark the configurator as experimental and avoid using it as the default path for now.

**Implementation Phases (Codex Checklist)**

_Phase 1 – Core CLI & structure_
- [x] Scaffold the CLI binary with subcommands: `software`, `onboard`, `config`, `versions`.
- [x] Define a data model for software entries (id, display name, GUI/CLI, version command, install/update/uninstall strategy, dependencies, Homebrew cask/formula name).
- [x] Implement loading of software templates/configs, including a default `hmziq` template.
- [x] Keep an interactive Ratatui-based TUI as the default mode when no subcommand is provided, so users get a near-GUI experience with live feedback while installs run.
- [x] Mirror CLI actions (install/update/uninstall/status refresh) inside the TUI so both entrypoints offer the same capabilities.

_Phase 2 – Software manager engine_
- [x] Implement a per-software adapter trait (install, update, uninstall, status, test).
- [x] Register adapters for key tools (Rust, Bun, Node.js, just, Codex, Flutter, Android Studio, Xcode, VS Code, Cursor, etc.).
- [x] Implement dependency-aware execution so prerequisites (e.g. Rust for `just`, Bun/Node for `codex`) are installed first.
- [x] Implement status detection (installed / outdated / not installed) using the standardized version detection rules and Homebrew metadata.
- [x] Implement install/update/uninstall commands for individual software and for all items in a template.
- [x] Implement a `versions` command (or similar) that prints all detected software versions in one run.

_Phase 3 – GUI apps & Homebrew_
- [ ] Implement respectful installation of GUI apps via Homebrew casks (e.g. `brew install --cask android-studio`).
- [ ] Detect existing `.app` bundles via `mdls` / `mdfind` and offer skip/upgrade/reinstall, rather than overwriting silently.
- [ ] Ensure GUI adapters only manage application binaries (no user project/settings changes).

_Phase 4 – Onboarding & configurator_
- [ ] Implement a template-driven `onboard fresh` workflow that installs all software in the selected template in dependency order.
- [ ] Implement an `onboard update` workflow that updates all software defined in the template.
- [ ] Implement the configurator to list profiles and apply Git/SSH/GPG/email/username/default branch/pull mode settings.
- [ ] Add backup and dry-run support for configurator changes, mark as experimental, and gate it behind an explicit `--experimental-config` (or similar) flag.

_Phase 5 – Safety, testing & DX_
- [ ] Add a global dry-run flag to show planned actions without executing them.
- [ ] Add confirmation prompts for destructive operations (uninstall, major upgrades), with a `--yes` flag to skip prompts when desired.
- [ ] Add logging and a concise summary report per run (per software success/failure).
- [ ] Implement a small test harness to run per-software adapter tests and dependency-chain tests.
- [ ] Optionally add a `--json` output mode so other tools (including Codex) can consume results programmatically.
