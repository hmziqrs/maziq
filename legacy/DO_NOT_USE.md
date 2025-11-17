# ⚠️ LEGACY CODE - DO NOT USE

This directory contains the **archived Rust implementation** of MazIQ.

## Status: DEPRECATED

**DO NOT use this code for any purpose.**

This implementation has been **abandoned** in favor of a Go rewrite using Bubbletea.

## Why Abandoned?

The Rust/tui-realm implementation suffered from:
- Immature TUI framework (tui-realm-stdlib 36% documented)
- Complex component model causing crashes and bugs
- Keyboard event handling issues requiring multiple rewrites
- Poor developer experience fighting framework quirks
- Overkill for a command-line wrapper tool

## What Happened Here?

We spent significant time:
- Fighting tui-realm's component lifecycle (components can't be cloned, must rebuild every frame)
- Fixing keyboard navigation (3+ iterations, still buggy)
- Working around stdlib component limitations
- Dealing with crashes from improper attr() usage

**None of this was using Rust's actual strengths.**

## Go Forward

See the new Go implementation in the parent directory using:
- **Bubbletea** - Mature, well-documented TUI framework
- **Charm Bubbles** - Rich component library
- **Lip Gloss** - Beautiful styling
- **Simpler model** - Elm architecture, easier to reason about

---

**Last commit:** November 17, 2025
**Reason for archival:** Wrong tool for the job
**Migration:** In progress to Go + Bubbletea
