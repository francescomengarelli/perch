# perch config and .perchignore

## Config file

The config is TOML, found at `~/.config/perch/config.toml` or passed with `--config <path>`.

```toml
dotfiles_dir = "~/dotfiles"   # optional; defaults to ~/dotfiles. A relative path resolves against the cwd.

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
- Every active module needs its folder to exist in the repo, even if it's empty. Otherwise `sync` fails.
- perch rewrites this file after every successful command, which drops comments. Keep explanations somewhere else, such as a README in the repo.

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

### Keeping the config in the repo

The usual layout keeps the config in the repo itself, at `~/dotfiles/common/.config/perch/config.toml`. `sync` then links it to `~/.config/perch/config.toml`.

On a fresh machine, the first run needs `--config` pointing at that file, because the link doesn't exist yet:

```sh
perch sync --config ~/dotfiles/common/.config/perch/config.toml
```

After that first run, a plain `perch sync` works.

## .perchignore

`.perchignore` uses gitignore syntax: globs, `dir/`, `!negation`, `#` comments. perch never reads `.gitignore`, git's global excludes, or `.git/info/exclude`. Hidden files are included unless they're ignored.

| Location | Applies to | Used by |
|---|---|---|
| `~/dotfiles/.perchignore` (repo root) | every active module. Patterns are relative to the repo root, so `common/.config/nvim/lazy-lock.json` or an unanchored `.DS_Store` | `sync` only |
| `~/dotfiles/<module>/<dir>/.perchignore` | that directory and everything below it | `sync` |
| a `.perchignore` inside a directory you `add` | that directory and everything below it | `add` |

Gotchas:
- A `.perchignore` inside a module is a file in that module, so `sync` links it into `~` too. To avoid this, list `.perchignore` in the root ignore file.
- `.git` at the repo root is never walked, because modules are subfolders. Any other junk inside a module, such as `.DS_Store` or caches, gets linked unless it's ignored.
- A file that's already linked stays linked after you add an ignore rule for it. Remove the stale link by hand.

Example root `.perchignore`:

```gitignore
.DS_Store
.perchignore
README.md
```
