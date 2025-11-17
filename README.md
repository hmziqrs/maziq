# MazIQ

> **macOS Provisioning & Automation Tool**

A beautiful, interactive TUI for managing your macOS development environment.

Built with Go + [Bubbletea](https://github.com/charmbracelet/bubbletea).

---

## Features

- 📦 **Install** essential development tools via Homebrew, Cargo, NPM, etc.
- 🔄 **Update** installed software to latest versions
- ✅ **Check** installation status and versions across your system
- 📋 **Templates** for different dev environments (web, mobile, data science, etc.)
- 🧪 **E2E Testing** for package manager workflows
- 🎨 **Beautiful TUI** with keyboard navigation

---

## Quick Start

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
- macOS (Linux support planned)

### Setup
```bash
# Clone
git clone https://github.com/hmziqrs/maziq.git
cd maziq

# Install dependencies
go mod download

# Run
go run cmd/maziq/main.go
```

### Project Structure
```
cmd/
  maziq/          # Entry point
internal/
  tui/            # Bubbletea UI components
  catalog/        # Software definitions
  manager/        # Package manager operations
  templates/      # Template loading
templates/        # TOML template files
```

---

## License

MIT
