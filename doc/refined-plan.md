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
