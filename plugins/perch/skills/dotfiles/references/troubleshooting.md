# perch troubleshooting

## `<path> is not mine. i am not touching it` / `N conflicts found. resolve before syncing. no changes applied`

A target in `~` already exists and isn't perch's link to that exact source. Nothing was changed.

Check what the target is with `ls -l <path>` and `readlink <path>`, then use the matching fix:
- **A real file:** compare it with the repo version (`diff <path> ~/dotfiles/<module>/<rel>`). Ask the user which version wins.
  - If the repo version wins: `mv <path> <path>.bak`.
  - If the local version wins: `cp <path> ~/dotfiles/<module>/<rel>`, then `mv <path> <path>.bak`.
  - Then run `perch sync` again.
- **A symlink to an old repo location** (for example after hand-moving the repo): `rm <path>` (it's only a link), then sync again.
- **A path under a directory symlink** (for example from GNU Stow): remove that directory link first (`stow -D <pkg>`, or `rm <dirlink>`).
- **Two modules providing the same file:** both are active and map to the same `~/<rel>`. Keep the file in only one module, or make the `when` conditions mutually exclusive.

## `IO error for operation on ~/dotfiles/<module>: No such file or directory`

An active module has no folder in the repo. Common causes:
- The built-in defaults (`linux` or `macos`) are active but the repo has no such folder.
- The config names a module that hasn't been created yet.
- `dotfiles_dir` is wrong.

Fix it with `mkdir ~/dotfiles/<module>`, or remove the module from the config.

## `i couldn't find a config file at X - does it exist?`

The `--config` path doesn't exist. Check the path; perch expands `~`, but other variables aren't expanded unless your shell does it.

## A module's files aren't being linked

- Its `when` failed. Test it with `sh -c '<when>'; echo $?`.
- A config file exists and doesn't list the module. With a config, the defaults are dropped.
- perch is reading a different config than you think. Check `ls -l ~/.config/perch/config.toml`, and pass `--config` explicitly if needed.
- The file is matched by a `.perchignore`.
- `perch update` synced with the old module list. Run `perch sync` again.

## `N conflict found. resolve before adding them` (from `add`)

The destination in `~/dotfiles/<module>/` already exists. Either:
- the file is already tracked in that module, so check whether `~/<rel>` should just be linked (`perch sync`), or
- pick a different module, or move the repo copy aside after confirming with the user.

## `... is outside your home directory — skipping`

perch only manages paths under `$HOME`. If the path is a symlink, its **real** target is what counts.

## `i couldn't move X to Y`

`rename` failed, often because the repo and `~` are on different filesystems. Files listed before the failure have already been moved and linked. Check `perch status` and the remaining files, then move the rest by hand (`mv` + `ln -s <repo-path> <home-path>`), or relocate the repo onto the same filesystem.

## The config shows a diff after every command

perch writes the config back after every successful command, dropping comments and reformatting. Commit the normalized version once and keep comments out of it. This is known behaviour, not corruption.

## Stale or dangling links

These come from files removed from the repo, renamed modules, or modules that are no longer active. perch never removes links. To find them:

```sh
find ~ -maxdepth 6 -xtype l -lname "$HOME/dotfiles/*" 2>/dev/null   # adjust if dotfiles_dir differs
```

Confirm the list with the user, then delete them with `rm`.

## `git pull didn't go well — check the output above`

`perch update` stops if `git pull` fails: local changes, a merge conflict, no upstream, or authentication. Resolve it with git in the repo (`git -C ~/dotfiles ...`), then run `perch update` or `perch sync` again.
