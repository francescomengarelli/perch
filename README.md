![GitHub Release](https://img.shields.io/github/v/release/francescomengarelli/perch?display_name=release&style=flat-square&color=blue)
![License](https://img.shields.io/github/license/francescomengarelli/perch?style=flat-square&color=green)
![CI](https://img.shields.io/github/actions/workflow/status/francescomengarelli/perch/release.yml?style=flat-square)

# perch

your dotfiles, settled into place.

## Installation

```sh
curl -fsSL https://get.mengarelli.eu/perch | sh
```

Or build from source:

```sh
git clone git@github.com:francescomengarelli/perch.git
cd perch
cargo install --root ~/.local --path .
```

## Claude Code plugin

Perch ships a [Claude Code](https://claude.com/claude-code) plugin that teaches
Claude how to set up and manage your dotfiles with perch:

```
/plugin marketplace add francescomengarelli/perch
/plugin install perch@perch
```

It provides two skills: `perch:setup` (install, bootstrap, migrate from Stow
and friends) and `perch:dotfiles` (adopt, sync, update, troubleshoot).

## How it works

Perch manages dotfiles by stowing modules from your dotfiles repo into your
home directory. A module is a folder — its contents get symlinked to the
same relative path under `~`.

`~/dotfiles/macos/.config/nvim/init.lua` → `~/.config/nvim/init.lua`, but only on macOS.

Without a config file, Perch runs with sensible defaults:

| Module   | Stowed when |
| -------- | ----------- |
| `common` | always      |
| `macos`  | macOS only  |
| `linux`  | Linux only  |

Dotfiles directory defaults to `~/dotfiles`. Once you provide a config file,
these defaults are replaced entirely — only the modules you define are stowed.

### Ignoring files

Use `.perchignore` to keep files or directories from being symlinked. Its
patterns use the same syntax as `.gitignore`; Git's own ignore files are not
used by Perch.

Put one at the root of the dotfiles repository (`~/dotfiles/.perchignore`) for
rules loaded with every active module; its patterns are relative to the
repository root (e.g. `common/.config/nvim/lazy-lock.json`). You can also put
one anywhere inside a module. For example, `~/dotfiles/common/.config/.perchignore`
applies only to the `.config` directory and its descendants. Both `sync` and
`add` honor them, and `.perchignore` files themselves are never symlinked.

```gitignore
# In ~/dotfiles/common/.config/.perchignore:
# do not stow local Neovim state
nvim/.local/
```

## Commands

| Command                   | What it does                                            |
| ------------------------- | ------------------------------------------------------- |
| `perch sync`              | Symlink all active modules to their target locations    |
| `perch sync --dry-run`    | Show what `sync` would link and what conflicts          |
| `perch update`            | Pull latest changes from remote, then re-sync           |
| `perch status`            | Run `git status` in the dotfiles repo                   |
| `perch add <m> <path>...` | Move files into a module and symlink them back          |
| `perch remove <path>...`  | Stop managing files: move them back out of the repo     |
| `perch prune [--dry-run]` | Remove stale links (file gone, or module inactive)      |
| `perch move-dir <path>`   | Move the dotfiles directory to a new location           |
| `perch self-upgrade`      | Fetch latest perch release and update to it             |

`sync` never overwrites anything: if a target already exists and isn't one of
perch's links, nothing is changed until you resolve it.

Perch remembers the links it creates in `$XDG_STATE_HOME/perch/links`
(default `~/.local/state/perch/links`) so `prune` can find stale ones. If you
upgraded from an older version, run `perch sync` once to fill it in.

## Configuration

Perch looks for a config at `~/.config/perch/config.toml`.
Pass `--config <path>` to override.

```toml
dotfiles_dir = "~/dotfiles"

[[module]]
name = "linux"
when = "uname -s | grep -q Linux"

[[module]]
name = "work"
when = "hostname | grep -q work-machine"
```

`when` is a shell command — exit `0` means stow it. Condition on anything:
OS, hostname, environment variables, presence of a file, whatever.

A relative `dotfiles_dir` is relative to your home directory. Perch only
writes to the config file when the dotfiles directory moves (`move-dir`, or an
`update` that pulls a new `dotfiles_dir`), and then edits just that line —
your comments and formatting are kept.

## First-time setup

Your Perch config can live inside your dotfiles repo itself.
On a fresh machine:

```sh
git clone <your-dotfiles-repo> ~/dotfiles
perch sync --config ~/dotfiles/common/.config/perch/config.toml
```

After the first sync, `~/.config/perch/config.toml` is live and
`perch sync` works without `--config` from then on.

> [!NOTE]
> Make sure to clone in the directory specified in your `config.toml`, or Perch won't be
> able to find your dotfiles the first time.
