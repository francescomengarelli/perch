---
name: setup
description: Install perch and set up a dotfiles repo with it. Use when installing perch, bootstrapping dotfiles on a new machine, starting a dotfiles repo from scratch, or migrating to perch from GNU Stow, yadm, chezmoi, a bare git repo or loose config files.
allowed-tools: Bash(command -v perch) Bash(perch --version) Bash(perch sync --dry-run)
---

# Setting up perch

perch is a dotfiles manager. It symlinks each file in `~/dotfiles/<module>/` to the same relative path under `~`, for every module whose `when` condition passes. For day-to-day use after setup (adding files, syncing, troubleshooting), use the `dotfiles` skill.

## 1. Install

Check whether perch is already installed: `command -v perch && perch --version`.
These instructions need perch 0.10.0 or newer. If an older version is installed, suggest `perch self-upgrade` and ask before running it.

If it isn't, confirm with the user, then install one of these ways:

```sh
# prebuilt release into ~/.local/bin
curl -fsSL https://raw.githubusercontent.com/francescomengarelli/perch/main/install.sh | sh

# or build from source (needs cargo)
cargo install --root ~/.local --git https://github.com/francescomengarelli/perch
```

Make sure `~/.local/bin` is on `PATH`. perch also needs `git` and `sh`.

## 2. Pick a path

Ask the user which one applies.

### A. New machine, existing perch dotfiles repo

1. Find out where the repo's config expects to live. Look for `dotfiles_dir` in `common/.config/perch/config.toml`; the default is `~/dotfiles`. Clone into **that** directory:
   ```sh
   git clone <repo-url> ~/dotfiles
   ```
2. Preview the first sync, pointing perch at the in-repo config:
   ```sh
   perch sync --dry-run --config ~/dotfiles/common/.config/perch/config.toml
   ```
   Real files already in `~`, such as a distro default `.bashrc`, show up as `conflict` lines and would abort the sync. Ask the user what to do with each one, then back it up with `mv <file> <file>.bak`.
3. Run the first sync:
   ```sh
   perch sync --config ~/dotfiles/common/.config/perch/config.toml
   ```
   If the repo has no config file, plain `perch sync` uses the defaults: `common`, plus `linux` or `macos`. Modules without a folder are skipped with a warning.
4. From now on, plain `perch sync` and `perch update` work.

### B. Starting from scratch

1. Create the repo and its modules:
   ```sh
   mkdir -p ~/dotfiles/common
   git -C ~/dotfiles init
   ```
2. Decide on modules with the user. A good start is:
   - `common` for files used everywhere.
   - One module per OS (`linux`, `macos`).
   - Optional host or role modules (`work`, `laptop`).
   Create a folder for every module, even if it's empty.
3. Write the config **inside the repo**, so it travels with the dotfiles:
   ```sh
   mkdir -p ~/dotfiles/common/.config/perch
   ```
   ```toml
   # ~/dotfiles/common/.config/perch/config.toml
   dotfiles_dir = "~/dotfiles"

   [[module]]
   name = "common"

   [[module]]
   name = "linux"
   when = "uname -s | grep -q Linux"

   [[module]]
   name = "macos"
   when = "uname -s | grep -q Darwin"
   ```
   Comments are fine. perch only ever edits the `dotfiles_dir` line, and only when the repo moves.
4. Link the config into place and confirm that perch works:
   ```sh
   perch sync --config ~/dotfiles/common/.config/perch/config.toml
   ```
5. Adopt the user's existing configs, choosing a module for each:
   ```sh
   perch add common ~/.gitconfig ~/.config/nvim
   perch add linux ~/.config/hypr
   ```
   Before adopting a directory, look for junk inside it, such as caches, state, plugin directories, or `.DS_Store`. Exclude it with a root `~/dotfiles/.perchignore` (step 6) or a `.perchignore` in that directory, or adopt only specific files. Write the ignore rules **before** running `add`, because `add` honors them.
   **Never adopt secrets** (`~/.ssh` keys, tokens, `.netrc`).
6. Optionally add a root `~/dotfiles/.perchignore`, for example with `.DS_Store` and `README.md`. Patterns are relative to the repo root.
7. Commit, and push if the user has a remote:
   ```sh
   git -C ~/dotfiles add -A && git -C ~/dotfiles commit -m "initial dotfiles"
   ```

### C. Migrating from another tool

Repo layouts map like this:

| From | Mapping |
|---|---|
| **GNU Stow** | Stow packages are already home-mirroring folders, so each package becomes a perch module. List them as modules in the config; packages that apply everywhere can be merged into `common`. |
| **yadm / bare git repo** | Files live directly in `~`. Create a new `~/dotfiles` repo and `perch add common <each tracked file>`. Get the list with `yadm list -a` or `git --git-dir=<bare> ls-files`. |
| **chezmoi** | Run `chezmoi archive` or `chezmoi apply` first, so the real files are in `~`. Then `perch add` them. Templates and secrets have no perch equivalent, so keep those files untracked or keep them per-module. |
| **Loose files** | Follow path B. |

**Migration from Stow:**
1. Run `stow -D <pkg>` for every package **before** `perch sync`. Stow's directory-level links make every file under them a conflict.
2. Remove leftover stow links pointing into the repo.
3. Run `perch sync --dry-run`. It should list `link` lines only, with no conflicts.
4. Run `perch sync`.

## 3. Verify

- `perch status`: the repo is clean, or only has expected changes.
- Spot-check a few links with `readlink -f ~/.gitconfig`; each should point into the dotfiles repo.
- `perch sync --dry-run` should now report `0 to link` and no conflicts.
- Tell the user their daily commands:
  - `perch add <module> <path>` to adopt a new file, and `perch remove <path>` to stop managing one.
  - `perch update` to pull and sync.
  - `perch status` to see the repo state.
  - `perch prune` to clean up links left behind by deleted files or deactivated modules.
