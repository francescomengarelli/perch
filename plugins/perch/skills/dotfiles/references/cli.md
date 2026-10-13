# perch CLI reference

perch's output is plain text, almost all of it on stderr. It has no JSON output and no `--dry-run`. Errors print `Error: ...` and exit non-zero.

## Global flags

- `--config <path>`: use this config file. `~` is expanded.
  - If the file is missing, perch fails with `i couldn't find a config file at X - does it exist?`.
  - Without `--config`, perch uses `~/.config/perch/config.toml` if it exists, and the built-in defaults otherwise.
- `-v`: log the "using defaults" notice and each path being added.
- `-vv`: also log each file linked or moved.
- `--version`, `--help`.

## `perch sync`

1. Walks every **active** module, file by file. It honors `.perchignore` and includes hidden files.
2. For each file at `<dotfiles>/<module>/<rel>`, it computes the target `~/<rel>`:
   - If the target is already a symlink to that exact source, it's skipped. That's why `sync` is safe to re-run.
   - If the target exists in any other form (a real file, or a link elsewhere), it prints `<target> is not mine. i am not touching it` and counts a conflict.
3. If there are any conflicts, perch prints `N conflicts found. resolve before syncing. no changes applied` and **nothing is written**.
4. Otherwise it creates the parent directories (as real directories) and an absolute symlink for each file, then prints `N modules synced`.

- If an ancestor directory in `~` is itself a symlink (for example a directory-level GNU Stow link), the targets resolve through it and show up as conflicts.
- If an active module's directory doesn't exist, the walk fails with an IO error.

## `perch add <module> <path>...`

The module comes first; after it you can pass any number of paths, which can be relative or absolute.

1. Directories are walked recursively. It honors `.perchignore` files **inside the source tree**, but not the repo-root `.perchignore`.
2. Each entry is resolved with `canonicalize`. If an entry is a symlink, the **real file it points to** is moved, and the new link is created at the real path, not at the original link.
3. These entries are skipped:
   - Broken symlinks (`skipping broken symlink ...`).
   - Files already inside the dotfiles repo (skipped silently).
   - Files outside `$HOME` (`... is outside your home directory — skipping`).
   - Duplicates reached through several paths.
4. For every file, the destination is `<dotfiles>/<module>/<path relative to $HOME>`. If any destination already exists, perch prints `... is already in my dotfiles directory — not overwriting it`, then `N conflict found. resolve before adding them`, and **nothing is moved**.
5. Otherwise each file is moved with `rename` and a symlink is left in its place. The final message is `N files added to ~/dotfiles and symlinked them back`.

- `rename` fails across filesystems, for example when `~` and the repo are on different mounts. A failure in the middle leaves the files before it already moved.
- The module name isn't validated. Check that the module is in the config if this file should be linked on other machines.

## `perch update`

1. Runs `git -C <dotfiles> pull`, with git's output passed through. If the pull fails, perch stops there.
2. Looks for `<module>/.config/perch/config.toml` in the active modules.
3. Runs `sync` with the module list that was loaded **before** the pull. If the pull added or changed modules, run `perch sync` again afterwards.
4. If the in-repo config names a different `dotfiles_dir`, it runs `move-dir` to that location automatically.

## `perch status`

Runs `git -C <dotfiles> status`.

## `perch move-dir <path>`

1. Copies every file in the repo to `<path>`. Symlinks inside the repo become real files, and empty directories are dropped.
2. Re-points the matching symlinks in `~`. Any symlink at a matching path is replaced, even one that pointed somewhere else.
3. Deletes the old directory and saves the new `dotfiles_dir` into the loaded config.
4. If it fails, it deletes `<path>`. **Never point it at an existing directory that holds data.**

Run it only when the user asks explicitly. Make sure the repo is committed and pushed first.

## `perch self-upgrade`

Downloads the latest GitHub release for this platform and replaces the running binary. It doesn't upgrade when the tag string equals the current version.

## Config side effect

After **any** successful command, perch writes the loaded config back as TOML. This drops comments and normalizes formatting. If the config is a symlink into the repo, the change appears as a git diff.
