# perch config and .perchignore

## Config file

The config is TOML, found at `~/.config/perch/config.toml` or passed with `--config <path>`.

```toml
# comments are kept: perch only ever edits the dotfiles_dir line
dotfiles_dir = "~/dotfiles"   # optional; defaults to ~/dotfiles. Relative paths are relative to $HOME.

[[module]]
name = "common"               # no `when` = always active

[[module]]
name = "linux"
when = "uname -s | grep -q Linux"

[[module]]
name = "work"
when = "hostname | grep -q work-laptop"
```

- `name` is the module's folder name at the top level of `dotfiles_dir`.
- `when` is run with `sh -c`, with stdout and stderr discarded. Exit 0 means the module is active.
- With a config present, **only** the modules listed in it exist. The built-in defaults (`common`, `linux`, `macos`) are gone, so list them again if you want them.
- An active module without a folder is skipped with a warning.
- perch writes this file only when `dotfiles_dir` changes (`move-dir`, or an `update` that relocates the repo), and then it edits just that line.

### Useful `when` conditions

| Condition | `when` |
|---|---|
| macOS | `uname -s \| grep -q Darwin` |
| Linux | `uname -s \| grep -q Linux` |
| Specific host | `[ "$(hostname -s)" = "mybox" ]` |
| Program installed | `command -v nvim` |
| Env var set | `[ -n "$WORK" ]` |
| Marker file present | `test -f ~/.work-machine` |
| WSL | `grep -qi microsoft /proc/version` |

To check what's active, run each `when` by hand: `sh -c '<when>'; echo $?`.

To clean up after deactivating a module or removing it from the config, run `perch prune --dry-run`, then `perch prune`.

### Keeping the config in the repo

The usual layout keeps the config in the repo itself, at `~/dotfiles/common/.config/perch/config.toml`. `sync` then links it to `~/.config/perch/config.toml`.

On a fresh machine, the first run needs `--config` pointing at that file, because the link doesn't exist yet:

```sh
perch sync --config ~/dotfiles/common/.config/perch/config.toml
```

After that first run, a plain `perch sync` works.

## .perchignore

`.perchignore` uses gitignore syntax: globs, `dir/`, `!negation`, `#` comments. perch never reads `.gitignore`, git's global excludes, or `.git/info/exclude`. Hidden files are included unless they're ignored. `.perchignore` files themselves are never linked into `~`.

| Location | Applies to | Used by |
|---|---|---|
| `~/dotfiles/.perchignore` (repo root) | every module. Patterns are relative to the repo root, so `common/.config/nvim/lazy-lock.json` or an unanchored `.DS_Store` | `sync` and `add` |
| `~/dotfiles/<module>/<dir>/.perchignore` | that directory and everything below it | `sync` |
| a `.perchignore` inside a directory you `add` | that directory and everything below it; it moves into the repo along with the files | `add` |

Gotchas:
- A root pattern written relative to the module (`/.config/foo`) won't match. Write it relative to the repo root (`common/.config/foo`), or leave it unanchored.
- `.git` at the repo root is never walked, because modules are subfolders.
- A file that's already linked stays linked after you add an ignore rule for it. Remove that link by hand with `rm` (it's only a symlink); `prune` doesn't handle ignored files.

Example root `.perchignore`:

```gitignore
.DS_Store
README.md
```
