# MazIQ Migration Plan: Rust → Go

**Status:** Planning Phase
**Decision Date:** November 17, 2025
**Target:** Go + Bubbletea TUI Framework

---

## Executive Summary

Migrating from Rust/tui-realm to Go/Bubbletea for the following reasons:

### Why Migrate?

**Wrong tool for the job:**
- MazIQ is a **command wrapper** - it executes `brew`, checks versions, manages installations
- No performance-critical code, no systems programming, no need for Rust's safety guarantees
- Fighting framework immaturity instead of building features

**Framework pain points:**
- tui-realm: immature, poorly documented (36% coverage)
- Components can't be cloned, must rebuild every frame
- Keyboard events broken across 3+ rewrites
- Complex lifecycle management for simple UI

**Go advantages:**
- Bubbletea: mature, heavily used (GitHub CLI, etc.)
- Excellent documentation and examples
- Elm architecture: simple, predictable
- Faster iteration, simpler codebase

---

## Architecture Comparison

### Current (Rust/tui-realm)

```
src/
├── components/         # tui-realm components
│   ├── menu.rs        # Complex event handling
│   ├── software.rs    # Table rebuilding every frame
│   └── e2e_test.rs    # Manual widget creation
├── realm_app.rs       # Framework boilerplate
├── messages.rs        # Message passing
├── catalog.rs         # Software definitions (PORT THIS)
├── manager.rs         # Brew wrapper (PORT THIS)
├── templates.rs       # Template system (PORT THIS)
└── app.rs            # Business logic (PORT THIS)
```

### Target (Go/Bubbletea)

```
cmd/
└── maziq/
    └── main.go         # Entry point

internal/
├── tui/
│   ├── app.go          # Main Bubble Tea app
│   ├── menu.go         # Menu screen model
│   ├── catalog.go      # Catalog screen model
│   └── e2e.go          # E2E test screen model
├── catalog/
│   ├── software.go     # Software definitions
│   └── catalog.go      # Catalog management
├── manager/
│   ├── brew.go         # Homebrew operations
│   └── version.go      # Version checking
├── templates/
│   └── loader.go       # TOML template loading
└── workflow/
    └── executor.go     # Task execution

templates/              # TOML files (KEEP AS-IS)
└── hmziq.toml
```

---

## Migration Phases

### Phase 1: Project Setup (Day 1, 2-3 hours)

**Goal:** Working Go project with basic structure

- [x] Archive Rust code to `legacy/`
- [ ] Initialize Go module: `go mod init github.com/hmziqrs/maziq`
- [ ] Install dependencies:
  ```bash
  go get github.com/charmbracelet/bubbletea
  go get github.com/charmbracelet/bubbles
  go get github.com/charmbracelet/lipgloss
  go get github.com/pelletier/go-toml/v2
  ```
- [ ] Create directory structure
- [ ] Hello World Bubbletea app

**Deliverable:** `go run cmd/maziq/main.go` shows a basic TUI

---

### Phase 2: Core Domain Logic (Day 1-2, 4-6 hours)

**Goal:** Port business logic without UI

#### 2.1 Software Catalog
```go
// internal/catalog/software.go
type Software struct {
    ID          string
    Name        string
    Category    string
    Summary     string
    Installer   InstallerType
    VersionCmd  string
}

type InstallerType int
const (
    Homebrew InstallerType = iota
    Cargo
    NPM
    // ... etc
)
```

Port from: `src/catalog.rs` (lines 1-800)

#### 2.2 Template System
```go
// internal/templates/loader.go
type Template struct {
    Name        string     `toml:"name"`
    Description string     `toml:"description"`
    Software    []Software `toml:"software"`
}

func LoadTemplate(path string) (*Template, error)
func LoadAll(dir string) ([]*Template, error)
```

Port from: `src/templates.rs`

#### 2.3 Software Manager
```go
// internal/manager/brew.go
type Manager interface {
    CheckStatus(software Software) (Status, error)
    Install(software Software) error
    Update(software Software) error
    Uninstall(software Software) error
}

type Status struct {
    Installed bool
    Version   string
    State     StatusState
}
```

Port from: `src/manager.rs`

**Deliverable:** CLI tool that lists software, checks statuses (no TUI yet)

---

### Phase 3: Basic TUI (Day 2-3, 6-8 hours)

**Goal:** Menu + Software Catalog screens with navigation

#### 3.1 Bubbletea App Structure
```go
// internal/tui/app.go
type Model struct {
    screen      Screen
    menu        menu.Model
    catalog     catalog.Model
    width       int
    height      int
}

func (m Model) Init() tea.Cmd
func (m Model) Update(msg tea.Msg) (tea.Model, tea.Cmd)
func (m Model) View() string
```

#### 3.2 Menu Screen
```go
// internal/tui/menu.go
type Model struct {
    list     list.Model  // from bubbles
    selected int
}

// Elm architecture - simple!
func (m Model) Update(msg tea.Msg) (Model, tea.Cmd) {
    switch msg := msg.(type) {
    case tea.KeyMsg:
        switch msg.String() {
        case "up", "k":
            m.selected--
        case "down", "j":
            m.selected++
        }
    }
    return m, nil
}
```

#### 3.3 Software Catalog Screen
```go
// internal/tui/catalog.go
type Model struct {
    table    table.Model  // from bubbles
    software []catalog.Software
    statuses map[string]manager.Status
}
```

**Deliverable:** Working menu + catalog navigation (no actions yet)

---

### Phase 4: Actions & Tasks (Day 3-4, 6-8 hours)

**Goal:** Install/update/uninstall with live progress

#### 4.1 Task Runner
```go
// internal/workflow/executor.go
type Task struct {
    ID      int
    Label   string
    Action  Action
    Status  TaskStatus
    Output  []string
}

func (e *Executor) Run(task Task) <-chan TaskEvent
```

#### 4.2 Live Updates
```go
// Use Bubbletea's Cmd system for async tasks
func installCmd(sw Software) tea.Cmd {
    return func() tea.Msg {
        // Run brew install in background
        // Return InstallCompleteMsg
    }
}
```

#### 4.3 Streaming Logs
```go
// internal/tui/logs.go
type Model struct {
    viewport viewport.Model  // from bubbles
    lines    []string
}

// Updates in real-time as tasks run
```

**Deliverable:** Full workflow execution with live logs

---

### Phase 5: Polish (Day 4-5, 4-6 hours)

**Goal:** Production-ready UX

- [ ] Styling with Lip Gloss (colors, borders, padding)
- [ ] Status indicators (spinners, progress bars)
- [ ] Error handling and user feedback
- [ ] Help screens
- [ ] E2E test workflow
- [ ] Template selection UI

**Deliverable:** Feature parity with Rust version + better UX

---

## What to Port vs. Rewrite

### Port Directly (domain logic)
✅ Software catalog definitions (`catalog.rs` → `catalog/software.go`)
✅ Template TOML structure (already external, just change parser)
✅ Brew command execution (`manager.rs` → `manager/brew.go`)
✅ Version checking logic
✅ Workflow definitions

### Rewrite from Scratch (UI)
🔄 All TUI components (tui-realm → Bubbletea)
🔄 Event handling (much simpler in Bubbletea)
🔄 Screen management (Elm architecture)
🔄 Task execution (use Go channels, not Rust mpsc)

### Keep As-Is
📁 `templates/*.toml` files
📁 `.gitignore`
📁 License

---

## Timeline Estimate

| Phase | Duration | Cumulative |
|-------|----------|------------|
| 1. Setup | 2-3 hours | 3h |
| 2. Domain Logic | 4-6 hours | 9h |
| 3. Basic TUI | 6-8 hours | 17h |
| 4. Actions | 6-8 hours | 25h |
| 5. Polish | 4-6 hours | 31h |

**Total: 3-4 days of focused work**

Compare to: Unknown weeks fighting tui-realm bugs

---

## Success Criteria

✅ All features from Rust version working
✅ Keyboard navigation works reliably
✅ Streaming logs during status refresh
✅ Install/update/uninstall workflows
✅ Template support
✅ E2E testing workflow
✅ Better UX than Rust version
✅ Code is maintainable and extensible

---

## Risk Mitigation

**Risk:** Missing Rust-specific features
**Mitigation:** MazIQ doesn't use Rust-specific features (no unsafe, no custom allocators, etc.)

**Risk:** Go learning curve
**Mitigation:** Go is simpler than Rust; Bubbletea has excellent tutorials

**Risk:** Bubbletea limitations
**Mitigation:** Bubbletea powers GitHub CLI and many production tools - proven at scale

---

## Next Steps

1. ✅ Archive Rust code
2. ⏳ Run `go mod init`
3. ⏳ Create `cmd/maziq/main.go` with hello world
4. ⏳ Install Charm dependencies
5. ⏳ Build first Bubbletea screen

**Let's build this the right way.**
