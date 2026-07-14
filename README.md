![GitHub Release](https://img.shields.io/github/v/release/francescomengarelli/perch?display_name=release&style=flat-square&color=blue)
![License](https://img.shields.io/github/license/francescomengarelli/perch?style=flat-square&color=green)
![CI](https://img.shields.io/github/actions/workflow/status/francescomengarelli/perch/release.yml?style=flat-square)

# perch

your dotfiles, settled into place.

## Installation

```sh
curl -fsSL https://raw.githubusercontent.com/francescomengarelli/perch/main/install.sh | sh
```

Or build from source:

```sh
git clone git@github.com:francescomengarelli/perch.git
cd perch
cargo install --root ~/.local --path .
```

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

## Commands

| Command                 | What it does                                         |
| ----------------------- | ---------------------------------------------------- |
| `perch sync`            | Symlink all active modules to their target locations |
| `perch update`          | Pull latest changes from remote, then re-sync        |
| `perch status`          | Run `git status` in the dotfiles repo                |
| `perch add <m> <path>`  | Move files into a module and symlink them back       |
| `perch move-dir <path>` | Move the dotfiles directory to a new location        |
| `perch self-upgrade`    | Feth latest perch release and update to it           |

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
