# amphora-setup

Setup completo do meu PKMS (Personal Knowledge Management System) — um ambiente integrado para captura, processamento e recuperação de conhecimento, construído em torno do Obsidian, Claude Code e Neovim.

O objetivo é reduzir carga cognitiva ao externalizar o máximo possível: reuniões são transcritas e resumidas automaticamente, vídeos viram notas, commits são registrados no vault, e o Claude Code opera diretamente no vault com comandos customizados para knowledge management.

---

## O que está incluído

### Scripts (`bin/`)

| Script | O que faz |
|---|---|
| `meeting-record` | Grava áudio do sistema (monitor sink), transcreve com Whisper e gera resumo + action items via Claude. Toggle com um keybind. |
| `meeting-transcribe` | Transcrição de áudio com `faster-whisper` (modelo medium, pt-BR). Usado pelo `meeting-record`. |
| `video-note` | Recebe uma URL do YouTube, extrai legenda, resume com Claude e salva nota no vault. |
| `daily-note` | Cria/abre a daily note do dia no Neovim via scratchpad do Hyprland. |
| `vault-log-updates.sh` | Registra pacotes instalados/atualizados/removidos do sistema no vault. |

### Git hook (`git-hooks/post-commit`)

Hook global que roda após cada commit em qualquer repositório:
- Registra o commit na daily note do vault com hash, repo e branch
- Atualiza a nota do projeto correspondente no vault com contexto gerado pelo Claude

### Claude Code (`claude/CLAUDE.md`)

`CLAUDE.md` do vault com comandos customizados para o Claude Code operar no vault:

- `/nota` — captura rápida de conhecimento
- `/foco` — sessão de trabalho profundo
- `/standup` — daily meeting
- `/review` — reflexão diária
- `/semana` — planejamento semanal
- `/semanal` — revisão semanal
- `/retro` — retrospectiva mensal
- `/morning` — rotina matinal
- `/tarefa` — registro de tarefas
- `/aprendizado` — captura de aprendizados técnicos
- `/ideia` — captura rápida de ideias
- `/brainstorm` — parceiro de brainstorming
- `/leitura` — diário de leitura
- `/log` — registro de sessão
- `/contexto` — contexto de projeto
- `/gmud` — criação de GMUD
- `/check` — revisão de tarefas

---

## Requisitos

- **Claude Code** — `curl -fsSL https://claude.ai/install.sh | sh`
- **Python 3** + `faster-whisper` — `pip install faster-whisper`
- **yt-dlp** — `pip install yt-dlp`
- **PipeWire** (`pw-record`) — para gravação de reuniões
- **Obsidian** — vault em `~/amphora` (ou configure `AMPHORA_VAULT`)
- **Neovim** — recomendado para edição do vault
- **libnotify** (`notify-send`) — para notificações desktop
- **Hyprland** — necessário apenas para `daily-note` (scratchpad)

### Plugins Obsidian

- `obsidian-git` — backup automático a cada minuto
- `dataview` — queries de tarefas e notas
- `templater-obsidian` — templates para daily notes
- `obsidian-tasks-plugin` — gerenciamento de tarefas
- `calendar` — navegação por daily notes
- `obsidian-reminder-plugin` — lembretes de tarefas

---

## Instalação

```bash
git clone https://github.com/nfvelten/amphora-setup
cd amphora-setup
bash install.sh
```

O script instala os binários em `~/.local/bin/`, configura o git hook global e copia o `CLAUDE.md` para o vault.

### Configuração via variáveis de ambiente

Adicione ao seu `~/.bashrc` ou `~/.zshrc` se necessário:

```bash
export AMPHORA_VAULT="$HOME/amphora"          # caminho do vault (default: ~/amphora)
export AMPHORA_SINK_MONITOR="alsa_output..."  # sink de áudio para meeting-record
                                              # detectado automaticamente se não definido
```

Para descobrir seu sink de áudio:
```bash
pw-cli list-objects | grep -o 'alsa_output[^"]*monitor'
```

---

## Integração com Neovim

A configuração do Neovim (incluindo integração com o vault via `oil.nvim`, `telescope` e navegação de wiki-links) está no repositório de dotfiles:

→ [github.com/nfvelten/dotfiles](https://github.com/nfvelten/dotfiles)

---

## Tema

O ambiente inteiro usa o tema **mateCreations** (Yerba Mate / Tererê) — disponível para Obsidian, Neovim, VS Code e Zen Browser, com alternância automática claro/escuro baseada em horário:

→ [github.com/nfvelten](https://github.com/nfvelten)
