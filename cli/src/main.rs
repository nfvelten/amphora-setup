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
    version,
    disable_help_subcommand = true
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
    /// Mostra ajuda detalhada dos comandos da CLI
    Help {
        /// Comando específico (opcional)
        command: Option<String>,
    },
    /// Guia das features do sistema amphora
    Guide {
        /// Tópico específico: scripts, claude, hook, obsidian (opcional)
        topic: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Install) => cmd_install(),
        Some(Commands::Check) => cmd_check(),
        Some(Commands::Update) => cmd_update(),
        Some(Commands::Help { command }) => cmd_help(command.as_deref()),
        Some(Commands::Guide { topic }) => cmd_guide(topic.as_deref()),
        None => cmd_help(None),
    }
}

// ── help ──────────────────────────────────────────────────────────────────────

fn cmd_help(command: Option<&str>) {
    match command {
        Some("install") => {
            println!();
            println!("{}", style("amphora install").bold().cyan());
            println!();
            println!("Wizard interativo que configura o ambiente amphora do zero.");
            println!();
            println!("{}", style("O que faz:").bold());
            println!("  {} Pergunta o caminho do vault Obsidian", ARROW);
            println!("  {} Pergunta o diretório para os scripts (~/.local/bin)", ARROW);
            println!("  {} Detecta automaticamente o sink de áudio (PipeWire)", ARROW);
            println!("  {} Permite escolher o que instalar:", ARROW);
            println!("      - Scripts de automação (meeting-record, video-note, daily-note...)");
            println!("      - Git hook global (registra commits na daily note do vault)");
            println!("      - CLAUDE.md (comandos /nota, /standup, /foco e outros)");
            println!("      - .obsidian config + Templates");
            println!();
            println!("{}", style("Uso:").bold());
            println!("  amphora install");
            println!();
            println!("{}", style("Variáveis de ambiente:").bold());
            println!("  AMPHORA_VAULT         Caminho do vault (default: ~/amphora)");
            println!("  AMPHORA_SINK_MONITOR  Sink de áudio para meeting-record");
            println!("                        (detectado via pw-cli se não definido)");
        }
        Some("check") => {
            println!();
            println!("{}", style("amphora check").bold().cyan());
            println!();
            println!("Verifica se todas as dependências necessárias estão instaladas.");
            println!();
            println!("{}", style("Dependências verificadas:").bold());
            println!("  claude         Claude Code CLI — engine de IA do sistema");
            println!("  nvim           Neovim — edição do vault pelo terminal");
            println!("  python3        Necessário para meeting-transcribe e video-note");
            println!("  faster-whisper Transcrição de áudio local (pip install faster-whisper)");
            println!("  yt-dlp         Download de legendas do YouTube (pip install yt-dlp)");
            println!("  pw-record      Gravação de áudio via PipeWire (pipewire)");
            println!("  notify-send    Notificações desktop (libnotify)");
            println!("  git            Versionamento do vault");
            println!();
            println!("{}", style("Uso:").bold());
            println!("  amphora check");
        }
        Some("update") => {
            println!();
            println!("{}", style("amphora update").bold().cyan());
            println!();
            println!("Atualiza scripts e configs já instalados após um git pull.");
            println!("Útil para sincronizar mudanças sem rodar o wizard de instalação completo.");
            println!();
            println!("{}", style("O que pode atualizar:").bold());
            println!("  Scripts          Copia versões novas para ~/.local/bin");
            println!("  CLAUDE.md        Atualiza comandos do Claude Code no vault");
            println!("  .obsidian config Atualiza configurações e templates do Obsidian");
            println!();
            println!("{}", style("Uso:").bold());
            println!("  amphora update");
        }
        Some(other) => {
            println!();
            println!(
                "{} Comando {} não reconhecido.",
                style(WARN).yellow(),
                style(other).bold()
            );
            println!();
            println!(
                "Comandos disponíveis: {}",
                style("install  check  update  help").cyan()
            );
        }
        None => {
            println!();
            println!("{}", style("amphora").bold().cyan());
            println!("Setup e gestão do ambiente PKMS amphora.");
            println!();
            println!("{}", style("Comandos:").bold());
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
            println!(
                "  {}     {}",
                style("help").green().bold(),
                "Mostra ajuda detalhada dos comandos"
            );
            println!();
            println!("{}", style("Exemplos:").bold());
            println!("  amphora check          # verificar dependências antes de instalar");
            println!("  amphora install        # wizard de instalação completo");
            println!("  amphora help install   # ajuda detalhada de um comando específico");
            println!();
            println!(
                "Repositório: {}",
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
            println!("{}", style("Scripts de automação").bold().cyan());
            println!();

            println!("{}", style("meeting-record").bold().green());
            println!("  Grava o áudio do sistema (monitor sink via PipeWire).");
            println!("  Ao parar a gravação, transcreve com faster-whisper e envia");
            println!("  a transcrição para o Claude, que gera um resumo estruturado");
            println!("  com contexto, decisões, próximos passos e participantes.");
            println!("  A nota é salva em Trabalho/Reuniões/ e linkada na daily note.");
            println!("  {}", style("Uso: keybind (ex: Super+R para iniciar/parar)").dim());
            println!();

            println!("{}", style("meeting-transcribe").bold().green());
            println!("  Transcreve um arquivo de áudio usando faster-whisper (modelo medium).");
            println!("  Usado internamente pelo meeting-record, mas pode ser chamado diretamente.");
            println!("  {}", style("Uso: meeting-transcribe <arquivo.wav>").dim());
            println!();

            println!("{}", style("video-note").bold().green());
            println!("  Recebe uma URL do YouTube, baixa a legenda via yt-dlp,");
            println!("  limpa o VTT e envia para o Claude gerar um resumo com tema");
            println!("  principal, pontos-chave e conclusão.");
            println!("  A nota é salva em Pessoal/Vídeos/ e linkada na daily note.");
            println!("  {}", style("Uso: video-note <url>").dim());
            println!();

            println!("{}", style("daily-note").bold().green());
            println!("  Abre (ou cria) a daily note do dia no Neovim via scratchpad do Hyprland.");
            println!("  Se a nota não existe, cria com o template completo: foco do dia,");
            println!("  tarefas pessoal/trabalho, notas rápidas e log de notas.");
            println!("  {}", style("Requer: Hyprland + alacritty").dim());
            println!("  {}", style("Uso: keybind (ex: Super+D)").dim());
            println!();

            println!("{}", style("vault-log-updates.sh").bold().green());
            println!("  Registra pacotes instalados, atualizados ou removidos do sistema");
            println!("  (via pacman) em Pessoal/Sistema/Updates.md no vault.");
            println!("  {}", style("Uso: chamado via hook do sistema ou manualmente").dim());
        }

        Some("claude") => {
            println!();
            println!("{}", style("Comandos Claude Code").bold().cyan());
            println!("  Disponíveis quando o Claude Code está aberto dentro do vault.");
            println!();

            let commands = vec![
                ("/nota",       "Captura rápida de conhecimento — cria nota estruturada com contexto, links e backlinks"),
                ("/foco",       "Abre uma sessão de trabalho profundo com contexto do projeto e bloqueia distrações"),
                ("/standup",    "Daily meeting — registra o que foi feito, o que será feito e bloqueios"),
                ("/review",     "Reflexão diária — o que funcionou, o que melhorar, highlight do dia"),
                ("/semana",     "Planejamento semanal — define prioridades e metas da semana"),
                ("/semanal",    "Revisão semanal — retrospectiva do que foi entregue e aprendido"),
                ("/retro",      "Retrospectiva mensal — análise mais ampla de progresso e ajustes"),
                ("/morning",    "Rotina matinal — abre o dia com intenção e revisão das tarefas"),
                ("/tarefa",     "Registra uma tarefa de trabalho com contexto, prioridade e prazo"),
                ("/aprendizado","Captura aprendizado técnico — cria nota de estudo com conceitos e referências"),
                ("/ideia",      "Captura rápida de ideia — salva antes de perder sem interromper o fluxo"),
                ("/brainstorm", "Parceiro de brainstorming — explora ideias de forma não-linear"),
                ("/leitura",    "Diário de leitura — registra impressões, citações e insights de um livro"),
                ("/log",        "Registro de sessão — documenta o que foi feito numa sessão de trabalho"),
                ("/contexto",   "Carrega contexto de um projeto específico para a conversa"),
                ("/gmud",       "Cria nota de GMUD (Gestão de Mudança) para deploy ou alteração em produção"),
                ("/check",      "Revisão de tarefas — lista pendências e ajuda a priorizar"),
            ];

            for (cmd, desc) in commands {
                println!("  {}  {}", style(cmd).green().bold(), desc);
            }
        }

        Some("hook") => {
            println!();
            println!("{}", style("Git hook — post-commit").bold().cyan());
            println!();
            println!("Hook global que roda automaticamente após cada commit em qualquer repositório.");
            println!();
            println!("{}", style("O que faz:").bold());
            println!("  {} Registra o commit na daily note do vault", ARROW);
            println!("      Formato: hash · repo (branch): mensagem do commit");
            println!();
            println!("  {} Atualiza a nota do projeto correspondente no vault", ARROW);
            println!("      Busca em Pessoal/Projetos/ e Trabalho/ uma nota com o nome do repo.");
            println!("      Se encontrar, adiciona o commit em ## Commits com contexto gerado");
            println!("      pelo Claude (uma frase sobre o objetivo ou impacto da mudança).");
            println!();
            println!("{}", style("Observações:").bold());
            println!("  - Commits no próprio vault (amphora) são ignorados");
            println!("  - O contexto do Claude roda em background, não bloqueia o commit");
            println!("  - A nota do projeto precisa existir para receber o log");
        }

        Some("obsidian") => {
            println!();
            println!("{}", style("Obsidian — configuração").bold().cyan());
            println!();
            println!("{}", style("Plugins:").bold());
            let plugins = vec![
                ("obsidian-git",          "Backup automático a cada 1 min, auto-pull no boot, sync via merge"),
                ("dataview",              "Queries de tarefas e notas nas daily notes"),
                ("templater-obsidian",    "Templates com lógica: data atual, dia da semana em pt-BR, prompts"),
                ("obsidian-tasks-plugin", "Gerenciamento de tarefas com queries, filtros e datas"),
                ("calendar",              "Navegação por daily notes via calendário na sidebar"),
                ("obsidian-reminder-plugin", "Lembretes de tarefas com notificação desktop"),
                ("typewriter-mode",       "Foca a linha atual no centro da tela durante a escrita"),
            ];
            for (p, desc) in plugins {
                println!("  {}  {}", style(p).green().bold(), desc);
            }
            println!();
            println!("{}", style("Templates incluídos:").bold());
            println!("  Daily Notes      Template completo com foco, tarefas, log e dataview");
            println!("  Aprendizado      Estrutura para notas de estudo técnico");
            println!("  Weekly Review    Retrospectiva semanal com queries de tarefas");
            println!("  Demanda          Template para demandas de trabalho com retrospectiva");
            println!("  Review           Template para reviews de filmes, séries e podcasts");
            println!();
            println!("{}", style("Tema:").bold());
            println!("  O vault funciona melhor com o tema mateCreations (Yerba Mate / Tererê).");
            println!("  {} github.com/nfvelten", ARROW);
        }

        Some(other) => {
            println!();
            println!(
                "{} Tópico {} não reconhecido.",
                style(WARN).yellow(),
                style(other).bold()
            );
            println!();
            println!(
                "Tópicos disponíveis: {}",
                style("scripts  claude  hook  obsidian").cyan()
            );
        }

        None => {
            println!();
            println!("{}", style("amphora guide — visão geral do sistema").bold().cyan());
            println!();
            println!("O amphora é um PKMS (Personal Knowledge Management System) construído");
            println!("para reduzir carga cognitiva e externalizar o pensamento em notas");
            println!("estruturadas, com automação de captura e integração com IA.");
            println!();
            println!("{}", style("Componentes:").bold());
            println!(
                "  {}        Scripts de automação — gravação, transcrição, ingestão de vídeos",
                style("scripts").green().bold()
            );
            println!(
                "  {}         Comandos do Claude Code no vault — /nota, /foco, /standup...",
                style("claude").green().bold()
            );
            println!(
                "  {}           Git hook global — registra commits nas notas do vault",
                style("hook").green().bold()
            );
            println!(
                "  {}        Configuração do Obsidian — plugins, templates e tema",
                style("obsidian").green().bold()
            );
            println!();
            println!("{}", style("Exemplos:").bold());
            println!("  amphora guide scripts    # o que cada script faz e como usar");
            println!("  amphora guide claude     # lista de comandos /cmd disponíveis");
            println!("  amphora guide hook       # como o git hook funciona");
            println!("  amphora guide obsidian   # plugins e templates incluídos");
        }
    }
    println!();
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
