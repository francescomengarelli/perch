# perch troubleshooting

## `<path> is not mine. i am not touching it` / `N conflicts found. resolve before syncing. no changes applied`

A target in `~` already exists and isn't perch's link to that exact source. Nothing was changed. `perch sync --dry-run` lists every conflict at once.

Check what the target is with `ls -l <path>` and `readlink <path>`, then use the matching fix:
- **A real file:** compare it with the repo version (`diff <path> ~/dotfiles/<module>/<rel>`). Ask the user which version wins.
  - If the repo version wins: `mv <path> <path>.bak`.
  - If the local version wins: `cp <path> ~/dotfiles/<module>/<rel>`, then `mv <path> <path>.bak`.
  - Then run `perch sync` again.
- **A symlink to an old repo location** (for example after hand-moving the repo instead of using `move-dir`): `rm <path>` (it's only a link), then sync again.
- **A path under a directory symlink** (for example from GNU Stow): remove that directory link first (`stow -D <pkg>`, or `rm <dirlink>`).
- **Two modules providing the same file:** both are active and map to the same `~/<rel>`. Keep the file in only one module, or make the `when` conditions mutually exclusive.

## `module 'X' is active but … doesn't exist — skipping`

This is a warning, not an error. Either create the folder (`mkdir ~/dotfiles/X`), or remove the module from the config. It usually means a built-in default (`linux` or `macos`) is active without a folder, or `dotfiles_dir` is wrong.

## `i couldn't find a config file at X - does it exist?`

The `--config` path doesn't exist. Check the path; perch expands `~`, but other variables aren't expanded unless your shell does it.

## A module's files aren't being linked

- Its `when` failed. Test it with `sh -c '<when>'; echo $?`.
- A config file exists and doesn't list the module. With a config, the defaults are dropped.
- perch is reading a different config than you think. Check `ls -l ~/.config/perch/config.toml`, and pass `--config` explicitly if needed.
- The file is matched by a `.perchignore`.
- `perch sync --dry-run` shows exactly what would be linked.

## `N conflict found. resolve before adding them` (from `add`)

The destination in `~/dotfiles/<module>/` already exists. Either:
- the file is already tracked in that module, so check whether `~/<rel>` should just be linked (`perch sync`), or
- pick a different module, or move the repo copy aside after confirming with the user.

## `module 'X' isn't in your config` / `isn't active on this machine` (from `add`)

The files were still added, but `sync` won't link that module (anywhere, or on this machine). Add the module to the config, or `perch remove` the files and add them to the right module.

## `... is outside your home directory — skipping`

perch only manages paths under `$HOME`. If the path is a symlink, its **real** target is what counts.

## `… isn't managed by me — skipping` (from `remove`)

The path isn't a perch symlink into the dotfiles dir. Check it with `readlink`. It's already a real file, or it's a link made by something else.

## `no link record yet — run perch sync once` (from `prune`)

The link record is new in perch 0.10. One `perch sync` records every current link. Links left over from before the upgrade, whose repo files were already gone at that point, aren't recorded. Find those with:

```sh
find ~ -maxdepth 6 -xtype l -lname "$HOME/dotfiles/*" 2>/dev/null   # adjust if dotfiles_dir differs
```

Confirm the list with the user, then remove them with `rm`.

## `… already exists — pick a path that doesn't` (from `move-dir`)

`move-dir` won't merge into, or overwrite, an existing non-empty directory. Choose a new path, or empty that directory first (after asking the user).

## `git pull didn't go well — check the output above`

`perch update` stops if `git pull` fails: local changes, a merge conflict, no upstream, or authentication. Resolve it with git in the repo (`git -C ~/dotfiles ...`), then run `perch update` or `perch sync` again.
