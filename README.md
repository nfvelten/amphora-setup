# amphora-setup

Complete setup for my PKMS (Personal Knowledge Management System) — an integrated environment for capturing, processing and retrieving knowledge, built around Obsidian, Claude Code and Neovim.

The goal is to reduce cognitive load by externalizing as much as possible: meetings are transcribed and summarized automatically, videos become notes, commits are logged to the vault, and Claude Code operates directly in the vault with custom commands for knowledge management.

---

## What's included

### Scripts (`bin/`)

| Script | What it does |
|---|---|
| `meeting-record` | Records system audio (monitor sink), transcribes with Whisper and generates a summary + action items via Claude. Toggle with a keybind. |
| `meeting-transcribe` | Audio transcription with `faster-whisper` (medium model). Used by `meeting-record`. |
| `video-note` | Takes a YouTube URL, extracts subtitles, summarizes with Claude and saves a note to the vault. |
| `vault-log-updates.sh` | Logs installed/updated/removed system packages to the vault. |
| `newsboat-save` | Takes a URL from newsboat, scrapes the article, summarizes with Claude and saves a note to the vault. |
| `newsboat-save-bg` | Background wrapper for `newsboat-save` — keeps newsboat responsive while the article is processed. |

### Git hook (`git-hooks/post-commit`)

Global hook that runs after every commit in any repository:
- Logs the commit to the vault daily note with hash, repo and branch
- Updates the corresponding project note in the vault with Claude-generated context

### Claude Code (`claude/CLAUDE.md`)

`CLAUDE.md` installed in the vault with custom commands for Claude Code to operate in the vault:

- `/note` — quick knowledge capture
- `/focus` — deep work session
- `/standup` — daily meeting
- `/review` — daily reflection
- `/week` — weekly planning
- `/weekly` — weekly review
- `/retro` — monthly retrospective
- `/morning` — morning routine
- `/task` — task logging
- `/learning` — technical learning capture
- `/idea` — quick idea capture
- `/brainstorm` — brainstorming partner
- `/reading` — reading journal
- `/log` — session log
- `/context` — project context
- `/change` — change management note
- `/check` — task review

### Neovim (`nvim/`)

Plugin files for LazyVim that integrate Neovim with the vault:

- `obsidian.lua` — obsidian.nvim config with workspace, pt-BR daily notes, `K` hover preview, `[[` wikilink autocomplete
- `vault-tasks.lua` — task picker, quick capture, backlinks, all `<leader>v*` keymaps
- `vault-keymaps.lua` — `<leader>od` (daily note), `<leader>ov` (open vault), `<leader>ot` (add task)

---

## Requirements

- **Claude Code** — `curl -fsSL https://claude.ai/install.sh | sh`
- **Python 3** + `faster-whisper` — `pip install faster-whisper`
- **yt-dlp** — `pip install yt-dlp`
- **PipeWire** (`pw-record`) — for meeting recording
- **Obsidian** — vault at `~/amphora` (or set `AMPHORA_VAULT`)
- **Neovim** — recommended for vault editing
- **libnotify** (`notify-send`) — for desktop notifications
- **rdrview** or **w3m** — article extraction for `newsboat-save`
- **newsboat** — only required for `newsboat-save` integration

### Obsidian plugins

- `obsidian-git` — auto-backup every minute
- `dataview` — task and note queries
- `templater-obsidian` — templates for daily notes
- `obsidian-tasks-plugin` — task management
- `calendar` — daily note navigation
- `obsidian-reminder-plugin` — task reminders

---

## CLI

The setup is managed by the `amphora` CLI, written in Rust.

```
amphora install    Interactive installation wizard
amphora check      Check system dependencies
amphora update     Update scripts and configs in the vault
```

### Installing the CLI

```bash
git clone https://github.com/nfvelten/amphora-setup
cd amphora-setup/cli
cargo build --release
cp target/release/amphora ~/.local/bin/amphora
```

### Usage

```bash
# Check dependencies before installing
amphora check

# Interactive wizard — asks for vault path, audio sink, what to install
amphora install

# Install a specific component
amphora install scripts
amphora install hook
amphora install claude
amphora install obsidian
amphora install nvim

# Update scripts/configs after a pull
amphora update
```

The wizard auto-detects the audio sink via `pw-cli` and pre-fills defaults. Everything is configurable via prompts — nothing is assumed.

### Guide

The `guide` command documents each part of the system:

```bash
amphora guide                # overview of all components
amphora guide scripts        # what each script does and how to use it
amphora guide claude         # list of available /cmd commands
amphora guide hook           # how the git hook works
amphora guide obsidian       # included plugins and templates
amphora guide nvim           # Neovim plugin files and keymaps
amphora guide omarchy        # Omarchy integration and mateCreations themes
```

---

## Neovim integration

The Neovim configuration (including vault integration via `oil.nvim`, `telescope` and wiki-link navigation) is available in the dotfiles repository:

→ [github.com/nfvelten/dotfiles](https://github.com/nfvelten/dotfiles)

---

## Theme

The entire environment uses the **mateCreations** theme (Yerba Mate / Tererê) — available for Obsidian, Neovim, VS Code and Zen Browser, with automatic light/dark switching based on time of day:

→ [github.com/nfvelten](https://github.com/nfvelten)
