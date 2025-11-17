# MazIQ

> **macOS Provisioning & Automation Tool**

A beautiful, interactive TUI for managing your macOS development environment.

---

## 🚧 Status: Migration in Progress

**We're rewriting MazIQ in Go + Bubbletea.**

The previous Rust/tui-realm implementation has been archived to `legacy/` (see `legacy/DO_NOT_USE.md` for details).

### Why Go?

- **Better fit:** MazIQ is a command wrapper, not a systems tool
- **Mature TUI:** Bubbletea is battle-tested (GitHub CLI, etc.)
- **Simpler:** Elm architecture > complex component lifecycle
- **Faster iteration:** Less time fighting frameworks, more time building features

See [`MIGRATION_PLAN.md`](./MIGRATION_PLAN.md) for the full migration plan.

---

## What is MazIQ?

MazIQ helps you:
- 📦 **Install** essential development tools via Homebrew, Cargo, NPM, etc.
- 🔄 **Update** installed software to latest versions
- ✅ **Check** installation status and versions across your system
- 📋 **Templates** for different dev environments (web, mobile, data science, etc.)
- 🧪 **E2E Testing** for package manager workflows

### Features

- **Interactive TUI** - Beautiful terminal interface with keyboard navigation
- **Multi-Source** - Supports Homebrew, Cargo, NPM, Go, Flutter, and more
- **Template-Based** - TOML files define software bundles
- **Streaming Logs** - Watch installations in real-time
- **Dry-Run Mode** - Preview what will happen without executing

---

## Quick Start (Coming Soon)

```bash
# Install
go install github.com/hmziqrs/maziq@latest

# Run interactive mode
maziq

# Use a template
maziq onboard --template hmziq

# Check status
maziq status
```

---

## Development

### Prerequisites
- Go 1.21+
- macOS (for now - Linux support planned)

### Build
```bash
go build -o maziq cmd/maziq/main.go
./maziq
```

### Project Structure
```
cmd/
  maziq/          # Entry point
internal/
  tui/            # Bubbletea UI
  catalog/        # Software definitions
  manager/        # Brew/version operations
  templates/      # Template loading
  workflow/       # Task execution
templates/        # TOML template files
legacy/           # Archived Rust code (DO NOT USE)
```

---

## Progress

See [`MIGRATION_PLAN.md`](./MIGRATION_PLAN.md) for detailed timeline.

- [x] Archive Rust implementation
- [x] Create migration plan
- [ ] Go project setup
- [ ] Port domain logic
- [ ] Build Bubbletea TUI
- [ ] Implement workflows
- [ ] Polish & release

---

## Contributing

**Not accepting contributions yet** - we're in the middle of a rewrite.

Check back in a week or two once the Go version is stable!

---

## License

MIT

---

## Legacy Code

The original Rust implementation is archived in `legacy/`.

**Do not use it.** It has been abandoned due to framework immaturity and complexity.

See `legacy/DO_NOT_USE.md` for the full story.
