# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This is a personal Obsidian knowledge vault (amphora), used for note-taking and knowledge management. Notes are written in Portuguese (pt-BR) and edited via both Obsidian and Neovim. Version control is handled automatically by the obsidian-git plugin, which auto-commits every minute with the message `vault backup: {{date}}`.

## Structure

- `Daily Notes/` — Daily entries named `dd-mm-yyyy.md`, organized by month in subdirectories (e.g., `Agosto/`, `Julho/`)
- `Pessoal/` — Personal topics: diet, workouts, reading lists, routines, tasks
- `Trabalho/` — Work-related notes: projects, initiatives, technical solutions
- `Posts/` — Draft posts
- `Templates/` — Obsidian templates using Templater plugin syntax (`<% tp.* %>`)

## Conventions

- Daily note filenames: `dd-mm-yyyy.md`
- Tags in frontmatter YAML (`tags: [daily_notes]`) and inline (`#pessoal`, `#trabalho`)
- Internal links use Obsidian wiki-link syntax: `[[note name]]`
- Tasks use Obsidian Tasks plugin syntax: `- [ ] task` with optional tags/dates
- Daily notes template includes sections: Foco Principal, Lista de Tarefas (pessoal + trabalho), Notas Rápidas, Tarefas incompletas (dataview query block)

## Plugins in Use

- **obsidian-git**: Auto-backup every 1 minute, auto-pull on boot, merge sync strategy
- **dataview**: For querying tasks and notes
- **templater-obsidian**: Template engine for daily notes
- **obsidian-tasks-plugin**: Task management with queries
- **calendar**: Calendar view for daily notes navigation
- **obsidian-reminder-plugin**: Task reminders
