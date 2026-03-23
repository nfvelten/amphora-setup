#!/usr/bin/env bash
VAULT="/home/nfvelten/amphora"
FILE="$VAULT/Pessoal/Sistema/Updates.md"
DATE=$(date +%d-%m-%Y)
TIME=$(date +%H:%M)

PACKAGES=$(grep "\[ALPM\] \(upgraded\|installed\|removed\)" /var/log/pacman.log \
  | grep "$(date +%Y-%m-%dT)" \
  | awk '{
      action = $2
      pkg = $3
      printf "- %s `%s`\n", action, pkg
    }')

[ -z "$PACKAGES" ] && PACKAGES="- (sem pacotes registrados)"

{
  echo ""
  echo "## $DATE $TIME"
  echo ""
  echo "$PACKAGES"
} >> "$FILE"
