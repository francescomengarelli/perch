# perch CLI reference (0.10+)

Most of perch's messages are plain text on stderr. The listings from `--dry-run` go to stdout. Errors print `Error: ...` and exit non-zero.

## Global flags

- `--config <path>`: use this config file. `~` is expanded, and relative paths are relative to the cwd.
  - If the file is missing, perch fails with `i couldn't find a config file at X - does it exist?`.
  - Without `--config`, perch uses `~/.config/perch/config.toml` if it exists, and the built-in defaults otherwise.
- `-v`: log the "using defaults" notice, each path being added, and each link pruned.
- `-vv`: also log each file linked, moved or restored.
- `--version`, `--help`.

## `perch sync [--dry-run|-n]`

1. Walks every **active** module, file by file. It honors `.perchignore`, skips `.perchignore` files themselves, and includes hidden files.
   - An active module whose folder doesn't exist is skipped with a warning (`module 'X' is active but … doesn't exist — skipping`).
2. For each file at `<dotfiles>/<module>/<rel>`, it computes the target `~/<rel>`:
   - If the target is already a symlink to that exact source, it's skipped. That's why `sync` is safe to re-run.
   - If the target exists in any other form (a real file, or a link elsewhere), it's a conflict.
3. If there are any conflicts, perch prints `<target> is not mine. i am not touching it` for each one, then `N conflicts found. resolve before syncing. no changes applied`, and **nothing is written**.
4. Otherwise it creates the parent directories (as real directories) and an absolute symlink for each file. It records the links in the link record, then prints `N modules synced`.

- `--dry-run` prints `link ~/x -> ~/dotfiles/m/x` and `conflict ~/x (not mine)` lines to stdout, plus a summary, and changes nothing. It exits non-zero if there are conflicts.
- If an ancestor directory in `~` is itself a symlink (for example a directory-level GNU Stow link), the targets resolve through it and show up as conflicts.

## `perch add <module> <path>...`

The module comes first. After it, pass one or more paths, which can be relative or absolute.

1. Directories are walked recursively. It honors `.perchignore` files inside the source tree **and** the repo-root `.perchignore`.
2. If the module isn't in the config, or isn't active on this machine, perch prints a warning but still continues.
3. Each entry is resolved with `canonicalize`. If an entry is a symlink, the **real file it points to** is moved, and the new link is created at the real path, not at the original link.
4. These entries are skipped:
   - Broken symlinks (`skipping broken symlink ...`).
   - Files already inside the dotfiles repo (skipped silently).
   - Files outside `$HOME` (`... is outside your home directory — skipping`).
   - Duplicates reached through several paths.
   - Files matched by an ignore rule.
5. For every file, the destination is `<dotfiles>/<module>/<path relative to $HOME>`. If any destination already exists, perch prints `... is already in my dotfiles directory — not overwriting it`, then `N conflict found. resolve before adding them`, and **nothing is moved**.
6. Otherwise each file is moved and a symlink is left in its place. Moves across filesystems work, by copying and then deleting.

## `perch remove <path>...`

This is the opposite of `add`. Each argument is either a perch symlink or a directory, which is searched for perch symlinks.

- For each link that points into the dotfiles dir, perch:
  1. deletes the link;
  2. moves the repo file back to the link's location;
  3. deletes any directories in the repo left empty, up to but not including the module folder.
- Paths that aren't perch links are reported (`… isn't managed by me — skipping`) and left alone. Inside directories they're skipped silently.
- A link whose repo file is already gone is skipped; `prune` handles those.
- `remove` doesn't commit. Commit the deletion in the repo afterwards.

## `perch prune [--dry-run|-n]`

Goes through the link record and deletes links that point into the dotfiles dir where either:
- the repo file is gone, or
- the link's module isn't active anymore (its `when` fails, or it was removed from the config).

It never deletes real files or directories, and never touches links that point anywhere else.
- With `--dry-run`, it prints `prune ~/x (reason)` lines and changes nothing.
- If the link record is empty, it prints `no link record yet — run perch sync once`.

## `perch update`

1. Runs `git -C <dotfiles> pull`, with git's output passed through. If the pull fails, perch stops there.
2. Re-reads the config, because the pull may have changed it.
3. If the new config names a different `dotfiles_dir`, it runs `move-dir` there first.
4. Syncs with the **new** module list. Conflicts abort it just as they do for `sync`.

## `perch status`

Runs `git -C <dotfiles> status`.

## `perch move-dir <path>`

1. It refuses if `<path>` exists, unless it's an empty directory. It also refuses if one directory is inside the other.
2. It renames the repo. Across filesystems it copies instead, preserving symlinks and empty directories, then deletes the original.
3. It re-points only the links in `~` that pointed into the old location; other links are untouched.
4. It writes the new `dotfiles_dir` into the config in use. Only that line changes, and comments are kept.

Run it only when the user asks explicitly.

## `perch self-upgrade`

Downloads the latest GitHub release for this platform and replaces the running binary, but only if the release is newer than the installed version.

## Files perch writes

- **The config:** only when `dotfiles_dir` changes, which happens with `move-dir` or with an `update` that relocates the repo. Only that key is edited.
- **The link record:** `${XDG_STATE_HOME:-~/.local/state}/perch/links`, one path per line. It's used by `prune`. The first `sync` after upgrading from an older version fills it in.
