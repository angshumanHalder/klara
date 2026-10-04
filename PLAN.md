# Klara: macOS terminal implementation plan

## Scope and working assumptions

- Target: macOS only, one desktop app, built part time by a developer who is still learning Rust.
- Use GPUI for the window, UI, and Metal-backed drawing. Validate terminal-style cell rendering and ligature shaping in a spike before committing to it.
- Use an existing terminal emulation crate for escape sequences, screen state, and PTY integration where it fits. Do not write a VT parser from scratch.
- A **tab** contains one or more split **panes**. Each pane owns one shell process and its terminal state.
- A **session** is a named set of tabs and panes. Its shells survive closing the app window and can be reattached after relaunch. A macOS user logout or reboot ends the shell processes; saved layout can still be restored with new shells.
- Agent statuses are the last priority. They require explicit events from Herdr, rather than guesses from terminal output.
- Initial release: local shells only. No SSH, collaboration, image protocol, plugin system, or cross-platform layer.

## Milestones and atomic steps

Each checkbox is a separately reviewable change. The day and week figures below are **reference engineering effort for an experienced Rust developer at 40 focused hours per week**, including a focused check of each change. They are not a calendar promise for this project. Use the part-time estimates in the time assessment below. Steps within a milestone are roughly sequential.

### 0. Feasibility and skeleton — 1–1.5 weeks

- [x] Initialize a single-package Cargo app target. (0.25 day)
- [ ] Verify `cargo build --locked --release` on macOS and add a GitHub Actions macOS build. (0.25 day)
- [x] Open a GPUI window on macOS, draw a fixed grid of cells, and measure redraw time while changing rows. (1 day)
- [ ] Spike monospaced font metrics, fallback glyphs, emoji, and common ligatures in a grid; define how shaped glyphs map back to cells for selection and cursor painting. (1–2 days)
- [ ] Build a small terminal core around a PTY and terminal emulation crate: start the user's shell, render its grid, send keystrokes, and resize it. Record dependency/API decisions. (1–2 days)
- [ ] Set a performance baseline and acceptance targets on representative Apple Silicon hardware: prompt/input latency, sustained output, idle CPU, and large scrollback memory. (0.5–1 day)

**Gate:** Continue with GPUI only after the grid and ligature spike produces correct cell boundaries and acceptable redraw behavior. If it fails, revise the renderer choice before implementing the UI around it.

### 1. Usable single-pane terminal — 2–3 weeks

- [ ] Start a login shell in a PTY and surface startup/exit errors in the UI. (1 day)
- [ ] Feed PTY output into terminal screen state off the UI thread; schedule bounded UI updates. (1–2 days)
- [ ] Paint visible cells, attributes, cursor, and selection from screen state. (2–3 days)
- [ ] Send typed text and standard key combinations to the PTY; handle macOS text input/IME. (1–2 days)
- [ ] Resize the PTY and reflow/redraw when the window or font metrics change. (1 day)
- [ ] Support scrollback and viewport scrolling with a configurable line limit. (1–2 days)
- [ ] Implement clipboard copy/paste with safe handling of bracketed paste. (1 day)
- [ ] Check common CLI programs (`zsh`, `vim`, `less`, `top`) and fix terminal mode regressions. (1–2 days)

### 2. Renderer and typography — 2–3 weeks

- [ ] Cache shaped text/glyph runs by content and style; invalidate only changed rows or regions. (2–3 days)
- [ ] Add ligature toggle, render common programming ligatures, and keep cursor, selection, and hit testing aligned to terminal cells. (2–3 days)
- [ ] Render combining marks, wide characters, emoji, underlines, bold/italic, and font fallback correctly. (2–3 days)
- [ ] Batch drawing and cap repaint rate under heavy PTY output. (1–2 days)
- [ ] Benchmark scrolling and high-volume output against the baseline; profile and fix the dominant bottleneck. (1–2 days)

### 3. Configuration — 1–1.5 weeks

- [ ] Define a small versioned TOML config: font family/size, ligatures, theme colors, shell, scrollback limit, and key bindings. (1 day)
- [ ] Load defaults plus the user's config; show file/line errors without preventing startup. (1 day)
- [ ] Apply theme colors and font size changes, including a quick font-size shortcut. (1–2 days)
- [ ] Add a documented config example and verify restart/reload behavior. (0.5–1 day)

### 4. Mouse, tabs, and panes — 2.5–3.5 weeks

- [ ] Translate pointer position into grid cell coordinates and implement drag selection, double click word selection, and scrolling. (1–2 days)
- [ ] Forward mouse buttons/motion/wheel only when the terminal application enables mouse reporting; preserve normal selection behavior otherwise. (1–2 days)
- [ ] Add tab create, close, switch, reorder, title, and keyboard shortcuts. (2–3 days)
- [ ] Show tabs and panes in a compact sidebar or floating panel with focus navigation. (1–2 days)
- [ ] Add horizontal/vertical splits, focus movement, resizing, and closing one pane without ending siblings. (2–3 days)
- [ ] Route input, selection, resize, and process exit to the focused pane; check nested panes under rapid changes. (1–2 days)

### 5. Sessions — 2–3 weeks

- [ ] Model named sessions and serialize their tab/pane layout and pane working directories. (1–2 days)
- [ ] Start a per-user background broker that owns PTYs independently of app windows. (2–3 days)
- [ ] Add local IPC to list, create, attach, detach, and terminate sessions; restrict access to the current user. (2–3 days)
- [ ] Reconnect after app relaunch and resynchronize each pane's current grid, scrollback, and process state. (2–3 days)
- [ ] Restore saved layouts after broker restart or reboot with fresh shells; explain that process state cannot survive reboot. (1–2 days)
- [ ] Test abnormal UI/broker exits and ensure stale sockets and child processes are cleaned up. (1–2 days)

### 6. Release hardening — 1.5–2.5 weeks

- [ ] Run a terminal compatibility pass covering alternate screen, colors, Unicode, clipboard, mouse protocols, and resize behavior. (2–3 days)
- [ ] Test long-running sessions, large scrollback, app relaunch, sleep/wake, multiple windows, and shutdown. (2–3 days)
- [ ] Add crash diagnostics, user-facing error messages, and a way to reset broken config. (1–2 days)
- [ ] Package and sign a macOS app; document install, configuration, keyboard shortcuts, session behavior, and known limits. (2–3 days)

### 7. Agent panel and Herdr integration (last priority) — 1.5–2.5 reference weeks

- [ ] Document Herdr's notification transport, event schema, agent identity, and lifecycle; make a sample event fixture. (0.5–1 day)
- [ ] Extend the tab/pane panel to list agents and focus their associated pane. (1–2 days)
- [ ] Ingest agent events and map them to `running`, `complete`, and `requires input`; preserve the last known state on disconnect. (2–3 days)
- [ ] Associate each agent with a pane/session where the event supplies an identifier; show unassociated agents separately. (1–2 days)
- [ ] Handle duplicate, delayed, missing, and out-of-order events; verify the panel remains responsive under rapid updates. (1–2 days)

## Time assessment

The core release milestones (0–6) total about 440–720 focused hours for an experienced Rust developer. For someone learning Rust while building this part time, use a provisional **1.5–2× learning and integration factor**. That gives these rough effort ranges:

| Outcome                                             | Estimated focused hours | At 10 hours/week | At 15 hours/week |
| --------------------------------------------------- | ----------------------: | ---------------: | ---------------: |
| First usable single-pane prototype (milestones 0–1) |             **180–360** |   **4–9 months** |   **3–6 months** |
| Core internal build (milestones 0–5)                |           **570–1,240** | **13–29 months** |  **9–19 months** |
| Core release candidate (milestones 0–6)             |           **660–1,440** | **15–33 months** | **10–22 months** |

Herdr agent statuses, left until last, add roughly **90–200 focused hours**, pending its event API. This is incremental and is not included in the core release rows above.

These are planning ranges, not deadlines. Use _focused build hours_, excluding weeks when no work happens. If you have only 5 hours per week, roughly double the 10-hour calendar figures; at 20 hours per week, roughly halve them. Re-estimate after the GPUI/PTY spike and the first working shell, using the time those steps actually took you.

Allow another **120–320 focused hours** of contingency if GPUI text shaping needs custom terminal-specific work or the chosen terminal core requires substantial adaptation. The renderer, PTY, and session broker are the largest sources of uncertainty.

The largest uncertainty is ligature shaping in a fixed cell grid: the glyph may span several cells while the terminal cursor and selection still operate per cell. Validate that in milestone 0, then revise the remaining estimate from the spike results.

## Technical references checked

- [GPUI README](https://github.com/zed-industries/zed/blob/main/crates/gpui/README.md): macOS Metal backend and current setup guidance; GPUI remains under active development.
- [alacritty_terminal PTY documentation](https://docs.rs/alacritty_terminal/latest/alacritty_terminal/tty/index.html): example of a Rust terminal core with PTY abstractions worth evaluating in the spike.
