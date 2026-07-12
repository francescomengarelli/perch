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

dot manages your dotfiles by stowing modules from your dotfiles repo into your home directory. A module is just a folder — any file inside it gets symlinked to the same relative path under `~`.

For example, `~/dotfiles/macos/.config/dot/config.toml` becomes `~/.config/dot/config.toml`, but only on macOS.

By default, without any config, dot ships with three built-in modules:

| Module | When it's stowed |
|--------|-----------------|
| `common` | always |
| `macos` | macOS only |
| `linux` | Linux only |

## Configuration

dot looks for a config file at `~/.config/dot/config.toml`. You can also pass one explicitly with `--config`.

```toml
dotfiles_dir = "~/dotfiles"

[[module]]
name = "linux"
when = "uname -s | grep -q Linux"

[[module]]
name = "work"
when = "hostname | grep -q work-machine"
```

`when` is a shell command — if it exits with `0`, the module gets stowed. This gives you full flexibility to condition on anything: OS, hostname, environment variables, whatever.

## First-time setup

You can keep your dot config inside your dotfiles repo itself. On a fresh machine:

```sh
git clone <your-dotfiles-repo> ~/dotfiles
dot sync --config ~/dotfiles/common/.config/dot/config.toml
```

After the first sync, the config is stowed at `~/.config/dot/config.toml` and you can just run `dot sync` from then on.

## Requirements

- `git`
