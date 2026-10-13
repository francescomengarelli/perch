---
name: dotfiles
description: Manage dotfiles with perch, a symlink-farm dotfiles manager (like GNU Stow, with per-machine modules). Use when the user mentions dotfiles or perch, wants to track/adopt or untrack a config file in their dotfiles repo, sync or update dotfiles, clean up stale dotfile links, edits a file under ~ that is a symlink into ~/dotfiles, or hits a perch error such as "is not mine. i am not touching it".
allowed-tools: Bash(perch status) Bash(perch --version) Bash(perch --help) Bash(perch help *) Bash(perch sync --dry-run) Bash(perch prune --dry-run)
---

# Managing dotfiles with perch

perch keeps dotfiles in a git repo (default `~/dotfiles`) and symlinks them into `$HOME`.
If perch isn't installed, or there's no dotfiles repo yet, use the `setup` skill instead.

**These instructions are for perch 0.10.0 or newer.** Check with `perch --version`.
If it's older, suggest `perch self-upgrade` and ask before running it. Older versions don't have `sync --dry-run`, `prune` or `remove`, and they rewrite the config file (dropping its comments) on every run.

## Mental model

- A **module** is a top-level folder in the dotfiles repo that mirrors `$HOME`:
  `~/dotfiles/<module>/<rel>` is linked at `~/<rel>`.
  For example, `~/dotfiles/macos/.config/nvim/init.lua` links to `~/.config/nvim/init.lua`.
- Links are made **per file**. Every file gets its own absolute symlink. Parent directories in `~` are created as real directories, not links.
- A module is **active** when its `when` shell command exits 0. A module with no `when` is always active.
- With no config file, the modules are:
  - `common` (always active)
  - `linux` (`uname -s | grep -q Linux`)
  - `macos` (`uname -s | grep -q Darwin`)
- Once a config file exists, it **replaces those defaults completely**. Config lookup order:
  1. `--config <path>`
  2. `~/.config/perch/config.toml`
  3. the built-in defaults
- The config usually lives in the repo itself, at `common/.config/perch/config.toml`, and is symlinked into place.

## When to use perch

- Use perch when `~/dotfiles` (or the configured `dotfiles_dir`) or `~/.config/perch/config.toml` exists, or when the user asks for it.
- **Before editing any file under `~`, check whether it's a perch link.** Run `readlink -f <file>`.
  - If it resolves into the dotfiles repo, editing through the link is fine; the change lands in the repo.
  - Mention that the change should be committed in the repo (`perch status` shows it).
- To track or untrack files, use `perch add` and `perch remove`. Don't hand-move files and hand-create links.
- perch has no templating or secrets support. Don't put tokens or keys in the repo; suggest keeping them in an untracked local file that the tracked config sources.

## Commands

The module comes **first** in `add`. Global flags `--config <path>` and `-v`/`-vv` go before or after the subcommand.

| Task | Command |
|---|---|
| Adopt files into a module: move them into the repo and link them back | `perch add <module> <path>...` |
| Stop managing files: move them back out of the repo, replacing the links | `perch remove <path>...` |
| Preview a sync: what would be linked, what conflicts | `perch sync --dry-run` |
| Link all active modules into `~` (safe to re-run) | `perch sync` |
| Preview stale links (file gone, or module no longer active) | `perch prune --dry-run` |
| Delete those stale links | `perch prune` |
| `git pull` the repo, then sync (picks up config changes from the pull) | `perch update` |
| `git status` of the repo | `perch status` |
| Relocate the repo (the destination must not exist) | `perch move-dir <path>` |
| Upgrade the perch binary | `perch self-upgrade` |

- `add` and `remove` take files or directories. Directories are handled recursively, file by file.
- perch never commits. After `add` or `remove`, or after editing a tracked file, commit in the repo:
  `git -C ~/dotfiles add -A && git -C ~/dotfiles commit -m "..."`.
  Do that only if the user wants a commit.
- Pick the module deliberately:
  - `common` for files used everywhere.
  - An OS or host module for machine-specific files.
  - `add` warns if the module isn't in the config or isn't active here. Take the warning seriously: files in such a module won't be linked by `sync` on other machines, or on this one.

## Safety rules

perch can't undo anything. Work carefully on a real home directory.

1. **Run `perch sync --dry-run` before `sync` or `update`** on anyone's real home directory. It lists `link …` and `conflict …` lines, and exits non-zero if there are conflicts.
2. **A conflict aborts the whole sync, and nothing is changed.** A conflict is a target that already exists and isn't perch's link. Never delete the conflicting file to clear it. Ask the user, then do one of these:
   - Back it up: `mv <file> <file>.bak`.
   - If the local version should win, copy it over the repo version first.
3. **Run `perch prune --dry-run` before `perch prune`**, and show the user the list. `prune` only knows links recorded by perch 0.10+. Links whose files were already gone before the upgrade aren't recorded, and `sync` won't record them. See `no link record yet` in [references/troubleshooting.md](references/troubleshooting.md).
4. **Run `move-dir` and `self-upgrade` only when the user explicitly asks.** Make sure the repo is committed first.
5. Use `readlink` and `ls -l`, not `cat`, to inspect whether something is linked.

## Ignoring files

`.perchignore` uses gitignore syntax. perch does **not** read `.gitignore`.
- A root `~/dotfiles/.perchignore` applies to every module. Its patterns are relative to the repo root (`common/.config/nvim/lazy-lock.json`, or unanchored like `.DS_Store`).
- A `.perchignore` inside a module applies to its own directory and below.
- Both `sync` and `add` honor these files, and `.perchignore` files are never linked.
- The details are in [references/config.md](references/config.md).

## References

Read these only when needed:
- [references/cli.md](references/cli.md): exact semantics of every command, including what `add`, `remove` and `prune` skip. Read before running `add`/`remove` on directories with symlinks, or before `update` or `move-dir`.
- [references/config.md](references/config.md): config schema, `when` examples, `.perchignore` rules. Read when editing the config or adding a module.
- [references/troubleshooting.md](references/troubleshooting.md): perch error messages and their fixes. Read when a perch command fails or links are missing.
