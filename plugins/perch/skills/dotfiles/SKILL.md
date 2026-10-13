---
name: dotfiles
description: Manage dotfiles with perch, a symlink-farm dotfiles manager (like GNU Stow, with per-machine modules). Use when the user mentions dotfiles or perch, wants to track/adopt a config file in their dotfiles repo, sync or update dotfiles, edits a file under ~ that is a symlink into ~/dotfiles, or hits a perch error such as "is not mine. i am not touching it".
allowed-tools: Bash(perch status) Bash(perch --version) Bash(perch --help) Bash(perch help *)
---

# Managing dotfiles with perch

perch keeps dotfiles in a git repo (default `~/dotfiles`) and symlinks them into `$HOME`.
If perch isn't installed, or there's no dotfiles repo yet, use the `setup` skill instead.

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
- To track a new config file, use `perch add`. Don't hand-move files and hand-create links.
- perch has no templating or secrets support. Don't put tokens or keys in the repo; suggest keeping them in an untracked local file that the tracked config sources.

## Commands

The module comes **first** in `add`. Global flags `--config <path>` and `-v`/`-vv` go before or after the subcommand.

| Task | Command |
|---|---|
| Adopt files into a module: move them into the repo and link them back | `perch add <module> <path>...` |
| Link all active modules into `~` (safe to re-run) | `perch sync` |
| `git pull` the repo, then sync | `perch update` |
| `git status` of the repo | `perch status` |
| Relocate the repo | `perch move-dir <path>` |
| Upgrade the perch binary | `perch self-upgrade` |

- `add` takes files or directories. Directories are adopted recursively, file by file.
- perch never commits. After `add`, or after editing a tracked file, commit in the repo:
  `git -C ~/dotfiles add -A && git -C ~/dotfiles commit -m "..."`.
  Do that only if the user wants a commit.
- Pick the module deliberately:
  - `common` for files used everywhere.
  - An OS or host module for machine-specific files.
  - `add` doesn't check that the module exists or is active. A file added to an inactive module is linked on this machine now, but won't be linked by `sync` on other machines.

## Safety rules

perch has **no dry-run** and **no undo**. Work carefully on a real home directory.

1. **Before `sync` or `update`, look ahead.** List which targets already exist as real files, since those will be conflicts:
   ```sh
   cd ~/dotfiles/<module> && find . -type f -o -type l | while read -r f; do t="$HOME/${f#./}"; [ -e "$t" ] && [ ! -L "$t" ] && echo "conflict: $t"; done
   ```
   (`.perchignore`d files are skipped by perch but not by this check, so it may over-report.)
2. **A conflict aborts the whole sync, and nothing is changed.** Never delete the conflicting file to clear it. Ask the user, then do one of these:
   - Back it up: `mv <file> <file>.bak`.
   - If the local version should win, copy it over the repo version first.
3. **Every active module directory must exist.** If one is missing, `sync` fails with `IO error ... No such file or directory`. Create the directory (`mkdir ~/dotfiles/<module>`), or fix the config.
4. **Run `move-dir` and `self-upgrade` only when the user explicitly asks.**
   - `move-dir` copies the repo (flattening symlinks inside it) and deletes the old one.
   - If `move-dir` fails, it removes the destination directory.
5. **perch rewrites the config TOML after every successful command, including `status`.** This drops comments and reformats the file. A config diff in `git status` after running perch is usually this; tell the user rather than "fixing" it silently.
6. Use `readlink` and `ls -l`, not `cat`, to inspect whether something is linked.

## Untracking and cleanup

perch has no unlink or remove command. To stop managing a file:

```sh
rm ~/.config/foo/bar                                  # remove the symlink
mv ~/dotfiles/<module>/.config/foo/bar ~/.config/foo/bar
git -C ~/dotfiles add -A && git -C ~/dotfiles commit -m "untrack foo/bar"
```

Files deleted from the repo, or modules that are no longer active, leave **dangling links** behind. To find them, keep the search to the relevant directories (searching all of `~` is slow):

```sh
find ~ -maxdepth 6 -xtype l -lname "$HOME/dotfiles/*" 2>/dev/null   # adjust if dotfiles_dir differs
```

## Ignoring files

`.perchignore` uses gitignore syntax. perch does **not** read `.gitignore`.
- A root `~/dotfiles/.perchignore` applies to every module during `sync`. Its patterns are relative to the repo root.
- A `.perchignore` inside a module applies to its own directory and below.
- The details are in [references/config.md](references/config.md).

## References

Read these only when needed:
- [references/cli.md](references/cli.md): exact semantics of every command and what `add` skips. Read before running `add` on directories with symlinks, or before `update` or `move-dir`.
- [references/config.md](references/config.md): config schema, `when` examples, `.perchignore` rules. Read when editing the config or adding a module.
- [references/troubleshooting.md](references/troubleshooting.md): perch error messages and their fixes. Read when a perch command fails or links are missing.
