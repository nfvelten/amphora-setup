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
    about = "Setup e gestão do ambiente PKMS amphora",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Wizard interativo de instalação
    Install,
    /// Verifica dependências do sistema
    Check,
    /// Atualiza scripts e configs no vault
    Update,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Install) => cmd_install(),
        Some(Commands::Check) => cmd_check(),
        Some(Commands::Update) => cmd_update(),
        None => {
            println!("{}", style("amphora — PKMS setup").bold().cyan());
            println!();
            println!("Comandos disponíveis:");
            println!(
                "  {}  {}",
                style("install").green().bold(),
                "Wizard interativo de instalação"
            );
            println!(
                "  {}    {}",
                style("check").green().bold(),
                "Verifica dependências do sistema"
            );
            println!(
                "  {}   {}",
                style("update").green().bold(),
                "Atualiza scripts e configs no vault"
            );
            println!();
            println!(
                "Use {} para mais detalhes.",
                style("amphora <comando> --help").dim()
            );
        }
    }
}

// ── install ───────────────────────────────────────────────────────────────────

fn cmd_install() {
    let theme = ColorfulTheme::default();

    println!();
    println!("{}", style("=== amphora install ===").bold().cyan());
    println!();

    let default_vault = format!("{}/amphora", home());
    let vault_path: String = Input::with_theme(&theme)
        .with_prompt("Caminho do vault Obsidian")
        .default(default_vault)
        .interact_text()
        .unwrap();

    let default_bin = format!("{}/.local/bin", home());
    let bin_dir: String = Input::with_theme(&theme)
        .with_prompt("Diretório para instalar scripts")
        .default(default_bin)
        .interact_text()
        .unwrap();

    let detected_sink = detect_sink();
    let sink_hint = detected_sink.as_deref().unwrap_or("não detectado");
    let sink: String = Input::with_theme(&theme)
        .with_prompt(format!(
            "Sink de áudio para meeting-record (detectado: {sink_hint})"
        ))
        .default(detected_sink.unwrap_or_default())
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let options = vec![
        "Scripts (meeting-record, video-note, daily-note...)",
        "Git hook global (post-commit → registra commits no vault)",
        "CLAUDE.md no vault (comandos /nota, /standup, /foco...)",
        ".obsidian config + Templates",
        "Tudo acima",
    ];

    let selection = Select::with_theme(&theme)
        .with_prompt("O que instalar?")
        .items(&options)
        .default(4)
        .interact()
        .unwrap();

    let install_scripts = matches!(selection, 0 | 4);
    let install_hook = matches!(selection, 1 | 4);
    let install_claude = matches!(selection, 2 | 4);
    let install_obsidian = matches!(selection, 3 | 4);

    println!();
    println!("{}", style("Resumo:").bold());
    println!("  Vault:   {}", style(&vault_path).cyan());
    println!("  Scripts: {}", style(&bin_dir).cyan());
    if !sink.is_empty() {
        println!("  Sink:    {}", style(&sink).cyan());
    }
    println!();

    if !Confirm::with_theme(&theme)
        .with_prompt("Confirmar instalação?")
        .default(true)
        .interact()
        .unwrap()
    {
        println!("{} Cancelado.", WARN);
        return;
    }

    println!();

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

    println!();
    println!("{}", style("=== Instalação concluída ===").bold().green());
    println!();
    println!("Próximos passos:");
    println!(
        "  {} Abra o vault no Obsidian — plugins serão baixados automaticamente",
        ARROW
    );
    println!("  {} Tema: github.com/nfvelten (mateCreations)", ARROW);
    println!("  {} Neovim: github.com/nfvelten/dotfiles", ARROW);
    println!();
}

// ── check ─────────────────────────────────────────────────────────────────────

fn cmd_check() {
    println!();
    println!("{}", style("=== Verificando dependências ===").bold().cyan());
    println!();

    let deps: &[(&str, &str)] = &[
        ("claude", "curl -fsSL https://claude.ai/install.sh | sh"),
        ("nvim", "gerenciador de pacotes do sistema"),
        ("python3", "gerenciador de pacotes do sistema"),
        ("yt-dlp", "pip install yt-dlp"),
        ("pw-record", "pipewire (gerenciador de pacotes do sistema)"),
        ("notify-send", "libnotify (gerenciador de pacotes do sistema)"),
        ("git", "gerenciador de pacotes do sistema"),
    ];

    let mut all_ok = true;

    for (dep, hint) in deps {
        if which::which(dep).is_ok() {
            println!("  {} {}", style(CHECK).green(), style(dep).bold());
        } else {
            println!(
                "  {} {} — {}",
                style(WARN).yellow(),
                style(dep).bold(),
                style(hint).dim()
            );
            all_ok = false;
        }
    }

    let whisper_ok = Command::new("python3")
        .args(["-c", "import faster_whisper"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if whisper_ok {
        println!("  {} {}", style(CHECK).green(), style("faster-whisper").bold());
    } else {
        println!(
            "  {} {} — {}",
            style(WARN).yellow(),
            style("faster-whisper").bold(),
            style("pip install faster-whisper").dim()
        );
        all_ok = false;
    }

    println!();

    if all_ok {
        println!("{} Todas as dependências encontradas.", style(CHECK).green());
    } else {
        println!(
            "{} Algumas dependências estão faltando. Instale-as antes de rodar {}.",
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
        .with_prompt("Caminho do vault")
        .default(default_vault)
        .interact_text()
        .unwrap();

    let default_bin = format!("{}/.local/bin", home());
    let bin_dir: String = Input::with_theme(&theme)
        .with_prompt("Diretório dos scripts")
        .default(default_bin)
        .interact_text()
        .unwrap();

    let options = vec!["Scripts", "CLAUDE.md", ".obsidian config + Templates", "Tudo"];
    let selection = Select::with_theme(&theme)
        .with_prompt("O que atualizar?")
        .items(&options)
        .default(3)
        .interact()
        .unwrap();

    println!();

    let root = repo_root();
    let vault = PathBuf::from(&vault_path);

    if matches!(selection, 0 | 3) {
        let sink = detect_sink().unwrap_or_default();
        install_scripts_to(&root, &PathBuf::from(&bin_dir), &vault_path, &sink);
    }
    if matches!(selection, 1 | 3) {
        install_claude_md(&root, &vault);
    }
    if matches!(selection, 2 | 3) {
        install_obsidian_config(&root, &vault);
    }

    println!();
    println!("{} Update concluído.", style(CHECK).green());
    println!();
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn home() -> String {
    std::env::var("HOME").unwrap_or_else(|_| ".".to_string())
}

fn repo_root() -> PathBuf {
    // Ao rodar do repo: cli/target/debug/amphora → sobe 3 níveis
    // Ao rodar instalado: usa env var AMPHORA_REPO ou o diretório do binário
    if let Ok(repo) = std::env::var("AMPHORA_REPO") {
        return PathBuf::from(repo);
    }
    std::env::current_exe()
        .unwrap()
        .parent().unwrap() // debug/ ou release/
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
    println!("{}", style("Instalando scripts...").bold());
    fs::create_dir_all(bin_dir).ok();

    let scripts_dir = repo.join("bin");
    if !scripts_dir.exists() {
        println!("  {} bin/ não encontrado", style(WARN).yellow());
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
    println!("{}", style("Configurando git hook global...").bold());

    let hook_dir = PathBuf::from(home()).join(".config/git/hooks");
    let dst = hook_dir.join("post-commit");
    fs::create_dir_all(&hook_dir).ok();

    if dst.exists() {
        println!(
            "  {} post-commit já existe — compare com git-hooks/post-commit se necessário",
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

    println!("  {} post-commit instalado", style(CHECK).green());
}

fn install_claude_md(repo: &Path, vault: &Path) {
    println!("{}", style("Instalando CLAUDE.md...").bold());

    let src = repo.join("claude/CLAUDE.md");
    let dst = vault.join("CLAUDE.md");

    if dst.exists() {
        println!(
            "  {} CLAUDE.md já existe — compare com claude/CLAUDE.md se necessário",
            style(WARN).yellow()
        );
        return;
    }

    fs::create_dir_all(vault).ok();
    fs::copy(&src, &dst).unwrap();
    println!("  {} CLAUDE.md copiado", style(CHECK).green());
}

fn install_obsidian_config(repo: &Path, vault: &Path) {
    println!("{}", style("Instalando .obsidian config...").bold());

    let obsidian_src = repo.join("vault/.obsidian");
    if obsidian_src.exists() {
        copy_dir_all(&obsidian_src, &vault.join(".obsidian"));
        println!("  {} .obsidian config instalado", style(CHECK).green());
    }

    let templates_src = repo.join("vault/Templates");
    if templates_src.exists() {
        copy_dir_all(&templates_src, &vault.join("Templates"));
        println!("  {} Templates instalados", style(CHECK).green());
    }
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
