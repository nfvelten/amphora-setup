#!/usr/bin/env bash
# amphora-setup installer

set -euo pipefail

VAULT="${AMPHORA_VAULT:-$HOME/amphora}"
BIN_DIR="$HOME/.local/bin"
HOOK_TARGET="$HOME/.config/git/hooks/post-commit"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info()  { echo -e "${GREEN}✓${NC} $1"; }
warn()  { echo -e "${YELLOW}!${NC} $1"; }

echo "=== amphora-setup ==="
echo "Vault: $VAULT"
echo ""

# ── Dependências ──────────────────────────────────────────────────────────────
echo "Verificando dependências..."

check_dep() {
  if command -v "$1" &>/dev/null; then
    info "$1"
  else
    warn "$1 não encontrado — $2"
  fi
}

check_dep claude       "instale via: curl -fsSL https://claude.ai/install.sh | sh"
check_dep python3      "necessário para meeting-transcribe e video-note"
check_dep yt-dlp       "necessário para video-note (pip install yt-dlp)"
check_dep pw-record    "necessário para meeting-record (pipewire)"
check_dep notify-send  "necessário para notificações (libnotify)"
check_dep nvim         "recomendado para edição do vault"

python3 -c "import faster_whisper" 2>/dev/null \
  && info "faster-whisper" \
  || warn "faster-whisper não encontrado — pip install faster-whisper"

echo ""

# ── Scripts ───────────────────────────────────────────────────────────────────
echo "Instalando scripts em $BIN_DIR..."
mkdir -p "$BIN_DIR"

for script in bin/*; do
  name=$(basename "$script")
  cp "$script" "$BIN_DIR/$name"
  chmod +x "$BIN_DIR/$name"
  info "$name → $BIN_DIR/$name"
done

echo ""

# ── Git hook ──────────────────────────────────────────────────────────────────
echo "Configurando git hook global..."
mkdir -p "$(dirname "$HOOK_TARGET")"

if [ -f "$HOOK_TARGET" ]; then
  warn "post-commit já existe em $HOOK_TARGET — pulando (compare com git-hooks/post-commit)"
else
  cp git-hooks/post-commit "$HOOK_TARGET"
  chmod +x "$HOOK_TARGET"
  git config --global core.hooksPath "$HOME/.config/git/hooks"
  info "post-commit instalado"
fi

echo ""

# ── Vault ─────────────────────────────────────────────────────────────────────
echo "Configurando vault em $VAULT..."

if [ -d "$VAULT" ]; then
  warn "Vault já existe em $VAULT — pulando criação de estrutura"
else
  # Copia a estrutura do vault template
  cp -r vault "$VAULT"
  # Remove .gitkeep das pastas
  find "$VAULT" -name ".gitkeep" -delete
  info "Vault criado em $VAULT"
fi

# CLAUDE.md
if [ ! -f "$VAULT/CLAUDE.md" ]; then
  cp claude/CLAUDE.md "$VAULT/CLAUDE.md"
  info "CLAUDE.md copiado para o vault"
else
  warn "CLAUDE.md já existe no vault — compare com claude/CLAUDE.md se necessário"
fi

echo ""
echo "=== Instalação concluída ==="
echo ""
echo "Próximos passos:"
echo "  1. Abra $VAULT no Obsidian"
echo "  2. Ative os plugins: Settings → Community Plugins → Enable All"
echo "     (Obsidian baixa os plugins automaticamente)"
echo "  3. Para o tema mateCreations: https://github.com/nfvelten"
echo "  4. Para meeting-record, configure AMPHORA_SINK_MONITOR ou deixe detectar automaticamente"
echo "  5. Para Neovim: https://github.com/nfvelten/dotfiles"
echo ""
echo "Variáveis de ambiente opcionais (~/.bashrc ou ~/.zshrc):"
echo "  export AMPHORA_VAULT=\"\$HOME/amphora\"          # default"
echo "  export AMPHORA_SINK_MONITOR=\"alsa_output...\"   # para meeting-record"
