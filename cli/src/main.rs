use clap::{Parser, Subcommand};
use console::{style, Emoji};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

static CHECK: Emoji<'_, '_> = Emoji("✓ ", "");
static WARN: Emoji<'_, '_> = Emoji("! ", "");
static ARROW: Emoji<'_, '_> = Emoji("→ ", "");

#[derive(Parser)]
#[command(
    name = "amphora",
    about = "Setup and management of the amphora PKMS environment",
    version,
    disable_help_subcommand = true
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Interactive installation wizard (or install a specific component)
    Install {
        /// Specific component: scripts, hook, claude, obsidian, nvim (optional — installs everything if omitted)
        component: Option<String>,
    },
    /// Check system dependencies
    Check,
    /// Update scripts and configs in the vault
    Update,
    /// Show detailed help for CLI commands
    Help {
        /// Specific command (optional)
        command: Option<String>,
    },
    /// Guide to amphora system features
    Guide {
        /// Specific topic: scripts, claude, hook, obsidian (optional)
        topic: Option<String>,
    },
    /// Remove installed scripts and configs
    Uninstall {
        /// Specific component: scripts, hook, claude, nvim (optional — shows menu if omitted)
        component: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Install { component }) => cmd_install(component.as_deref()),
        Some(Commands::Check) => cmd_check(),
        Some(Commands::Update) => cmd_update(),
        Some(Commands::Help { command }) => cmd_help(command.as_deref()),
        Some(Commands::Guide { topic }) => cmd_guide(topic.as_deref()),
        Some(Commands::Uninstall { component }) => cmd_uninstall(component.as_deref()),
        None => cmd_help(None),
    }
}

// ── banner ────────────────────────────────────────────────────────────────────

fn print_banner() {
    println!();
    println!("  {}", style("╔═╗╔╦╗╔═╗╦ ╦╔═╗╦═╗╔═╗").cyan().bold());
    println!("  {}", style("╠═╣║║║╠═╝╠═╣║ ║╠╦╝╠═╣").cyan().bold());
    println!("  {}", style("╩ ╩╩ ╩╩  ╩ ╩╚═╝╩╚═╩ ╩").cyan().bold());
    println!("  {}", style("personal knowledge management system").dim());
    println!();
}

// ── help ──────────────────────────────────────────────────────────────────────

fn cmd_help(command: Option<&str>) {
    match command {
        Some("install") => {
            println!();
            println!("{}", style("amphora install").bold().cyan());
            println!();
            println!("Interactive wizard that sets up the amphora environment from scratch.");
            println!("Accepts an optional component to install only a specific part.");
            println!();
            println!(
                "  {} Run {} first to check all dependencies are in place.",
                style("Tip:").bold(),
                style("amphora check").cyan()
            );
            println!();
            println!("{}", style("What it does (full wizard):").bold());
            println!("  {} Asks for the Obsidian vault path", ARROW);
            println!("  {} Asks for the scripts directory (~/.local/bin)", ARROW);
            println!("  {} Auto-detects the audio sink (PipeWire)", ARROW);
            println!("  {} Lets you choose what to install:", ARROW);
            println!("      - Automation scripts (meeting-record, video-note, daily-note...)");
            println!("      - Global git hook (logs commits to the vault daily note)");
            println!("      - CLAUDE.md (commands /note, /standup, /focus and others)");
            println!("      - .obsidian config + Templates");
            println!("      - Neovim plugins (obsidian.nvim, vault-tasks, keymaps)");
            println!();
            println!("{}", style("Usage:").bold());
            println!("  amphora install                # full wizard");
            println!("  amphora install scripts        # only automation scripts");
            println!("  amphora install hook           # only global git hook");
            println!("  amphora install claude         # only CLAUDE.md in the vault");
            println!("  amphora install obsidian       # only .obsidian config + Templates");
            println!("  amphora install nvim           # Neovim plugins (won't overwrite existing)");
            println!();
            println!("{}", style("Environment variables:").bold());
            println!("  AMPHORA_VAULT         Vault path (default: ~/amphora)");
            println!("  AMPHORA_SINK_MONITOR  Audio sink for meeting-record");
            println!("                        (auto-detected via pw-cli if not set)");
        }
        Some("check") => {
            println!();
            println!("{}", style("amphora check").bold().cyan());
            println!();
            println!("Checks whether all required dependencies are installed.");
            println!();
            println!("{}", style("Dependencies checked:").bold());
            println!("  claude         Claude Code CLI — AI engine");
            println!("  nvim           Neovim — terminal vault editing");
            println!("  python3        Required for meeting-transcribe and video-note");
            println!("  faster-whisper Local audio transcription (pip install faster-whisper)");
            println!("  yt-dlp         YouTube subtitle downloader (pip install yt-dlp)");
            println!("  pw-record      Audio recording via PipeWire (pipewire)");
            println!("  notify-send    Desktop notifications (libnotify)");
            println!("  git            Vault version control");
            println!();
            println!("{}", style("Usage:").bold());
            println!("  amphora check");
        }
        Some("update") => {
            println!();
            println!("{}", style("amphora update").bold().cyan());
            println!();
            println!("Updates already-installed scripts and configs after a git pull.");
            println!("Useful to sync changes without running the full installation wizard.");
            println!();
            println!("{}", style("What can be updated:").bold());
            println!("  Scripts          Copies new versions to ~/.local/bin");
            println!("  CLAUDE.md        Updates Claude Code commands in the vault");
            println!("  .obsidian config Updates Obsidian settings and templates");
            println!("  Neovim plugins   Adds new plugin files (skips existing ones)");
            println!();
            println!("{}", style("Usage:").bold());
            println!("  amphora update");
        }
        Some(other) => {
            println!();
            println!(
                "{} Unknown command: {}",
                style(WARN).yellow(),
                style(other).bold()
            );
            println!();
            println!(
                "Available commands: {}",
                style("install  check  update  help").cyan()
            );
        }
        None => {
            print_banner();
            println!("{}", style("Commands:").bold());
            println!(
                "  {}  {}",
                style("install").green().bold(),
                "Interactive installation wizard"
            );
            println!(
                "  {}    {}",
                style("check").green().bold(),
                "Check system dependencies"
            );
            println!(
                "  {}   {}",
                style("update").green().bold(),
                "Update scripts and configs in the vault"
            );
            println!(
                "  {}     {}",
                style("help").green().bold(),
                "Show detailed help for commands"
            );
            println!(
                "  {}{}",
                style("uninstall").green().bold(),
                "  Remove installed scripts and configs"
            );
            println!();
            println!("{}", style("Examples:").bold());
            println!("  amphora check                  # check dependencies before installing");
            println!("  amphora install                # full installation wizard");
            println!("  amphora install scripts        # install only the scripts");
            println!("  amphora install hook           # install only the git hook");
            println!("  amphora uninstall              # remove everything interactively");
            println!("  amphora help install           # detailed help for a specific command");
            println!();
            println!(
                "Repository: {}",
                style("github.com/nfvelten/amphora-setup").dim()
            );
        }
    }
    println!();
}

// ── guide ─────────────────────────────────────────────────────────────────────

fn cmd_guide(topic: Option<&str>) {
    match topic {
        Some("scripts") => {
            println!();
            println!("{}", style("Automation scripts").bold().cyan());
            println!();

            println!("{}", style("meeting-record").bold().green());
            println!("  Records system audio (monitor sink via PipeWire).");
            println!("  When stopped, transcribes with faster-whisper and sends");
            println!("  the transcript to Claude, which generates a structured summary");
            println!("  with context, decisions, next steps and participants.");
            println!("  The note is saved in Work/Meetings/ and linked in the daily note.");
            println!("  {}", style("Usage: keybind (e.g. Super+R to start/stop)").dim());
            println!();

            println!("{}", style("meeting-transcribe").bold().green());
            println!("  Transcribes an audio file using faster-whisper (medium model).");
            println!("  Used internally by meeting-record, but can be called directly.");
            println!("  {}", style("Usage: meeting-transcribe <file.wav>").dim());
            println!();

            println!("{}", style("video-note").bold().green());
            println!("  Takes a YouTube URL, downloads subtitles via yt-dlp,");
            println!("  cleans the VTT and sends to Claude to generate a summary");
            println!("  with main topic, key points and conclusion.");
            println!("  The note is saved in Personal/Videos/ and linked in the daily note.");
            println!("  {}", style("Usage: video-note <url>").dim());
            println!();

            println!("{}", style("daily-note").bold().green());
            println!("  Opens (or creates) today's daily note in Neovim via Hyprland scratchpad.");
            println!("  If the note doesn't exist, creates it with the full template: focus,");
            println!("  personal/work tasks, quick notes and note log.");
            println!("  {}", style("Requires: Hyprland + alacritty").dim());
            println!("  {}", style("Usage: keybind (e.g. Super+D)").dim());
            println!();

            println!("{}", style("vault-log-updates.sh").bold().green());
            println!("  Logs installed, updated or removed system packages");
            println!("  (via pacman) to Personal/System/Updates.md in the vault.");
            println!("  {}", style("Usage: called via system hook or manually").dim());
            println!();

            println!("{}", style("newsboat-save").bold().green());
            println!("  Takes a URL from newsboat, scrapes the article with rdrview,");
            println!("  sends the content to Claude for a structured summary");
            println!("  (main idea, key points, why it matters) and saves a note to the vault.");
            println!("  Links the note in today's daily note. Skips duplicates.");
            println!("  {}", style("Requires: rdrview or w3m, curl").dim());
            println!("  {}", style("Usage: bound to a key in ~/.newsboat/config").dim());
            println!();

            println!("{}", style("newsboat-save-bg").bold().green());
            println!("  Background wrapper for newsboat-save — runs it detached so");
            println!("  newsboat doesn't freeze while the article is being processed.");
            println!("  {}", style("Usage: macro-key in newsboat config calling newsboat-save-bg").dim());
            println!();

            println!("{}", style("claude-amphora").bold().green());
            println!("  Toggles a floating Claude Code scratchpad inside the vault (Hyprland).");
            println!("  On first press: opens alacritty with Claude Code in the vault directory.");
            println!("  On subsequent presses: shows/hides the existing window.");
            println!("  {}", style("Requires: Hyprland + alacritty").dim());
            println!("  {}", style("Usage: bind = SUPER, C, exec, claude-amphora").dim());
        }

        Some("claude") => {
            println!();
            println!("{}", style("Claude Code commands").bold().cyan());
            println!("  Available when Claude Code is open inside the vault.");
            println!();

            let commands = vec![
                ("/note",       "Quick knowledge capture — creates a structured note with context, links and backlinks"),
                ("/focus",      "Opens a deep work session with project context"),
                ("/standup",    "Daily meeting — logs what was done, what will be done and blockers"),
                ("/review",     "Daily reflection — what worked, what to improve, day highlight"),
                ("/week",       "Weekly planning — sets priorities and goals for the week"),
                ("/weekly",     "Weekly review — retrospective of what was delivered and learned"),
                ("/retro",      "Monthly retrospective — broader analysis of progress and adjustments"),
                ("/morning",    "Morning routine — opens the day with intention and task review"),
                ("/task",       "Logs a work task with context, priority and deadline"),
                ("/learning",   "Technical learning capture — creates a study note with concepts and references"),
                ("/idea",       "Quick idea capture — saves before it's lost without interrupting flow"),
                ("/brainstorm", "Brainstorming partner — explores ideas non-linearly"),
                ("/reading",    "Reading journal — logs impressions, quotes and insights from a book"),
                ("/log",        "Session log — documents what was done in a work session"),
                ("/context",    "Loads context from a specific project into the conversation"),
                ("/change",     "Creates a change management note for deploys or production changes"),
                ("/check",      "Task review — lists pending items and helps prioritize"),
            ];

            for (cmd, desc) in commands {
                println!("  {}  {}", style(cmd).green().bold(), desc);
            }
        }

        Some("hook") => {
            println!();
            println!("{}", style("Git hook — post-commit").bold().cyan());
            println!();
            println!("Global hook that runs automatically after every commit in any repository.");
            println!();
            println!("{}", style("What it does:").bold());
            println!("  {} Logs the commit to the vault daily note", ARROW);
            println!("      Format: hash · repo (branch): commit message");
            println!();
            println!("  {} Updates the corresponding project note in the vault", ARROW);
            println!("      Looks in Personal/Projects/ and Work/ for a note matching the repo name.");
            println!("      If found, appends the commit under ## Commits with Claude-generated");
            println!("      context (one sentence about the goal or impact of the change).");
            println!();
            println!("{}", style("Notes:").bold());
            println!("  - Commits in the vault itself (amphora) are ignored");
            println!("  - Claude context runs in background, doesn't block the commit");
            println!("  - The project note must exist to receive the log");
        }

        Some("obsidian") => {
            println!();
            println!("{}", style("Obsidian — configuration").bold().cyan());
            println!();
            println!("{}", style("Plugins:").bold());
            let plugins = vec![
                ("obsidian-git",             "Auto-backup every 1 min, auto-pull on boot, merge sync strategy"),
                ("dataview",                 "Task and note queries in daily notes"),
                ("templater-obsidian",       "Template engine: current date, weekday, prompts"),
                ("obsidian-tasks-plugin",    "Task management with queries, filters and dates"),
                ("calendar",                 "Daily note navigation via sidebar calendar"),
                ("obsidian-reminder-plugin", "Task reminders with desktop notifications"),
                ("typewriter-mode",          "Keeps the current line centered while writing"),
            ];
            for (p, desc) in plugins {
                println!("  {}  {}", style(p).green().bold(), desc);
            }
            println!();
            println!("{}", style("Included templates:").bold());
            println!("  Daily Notes      Full template with focus, tasks, log and dataview query");
            println!("  Study Note       Structure for technical study notes");
            println!("  Weekly Review    Weekly retrospective with task queries");
            println!("  Work Demand      Template for work demands with retrospective");
            println!("  Review           Template for movie, series and podcast reviews");
            println!();
            println!("{}", style("Theme:").bold());
            println!("  The vault works best with the mateCreations theme (Yerba Mate / Tererê).");
            println!("  {} github.com/nfvelten", ARROW);
        }

        Some("nvim") => {
            println!();
            println!("{}", style("Neovim — vault integration").bold().cyan());
            println!();
            println!("Three plugin files for LazyVim that connect Neovim to the vault.");
            println!("Installed to ~/.config/nvim/lua/plugins/ without overwriting existing files.");
            println!();

            println!("{}", style("obsidian.lua").bold().green());
            println!("  obsidian.nvim configured for the vault workspace.");
            println!("  {} Follow wiki-links with gf", style("·").dim());
            println!("  {} Toggle task checkbox with <leader>ch", style("·").dim());
            println!("  {} Hover preview of [[link]] under cursor with K", style("·").dim());
            println!("  {} snacks.nvim as picker", style("·").dim());
            println!("  {} render-markdown.nvim for rendering (UI disabled)", style("·").dim());
            println!();

            println!("{}", style("vault-tasks.lua").bold().green());
            println!("  Full vault workflow — all keymaps under <leader>v*");
            println!("  {} <leader>va  tasks — work", style("·").dim());
            println!("  {} <leader>vp  tasks — personal", style("·").dim());
            println!("  {} <leader>vd  tasks — today's daily note", style("·").dim());
            println!("  {} <leader>vf  fulltext search in vault", style("·").dim());
            println!("  {} <leader>vn  new note", style("·").dim());
            println!("  {} <leader>vb  backlinks for current note", style("·").dim());
            println!("  {} <leader>vm  navigate vault (all notes)", style("·").dim());
            println!("  {} <leader>vi  quick capture → Inbox", style("·").dim());
            println!("  {} <leader>vg  lazygit in vault", style("·").dim());
            println!("  {} [[          insert wikilink (insert mode)", style("·").dim());
            println!();

            println!("{}", style("vault-keymaps.lua").bold().green());
            println!("  Standalone keymaps, no plugin dependency.");
            println!("  {} <leader>od  open today's daily note", style("·").dim());
            println!("  {} <leader>ov  open vault root", style("·").dim());
            println!("  {} <leader>ot  add task to today's daily note", style("·").dim());
        }

        Some("omarchy") => {
            println!();
            println!("{}", style("Omarchy + mateCreations").bold().cyan());
            println!();
            println!("Amphora is designed to work alongside Omarchy — a pre-configured");
            println!("Linux desktop environment built around a consistent, distraction-free");
            println!("workflow. While amphora works on any system, Omarchy provides the");
            println!("tightest integration.");
            println!();
            println!("  {}", style("github.com/nicksoutram/omarchy").dim());
            println!();
            println!("{}", style("mateCreations themes:").bold());
            println!();
            println!(
                "  {}   {}",
                style("Yerba Mate").green().bold(),
                style("dark  — deep greens and earthy tones").dim()
            );
            println!(
                "  {}       {}",
                style("Tererê").yellow().bold(),
                style("light — warm, low-contrast cream palette").dim()
            );
            println!();
            println!("  Available for: Obsidian · Neovim · VS Code · Zen Browser · Hyprland");
            println!("  Automatic light/dark switching based on time of day (6h–18h light).");
            println!();
            println!("  {}", style("github.com/nfvelten (mateCreations)").dim());
            println!();
            println!("{}", style("Recommended Hyprland keybinds:").bold());
            println!("  bind = SUPER, C, exec, claude-amphora   # Claude Code scratchpad");
            println!("  bind = SUPER, N, exec, daily-note        # daily note scratchpad");
            println!("  bind = SUPER, R, exec, meeting-record    # start/stop recording");
            println!();
            println!("{}", style("Window rules (windowrules.conf):").bold());
            println!("  windowrulev2 = float, class:claude-amphora");
            println!("  windowrulev2 = size 1100 640, class:claude-amphora");
            println!("  windowrulev2 = workspace special:claude-amphora, class:claude-amphora");
            println!("  windowrulev2 = float, class:daily-note");
            println!("  windowrulev2 = size 1100 640, class:daily-note");
            println!("  windowrulev2 = workspace special:daily-note, class:daily-note");
        }

        Some(other) => {
            println!();
            println!(
                "{} Unknown topic: {}",
                style(WARN).yellow(),
                style(other).bold()
            );
            println!();
            println!(
                "Available topics: {}",
                style("scripts  claude  hook  obsidian  nvim  omarchy").cyan()
            );
        }

        None => {
            println!();
            println!("{}", style("amphora guide — system overview").bold().cyan());
            println!();
            println!("Amphora is a PKMS (Personal Knowledge Management System) built");
            println!("to reduce cognitive load by externalizing thoughts into structured");
            println!("notes, with automated capture and AI integration.");
            println!();
            println!("{}", style("Components:").bold());
            println!(
                "  {}        Automation scripts — recording, transcription, video ingestion",
                style("scripts").green().bold()
            );
            println!(
                "  {}         Claude Code commands in the vault — /note, /focus, /standup...",
                style("claude").green().bold()
            );
            println!(
                "  {}           Global git hook — logs commits to vault notes",
                style("hook").green().bold()
            );
            println!(
                "  {}        Obsidian configuration — plugins, templates and theme",
                style("obsidian").green().bold()
            );
            println!(
                "  {}          Neovim plugin files — vault-tasks, obsidian.nvim, keymaps",
                style("nvim").green().bold()
            );
            println!(
                "  {}        Omarchy integration + mateCreations themes",
                style("omarchy").green().bold()
            );
            println!();
            println!("{}", style("Examples:").bold());
            println!("  amphora guide scripts    # what each script does and how to use it");
            println!("  amphora guide claude     # list of available /cmd commands");
            println!("  amphora guide hook       # how the git hook works");
            println!("  amphora guide obsidian   # included plugins and templates");
            println!("  amphora guide nvim       # Neovim plugin files and keymaps");
            println!("  amphora guide omarchy    # Omarchy integration and mateCreations themes");
        }
    }
    println!();
}

// ── install ───────────────────────────────────────────────────────────────────

fn cmd_install(component: Option<&str>) {
    match component {
        Some("scripts") => cmd_install_scripts(),
        Some("hook") => cmd_install_hook(),
        Some("claude") => cmd_install_claude(),
        Some("obsidian") => cmd_install_obsidian(),
        Some("nvim") => cmd_install_nvim(),
        Some(other) => {
            println!();
            println!(
                "{} Unknown component: {}",
                style(WARN).yellow(),
                style(other).bold()
            );
            println!();
            println!(
                "Available components: {}",
                style("scripts  hook  claude  obsidian  nvim").cyan()
            );
            println!();
        }
        None => cmd_install_all(),
    }
}

fn cmd_install_all() {
    let theme = ColorfulTheme::default();

    print_banner();

    let default_vault = format!("{}/amphora", home());
    let vault_path: String = Input::with_theme(&theme)
        .with_prompt("Obsidian vault path")
        .default(default_vault)
        .interact_text()
        .unwrap();

    let default_bin = format!("{}/.local/bin", home());
    let bin_dir: String = Input::with_theme(&theme)
        .with_prompt("Scripts install directory")
        .default(default_bin)
        .interact_text()
        .unwrap();

    let detected_sink = detect_sink();
    let sink_hint = detected_sink.as_deref().unwrap_or("not detected");
    let sink: String = Input::with_theme(&theme)
        .with_prompt(format!(
            "Audio sink for meeting-record (detected: {sink_hint})"
        ))
        .default(detected_sink.unwrap_or_default())
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let options = vec![
        "Scripts (meeting-record, video-note, daily-note...)",
        "Global git hook (post-commit → logs commits to vault)",
        "CLAUDE.md in vault (/note, /standup, /focus and others)",
        ".obsidian config + Templates",
        "Neovim plugins (obsidian.nvim, vault-tasks, keymaps)",
        "All of the above",
    ];

    let selection = Select::with_theme(&theme)
        .with_prompt("What to install?")
        .items(&options)
        .default(5)
        .interact()
        .unwrap();

    let install_scripts = matches!(selection, 0 | 5);
    let install_hook = matches!(selection, 1 | 5);
    let install_claude = matches!(selection, 2 | 5);
    let install_obsidian = matches!(selection, 3 | 5);
    let install_nvim = matches!(selection, 4 | 5);

    let default_nvim = format!("{}/.config/nvim", home());
    let nvim_dir: String = if install_nvim {
        Input::with_theme(&theme)
            .with_prompt("Neovim config directory")
            .default(default_nvim)
            .interact_text()
            .unwrap()
    } else {
        default_nvim
    };

    println!();
    println!("{}", style("Summary:").bold());
    println!("  Vault:   {}", style(&vault_path).cyan());
    println!("  Scripts: {}", style(&bin_dir).cyan());
    if !sink.is_empty() {
        println!("  Sink:    {}", style(&sink).cyan());
    }
    if install_nvim {
        println!("  Neovim:  {}", style(&nvim_dir).cyan());
    }
    println!();

    if !Confirm::with_theme(&theme)
        .with_prompt("Confirm installation?")
        .default(true)
        .interact()
        .unwrap()
    {
        println!("{} Cancelled.", WARN);
        return;
    }

    warn_missing_deps(install_scripts, install_hook, install_nvim);

    let root = repo_root();
    let vault = PathBuf::from(&vault_path);

    if install_scripts {
        install_scripts_to(&root, &PathBuf::from(&bin_dir), &vault_path, &sink);
    }
    if install_hook {
        install_git_hook(&root, &vault_path);
    }
    if install_claude {
        install_claude_md(&root, &vault);
    }
    if install_obsidian {
        install_obsidian_config(&root, &vault);
    }
    if install_nvim {
        install_nvim_plugins(&root, &vault_path, &nvim_dir);
    }

    println!();
    println!("{}", style("=== Installation complete ===").bold().green());
    println!();
    println!("{}", style("Next steps:").bold());
    println!(
        "  {} Open the vault in Obsidian — plugins will be downloaded automatically",
        ARROW
    );
    println!("  {} Neovim dotfiles: github.com/nfvelten/dotfiles", ARROW);
    println!();
    println!("{}", style("Recommended environment:").bold());
    println!(
        "  {} Works best on {} — a pre-configured Linux desktop environment",
        ARROW,
        style("Omarchy").cyan().bold()
    );
    println!(
        "    {}",
        style("github.com/nicksoutram/omarchy").dim()
    );
    println!();
    println!("{}", style("Themes:").bold());
    println!(
        "  {} {} {} and {} {} — available for Obsidian, Neovim, VS Code and Zen Browser",
        ARROW,
        style("Yerba Mate").green().bold(),
        style("(dark)").dim(),
        style("Tererê").yellow().bold(),
        style("(light)").dim(),
    );
    println!(
        "    {} Automatic light/dark switching based on time of day",
        style("·").dim()
    );
    println!(
        "    {}",
        style("github.com/nfvelten (mateCreations)").dim()
    );
    println!();
}

fn cmd_install_scripts() {
    let theme = ColorfulTheme::default();

    println!();
    println!("{}", style("=== amphora install scripts ===").bold().cyan());
    println!();

    let default_vault = format!("{}/amphora", home());
    let vault_path: String = Input::with_theme(&theme)
        .with_prompt("Obsidian vault path")
        .default(default_vault)
        .interact_text()
        .unwrap();

    let default_bin = format!("{}/.local/bin", home());
    let bin_dir: String = Input::with_theme(&theme)
        .with_prompt("Scripts install directory")
        .default(default_bin)
        .interact_text()
        .unwrap();

    let detected_sink = detect_sink();
    let sink_hint = detected_sink.as_deref().unwrap_or("not detected");
    let sink: String = Input::with_theme(&theme)
        .with_prompt(format!(
            "Audio sink for meeting-record (detected: {sink_hint})"
        ))
        .default(detected_sink.unwrap_or_default())
        .allow_empty(true)
        .interact_text()
        .unwrap();

    println!();
    let root = repo_root();
    install_scripts_to(&root, &PathBuf::from(&bin_dir), &vault_path, &sink);
    println!();
    println!("{} Scripts installed.", style(CHECK).green());
    println!();
}

fn cmd_install_hook() {
    let theme = ColorfulTheme::default();

    println!();
    println!("{}", style("=== amphora install hook ===").bold().cyan());
    println!();

    let default_vault = format!("{}/amphora", home());
    let vault_path: String = Input::with_theme(&theme)
        .with_prompt("Obsidian vault path")
        .default(default_vault)
        .interact_text()
        .unwrap();

    println!();
    let root = repo_root();
    install_git_hook(&root, &vault_path);
    println!();
    println!("{} Git hook installed.", style(CHECK).green());
    println!();
}

fn cmd_install_claude() {
    let theme = ColorfulTheme::default();

    println!();
    println!("{}", style("=== amphora install claude ===").bold().cyan());
    println!();

    let default_vault = format!("{}/amphora", home());
    let vault_path: String = Input::with_theme(&theme)
        .with_prompt("Obsidian vault path")
        .default(default_vault)
        .interact_text()
        .unwrap();

    println!();
    let root = repo_root();
    let vault = PathBuf::from(&vault_path);
    install_claude_md(&root, &vault);
    println!();
    println!("{} CLAUDE.md installed.", style(CHECK).green());
    println!();
}

fn cmd_install_nvim() {
    let theme = ColorfulTheme::default();

    println!();
    println!("{}", style("=== amphora install nvim ===").bold().cyan());
    println!();

    let default_vault = format!("{}/amphora", home());
    let vault_path: String = Input::with_theme(&theme)
        .with_prompt("Obsidian vault path")
        .default(default_vault)
        .interact_text()
        .unwrap();

    let default_nvim = format!("{}/.config/nvim", home());
    let nvim_dir: String = Input::with_theme(&theme)
        .with_prompt("Neovim config directory")
        .default(default_nvim)
        .interact_text()
        .unwrap();

    println!();
    let root = repo_root();
    install_nvim_plugins(&root, &vault_path, &nvim_dir);
    println!();
    println!("{} Neovim plugins installed.", style(CHECK).green());
    println!();
    println!("Restart Neovim — Lazy will install the plugins automatically.");
    println!();
}

fn cmd_install_obsidian() {
    let theme = ColorfulTheme::default();

    println!();
    println!("{}", style("=== amphora install obsidian ===").bold().cyan());
    println!();

    let default_vault = format!("{}/amphora", home());
    let vault_path: String = Input::with_theme(&theme)
        .with_prompt("Obsidian vault path")
        .default(default_vault)
        .interact_text()
        .unwrap();

    println!();
    let root = repo_root();
    let vault = PathBuf::from(&vault_path);
    install_obsidian_config(&root, &vault);
    println!();
    println!("{} Obsidian config installed.", style(CHECK).green());
    println!();
}

// ── check ─────────────────────────────────────────────────────────────────────

fn cmd_check() {
    println!();
    println!("{}", style("=== Checking dependencies ===").bold().cyan());
    println!();

    // ── Required ──────────────────────────────────────────────────────────────
    println!("{}", style("Required:").bold());

    let required: &[(&str, &str)] = &[
        ("claude",      "curl -fsSL https://claude.ai/install.sh | sh"),
        ("git",         "system package manager"),
        ("python3",     "system package manager"),
        ("pw-record",   "pipewire (system package manager)"),
        ("notify-send", "libnotify (system package manager)"),
        ("yt-dlp",      "pip install yt-dlp"),
    ];

    let mut all_ok = true;

    for (dep, hint) in required {
        if dep_ok(dep) {
            println!("  {} {}", style(CHECK).green(), style(dep).bold());
        } else {
            println!(
                "  {} {}  {}",
                style(WARN).yellow(),
                style(dep).bold(),
                style(format!("→ {hint}")).dim()
            );
            all_ok = false;
        }
    }

    if dep_ok("python3") {
        if python_module_ok("faster_whisper") {
            println!("  {} {}", style(CHECK).green(), style("faster-whisper").bold());
        } else {
            println!(
                "  {} {}  {}",
                style(WARN).yellow(),
                style("faster-whisper").bold(),
                style("→ pip install faster-whisper").dim()
            );
            all_ok = false;
        }
    }

    println!();

    // ── Optional ──────────────────────────────────────────────────────────────
    println!("{}", style("Optional:").bold());

    let optional: &[(&str, &str, &str)] = &[
        ("nvim",      "system package manager",  "vault editing in Neovim"),
        ("hyprctl",   "Hyprland WM",             "claude-amphora and daily-note scratchpads"),
        ("alacritty", "system package manager",  "claude-amphora and daily-note scratchpads"),
        ("rdrview",   "github.com/nicksoutram/rdrview", "article extraction (newsboat-save)"),
        ("w3m",       "system package manager",  "fallback article extraction (newsboat-save)"),
        ("curl",      "system package manager",  "newsboat-save"),
        ("newsboat",  "system package manager",  "RSS reader integration"),
    ];

    for (dep, hint, purpose) in optional {
        if dep_ok(dep) {
            println!(
                "  {} {}  {}",
                style(CHECK).green(),
                style(dep).bold(),
                style(format!("— {purpose}")).dim()
            );
        } else {
            println!(
                "  {} {}  {}  {}",
                style("·").dim(),
                style(dep).dim(),
                style(format!("— {purpose}")).dim(),
                style(format!("(install: {hint})")).dim()
            );
        }
    }

    println!();

    // ── Environment ───────────────────────────────────────────────────────────
    println!("{}", style("Environment:").bold());

    let omarchy_ok = dep_ok("omarchy") || std::path::Path::new("/usr/local/share/omarchy").exists()
        || std::path::Path::new(&format!("{}/.local/share/omarchy", home())).exists();

    if omarchy_ok {
        println!(
            "  {} {}  {}",
            style(CHECK).green(),
            style("omarchy").bold(),
            style("— recommended Linux desktop environment").dim()
        );
    } else {
        println!(
            "  {} {}  {}",
            style("·").dim(),
            style("omarchy").dim(),
            style("— recommended for best integration (github.com/nicksoutram/omarchy)").dim()
        );
    }

    // Check for mateCreations theme in Obsidian
    let theme_path = format!("{}/amphora/.obsidian/themes/Omarchy/theme.css", home());
    if std::path::Path::new(&theme_path).exists() {
        println!(
            "  {} {}  {}",
            style(CHECK).green(),
            style("mateCreations theme").bold(),
            style("— Yerba Mate / Tererê").dim()
        );
    } else {
        println!(
            "  {} {}  {}",
            style("·").dim(),
            style("mateCreations theme").dim(),
            style("— Yerba Mate / Tererê (github.com/nfvelten)").dim()
        );
    }

    println!();

    if all_ok {
        println!("{} All required dependencies found.", style(CHECK).green());
    } else {
        println!(
            "{} Some required dependencies are missing. Install them before running {}.",
            style(WARN).yellow(),
            style("amphora install").bold()
        );
    }
    println!();
}

// ── update ────────────────────────────────────────────────────────────────────

fn cmd_update() {
    let theme = ColorfulTheme::default();

    println!();
    println!("{}", style("=== amphora update ===").bold().cyan());
    println!();

    let default_vault = format!("{}/amphora", home());
    let vault_path: String = Input::with_theme(&theme)
        .with_prompt("Vault path")
        .default(default_vault)
        .interact_text()
        .unwrap();

    let default_bin = format!("{}/.local/bin", home());
    let bin_dir: String = Input::with_theme(&theme)
        .with_prompt("Scripts directory")
        .default(default_bin)
        .interact_text()
        .unwrap();

    let options = vec![
        "Scripts",
        "CLAUDE.md",
        ".obsidian config + Templates",
        "Neovim plugins",
        "Everything",
    ];
    let selection = Select::with_theme(&theme)
        .with_prompt("What to update?")
        .items(&options)
        .default(4)
        .interact()
        .unwrap();

    let default_nvim = format!("{}/.config/nvim", home());
    let nvim_dir: String = if matches!(selection, 3 | 4) {
        Input::with_theme(&theme)
            .with_prompt("Neovim config directory")
            .default(default_nvim)
            .interact_text()
            .unwrap()
    } else {
        default_nvim
    };

    println!();

    let root = repo_root();
    let vault = PathBuf::from(&vault_path);

    if matches!(selection, 0 | 4) {
        let sink = detect_sink().unwrap_or_default();
        install_scripts_to(&root, &PathBuf::from(&bin_dir), &vault_path, &sink);
    }
    if matches!(selection, 1 | 4) {
        install_claude_md(&root, &vault);
    }
    if matches!(selection, 2 | 4) {
        install_obsidian_config(&root, &vault);
    }
    if matches!(selection, 3 | 4) {
        install_nvim_plugins(&root, &vault_path, &nvim_dir);
    }

    println!();
    println!("{} Update complete.", style(CHECK).green());
    println!();
}

// ── dep helpers ───────────────────────────────────────────────────────────────

fn dep_ok(name: &str) -> bool {
    which::which(name).is_ok()
}

fn python_module_ok(module: &str) -> bool {
    Command::new("python3")
        .args(["-c", &format!("import {module}")])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Checks deps relevant to the selected components and prints warnings.
/// Returns true if all required deps are present.
fn warn_missing_deps(
    install_scripts: bool,
    install_hook: bool,
    install_nvim: bool,
) {
    let mut missing: Vec<(&str, &str)> = vec![];
    let mut optional_missing: Vec<(&str, &str)> = vec![];

    // Core dep for everything
    if !dep_ok("claude") {
        missing.push(("claude", "curl -fsSL https://claude.ai/install.sh | sh"));
    }

    if install_scripts {
        if !dep_ok("pw-record") {
            missing.push(("pw-record", "pipewire (system package manager)"));
        }
        if !dep_ok("notify-send") {
            missing.push(("notify-send", "libnotify (system package manager)"));
        }
        if !dep_ok("python3") {
            missing.push(("python3", "system package manager"));
        } else if !python_module_ok("faster_whisper") {
            missing.push(("faster-whisper", "pip install faster-whisper"));
        }
        if !dep_ok("yt-dlp") {
            missing.push(("yt-dlp", "pip install yt-dlp"));
        }
        if !dep_ok("curl") {
            missing.push(("curl", "system package manager"));
        }
        // Optional for newsboat-save
        if !dep_ok("rdrview") && !dep_ok("w3m") {
            optional_missing.push(("rdrview / w3m", "article extraction — newsboat-save"));
        }
        // Optional for claude-amphora / daily-note
        if !dep_ok("hyprctl") {
            optional_missing.push(("hyprctl", "Hyprland — needed for claude-amphora and daily-note"));
        }
        if !dep_ok("alacritty") {
            optional_missing.push(("alacritty", "needed for claude-amphora and daily-note scratchpads"));
        }
    }

    if install_hook && !dep_ok("git") {
        missing.push(("git", "system package manager"));
    }

    if install_nvim && !dep_ok("nvim") {
        missing.push(("nvim", "system package manager"));
    }

    if missing.is_empty() && optional_missing.is_empty() {
        return;
    }

    println!();
    if !missing.is_empty() {
        println!("{}", style("Missing dependencies:").bold().yellow());
        for (dep, hint) in &missing {
            println!(
                "  {} {}  {}",
                style(WARN).yellow(),
                style(dep).bold(),
                style(format!("→ {hint}")).dim()
            );
        }
    }
    if !optional_missing.is_empty() {
        println!("{}", style("Optional (not installed):").bold().dim());
        for (dep, hint) in &optional_missing {
            println!(
                "  {} {}  {}",
                style("·").dim(),
                style(dep).dim(),
                style(format!("— {hint}")).dim()
            );
        }
    }
    if !missing.is_empty() {
        println!();
        println!(
            "  {} Some required dependencies are missing.",
            style(WARN).yellow()
        );
        println!("  Scripts may not work correctly until they are installed.");
    }
    println!();
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn home() -> String {
    std::env::var("HOME").unwrap_or_else(|_| ".".to_string())
}

fn repo_root() -> PathBuf {
    // When running from repo: cli/target/debug/amphora → go up 3 levels
    // When installed: use AMPHORA_REPO env var or binary directory
    if let Ok(repo) = std::env::var("AMPHORA_REPO") {
        return PathBuf::from(repo);
    }
    std::env::current_exe()
        .unwrap()
        .parent().unwrap() // debug/ or release/
        .parent().unwrap() // target/
        .parent().unwrap() // cli/
        .parent().unwrap() // repo root
        .to_path_buf()
}

fn detect_sink() -> Option<String> {
    let output = Command::new("pw-cli")
        .args(["list-objects"])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("alsa_output") && line.contains("monitor") {
            if let Some(start) = line.find("alsa_output") {
                let chunk = &line[start..];
                let end = chunk.find('"').unwrap_or(chunk.len());
                return Some(chunk[..end].to_string());
            }
        }
    }
    None
}

fn install_scripts_to(repo: &Path, bin_dir: &Path, vault: &str, sink: &str) {
    println!("{}", style("Installing scripts...").bold());
    fs::create_dir_all(bin_dir).ok();

    let scripts_dir = repo.join("bin");
    if !scripts_dir.exists() {
        println!("  {} bin/ not found", style(WARN).yellow());
        return;
    }

    for entry in fs::read_dir(&scripts_dir).unwrap().flatten() {
        let src = entry.path();
        let name = src.file_name().unwrap().to_string_lossy().to_string();
        let dst = bin_dir.join(&name);

        let mut content = fs::read_to_string(&src).unwrap_or_default();
        if !vault.is_empty() {
            content = content.replace(
                r#"VAULT="${AMPHORA_VAULT:-$HOME/amphora}""#,
                &format!(r#"VAULT="${{AMPHORA_VAULT:-{vault}}}""#),
            );
        }
        if !sink.is_empty() {
            content = content.replace(
                r#"SINK_MONITOR="${AMPHORA_SINK_MONITOR:-$(pw-cli list-objects 2>/dev/null | grep -o 'alsa_output[^"]*monitor' | head -1)}""#,
                &format!(r#"SINK_MONITOR="${{AMPHORA_SINK_MONITOR:-{sink}}}""#),
            );
        }

        fs::write(&dst, &content).unwrap();
        fs::set_permissions(&dst, fs::Permissions::from_mode(0o755)).unwrap();
        println!(
            "  {} {} {}",
            style(CHECK).green(),
            style(&name).bold(),
            style(format!("→ {}", dst.display())).dim()
        );
    }
}

fn install_git_hook(repo: &Path, vault: &str) {
    println!("{}", style("Setting up global git hook...").bold());

    let hook_dir = PathBuf::from(home()).join(".config/git/hooks");
    let dst = hook_dir.join("post-commit");
    fs::create_dir_all(&hook_dir).ok();

    if dst.exists() {
        println!(
            "  {} post-commit already exists — compare with git-hooks/post-commit if needed",
            style(WARN).yellow()
        );
        return;
    }

    let src = repo.join("git-hooks/post-commit");
    let mut content = fs::read_to_string(&src).unwrap_or_default();
    if !vault.is_empty() {
        content = content.replace(
            r#"VAULT="${AMPHORA_VAULT:-$HOME/amphora}""#,
            &format!(r#"VAULT="${{AMPHORA_VAULT:-{vault}}}""#),
        );
    }

    fs::write(&dst, &content).unwrap();
    fs::set_permissions(&dst, fs::Permissions::from_mode(0o755)).unwrap();
    Command::new("git")
        .args(["config", "--global", "core.hooksPath", hook_dir.to_str().unwrap()])
        .output()
        .ok();

    println!("  {} post-commit installed", style(CHECK).green());
}

fn install_claude_md(repo: &Path, vault: &Path) {
    println!("{}", style("Installing CLAUDE.md...").bold());

    let src = repo.join("claude/CLAUDE.md");
    let dst = vault.join("CLAUDE.md");

    if dst.exists() {
        println!(
            "  {} CLAUDE.md already exists — compare with claude/CLAUDE.md if needed",
            style(WARN).yellow()
        );
        return;
    }

    fs::create_dir_all(vault).ok();
    fs::copy(&src, &dst).unwrap();
    println!("  {} CLAUDE.md copied", style(CHECK).green());
}

fn install_obsidian_config(repo: &Path, vault: &Path) {
    println!("{}", style("Installing .obsidian config...").bold());

    let obsidian_src = repo.join("vault/.obsidian");
    if obsidian_src.exists() {
        copy_dir_all(&obsidian_src, &vault.join(".obsidian"));
        println!("  {} .obsidian config installed", style(CHECK).green());
    }

    let templates_src = repo.join("vault/Templates");
    if templates_src.exists() {
        copy_dir_all(&templates_src, &vault.join("Templates"));
        println!("  {} Templates installed", style(CHECK).green());
    }
}

fn install_nvim_plugins(repo: &Path, vault: &str, nvim_dir: &str) {
    println!("{}", style("Installing Neovim plugins...").bold());

    let src_dir = repo.join("nvim/lua/plugins");
    if !src_dir.exists() {
        println!("  {} nvim/lua/plugins/ not found", style(WARN).yellow());
        return;
    }

    let dst_dir = PathBuf::from(nvim_dir).join("lua/plugins");
    fs::create_dir_all(&dst_dir).ok();

    for entry in fs::read_dir(&src_dir).unwrap().flatten() {
        let src = entry.path();
        if src.extension().and_then(|e| e.to_str()) != Some("lua") {
            continue;
        }
        let name = src.file_name().unwrap().to_string_lossy().to_string();
        let dst = dst_dir.join(&name);

        if dst.exists() {
            println!(
                "  {} {} — already exists, skipping",
                style(WARN).yellow(),
                style(&name).bold()
            );
            continue;
        }

        let content = fs::read_to_string(&src)
            .unwrap_or_default()
            .replace("~/amphora", vault);

        fs::write(&dst, &content).unwrap();
        println!(
            "  {} {} {}",
            style(CHECK).green(),
            style(&name).bold(),
            style(format!("→ {}", dst.display())).dim()
        );
    }
}

// ── uninstall ─────────────────────────────────────────────────────────────────

fn cmd_uninstall(component: Option<&str>) {
    let theme = ColorfulTheme::default();

    println!();
    println!("{}", style("=== amphora uninstall ===").bold().cyan());
    println!();

    let scripts = vec![
        "meeting-record",
        "meeting-transcribe",
        "video-note",
        "daily-note",
        "newsboat-save",
        "newsboat-save-bg",
        "claude-amphora",
        "vault-log-updates.sh",
    ];

    let nvim_plugins = vec!["obsidian.lua", "vault-tasks.lua", "vault-keymaps.lua"];

    match component {
        Some("scripts") => {
            let default_bin = format!("{}/.local/bin", home());
            let bin_dir: String = Input::with_theme(&theme)
                .with_prompt("Scripts directory")
                .default(default_bin)
                .interact_text()
                .unwrap();

            if !Confirm::with_theme(&theme)
                .with_prompt(format!("Remove {} scripts from {}?", scripts.len(), bin_dir))
                .default(false)
                .interact()
                .unwrap()
            {
                println!("{} Cancelled.", WARN);
                return;
            }

            let bin = PathBuf::from(&bin_dir);
            for name in &scripts {
                let path = bin.join(name);
                if path.exists() {
                    fs::remove_file(&path).ok();
                    println!("  {} {} removed", style(CHECK).green(), style(name).bold());
                } else {
                    println!("  {} {} — not found", style("·").dim(), style(name).dim());
                }
            }
        }

        Some("hook") => {
            let hook_path = PathBuf::from(home()).join(".config/git/hooks/post-commit");
            if !hook_path.exists() {
                println!("{} post-commit hook not found.", WARN);
                return;
            }
            if !Confirm::with_theme(&theme)
                .with_prompt(format!("Remove {}?", hook_path.display()))
                .default(false)
                .interact()
                .unwrap()
            {
                println!("{} Cancelled.", WARN);
                return;
            }
            fs::remove_file(&hook_path).ok();
            println!("  {} post-commit removed", style(CHECK).green());
        }

        Some("claude") => {
            let default_vault = format!("{}/amphora", home());
            let vault_path: String = Input::with_theme(&theme)
                .with_prompt("Vault path")
                .default(default_vault)
                .interact_text()
                .unwrap();
            let claude_md = PathBuf::from(&vault_path).join("CLAUDE.md");
            if !claude_md.exists() {
                println!("{} CLAUDE.md not found in vault.", WARN);
                return;
            }
            if !Confirm::with_theme(&theme)
                .with_prompt(format!("Remove {}?", claude_md.display()))
                .default(false)
                .interact()
                .unwrap()
            {
                println!("{} Cancelled.", WARN);
                return;
            }
            fs::remove_file(&claude_md).ok();
            println!("  {} CLAUDE.md removed", style(CHECK).green());
        }

        Some("nvim") => {
            let default_nvim = format!("{}/.config/nvim", home());
            let nvim_dir: String = Input::with_theme(&theme)
                .with_prompt("Neovim config directory")
                .default(default_nvim)
                .interact_text()
                .unwrap();

            if !Confirm::with_theme(&theme)
                .with_prompt(format!("Remove {} plugin files from {}?", nvim_plugins.len(), nvim_dir))
                .default(false)
                .interact()
                .unwrap()
            {
                println!("{} Cancelled.", WARN);
                return;
            }

            let plugins_dir = PathBuf::from(&nvim_dir).join("lua/plugins");
            for name in &nvim_plugins {
                let path = plugins_dir.join(name);
                if path.exists() {
                    fs::remove_file(&path).ok();
                    println!("  {} {} removed", style(CHECK).green(), style(name).bold());
                } else {
                    println!("  {} {} — not found", style("·").dim(), style(name).dim());
                }
            }
        }

        Some(other) => {
            println!(
                "{} Unknown component: {}",
                style(WARN).yellow(),
                style(other).bold()
            );
            println!();
            println!(
                "Available components: {}",
                style("scripts  hook  claude  nvim").cyan()
            );
            println!();
            return;
        }

        None => {
            let options = vec![
                "Scripts (from ~/.local/bin)",
                "Global git hook (post-commit)",
                "CLAUDE.md (from vault)",
                "Neovim plugins",
                "Everything",
            ];
            let selection = Select::with_theme(&theme)
                .with_prompt("What to uninstall?")
                .items(&options)
                .default(0)
                .interact()
                .unwrap();

            let do_scripts  = matches!(selection, 0 | 4);
            let do_hook     = matches!(selection, 1 | 4);
            let do_claude   = matches!(selection, 2 | 4);
            let do_nvim     = matches!(selection, 3 | 4);

            let default_bin   = format!("{}/.local/bin", home());
            let default_vault = format!("{}/amphora", home());
            let default_nvim  = format!("{}/.config/nvim", home());

            let bin_dir = if do_scripts {
                Input::with_theme(&theme)
                    .with_prompt("Scripts directory")
                    .default(default_bin)
                    .interact_text()
                    .unwrap()
            } else { default_bin };

            let vault_path = if do_claude {
                Input::with_theme(&theme)
                    .with_prompt("Vault path")
                    .default(default_vault)
                    .interact_text()
                    .unwrap()
            } else { default_vault };

            let nvim_dir = if do_nvim {
                Input::with_theme(&theme)
                    .with_prompt("Neovim config directory")
                    .default(default_nvim)
                    .interact_text()
                    .unwrap()
            } else { default_nvim };

            println!();
            if !Confirm::with_theme(&theme)
                .with_prompt("Confirm uninstall?")
                .default(false)
                .interact()
                .unwrap()
            {
                println!("{} Cancelled.", WARN);
                return;
            }
            println!();

            if do_scripts {
                println!("{}", style("Removing scripts...").bold());
                let bin = PathBuf::from(&bin_dir);
                for name in &scripts {
                    let path = bin.join(name);
                    if path.exists() {
                        fs::remove_file(&path).ok();
                        println!("  {} {} removed", style(CHECK).green(), style(name).bold());
                    }
                }
            }

            if do_hook {
                println!("{}", style("Removing git hook...").bold());
                let hook_path = PathBuf::from(home()).join(".config/git/hooks/post-commit");
                if hook_path.exists() {
                    fs::remove_file(&hook_path).ok();
                    println!("  {} post-commit removed", style(CHECK).green());
                } else {
                    println!("  {} post-commit — not found", style("·").dim());
                }
            }

            if do_claude {
                println!("{}", style("Removing CLAUDE.md...").bold());
                let claude_md = PathBuf::from(&vault_path).join("CLAUDE.md");
                if claude_md.exists() {
                    fs::remove_file(&claude_md).ok();
                    println!("  {} CLAUDE.md removed", style(CHECK).green());
                } else {
                    println!("  {} CLAUDE.md — not found", style("·").dim());
                }
            }

            if do_nvim {
                println!("{}", style("Removing Neovim plugins...").bold());
                let plugins_dir = PathBuf::from(&nvim_dir).join("lua/plugins");
                for name in &nvim_plugins {
                    let path = plugins_dir.join(name);
                    if path.exists() {
                        fs::remove_file(&path).ok();
                        println!("  {} {} removed", style(CHECK).green(), style(name).bold());
                    }
                }
            }

            println!();
            println!("{} Uninstall complete.", style(CHECK).green());
        }
    }
    println!();
}

fn copy_dir_all(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).ok();
    for entry in fs::read_dir(src).unwrap().flatten() {
        let s = entry.path();
        let d = dst.join(entry.file_name());
        if s.is_dir() {
            copy_dir_all(&s, &d);
        } else {
            fs::copy(&s, &d).ok();
        }
    }
}
