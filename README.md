# dot

A minimal dotfiles manager for macOS and Linux.

## Installation

```sh
curl -fsSL https://raw.githubusercontent.com/francescomengarelli/dot/main/install.sh | sh
```

Or build from source:

```sh
git clone git@github.com:francescomengarelli/dot.git
cd dot
cargo install --root ~/.local --path .
```

## How it works

dot manages dotfiles by stowing modules from your dotfiles repo into your
home directory. A module is a folder - inside it get symlinked to the
same relative path under `~`.

`~/dotfiles/macos/.config/nvim/init.lua` → `~/.config/nvim/init.lua`, but only on macOS.

Without a config file, dot runs with hardcoded defaults:

| Module   | Stowed when |
| -------- | ----------- |
| `common` | always      |
| `macos`  | macOS only  |
| `linux`  | Linux only  |

Dotfiles directory defaults to `~/dotfiles`. Once you provide a config file,
these defaults are replaced entirely — only the modules you define are stowed.

## Commands

| Command              | What it does                                         |
| -------------------- | ---------------------------------------------------- |
| `dot sync`           | Symlink all active modules to their target locations |
| `dot update`         | Pull latest changes from the remote, then re-sync    |
| `dot status`         | Run `git status` in the dotfiles repo                |
| `dot add <m> <path>` | Move files into a module and symlink them back       |

## Configuration

dot looks for a config at `~/.config/dot/config.toml`.
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

## First-time setup

Your dot config can live inside your dotfiles repo itself.
On a fresh machine:

```sh
git clone <your-dotfiles-repo> ~/dotfiles
dot sync --config ~/dotfiles/common/.config/dot/config.toml
```

After the first sync, `~/.config/dot/config.toml` is live and
`dot sync` works without `--config` from then on.
