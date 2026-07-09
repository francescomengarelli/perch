# dot

A minimal dotfiles manager for macOS and Linux.

## Install

```sh
curl -sSf https://raw.githubusercontent.com/francescomengarelli/dot/main/install.sh | sh
```

Then clone your dotfiles and sync:

```sh
git clone <your-dotfiles-repo> ~/dotfiles
~/.local/bin/dot sync
```

## Commands

- `dot sync` — stow the right modules for the current machine
- `dot update` — pull latest dotfiles and re-sync
- `dot status` — git status of the dotfiles repo

## Dotfiles structure

Modules are subdirectories of `~/dotfiles`. `dot sync` automatically picks the right ones:

| Module     | When             |
| ---------- | ---------------- |
| `common`   | always           |
| `macos`    | macOS only       |
| `linux`    | Linux only       |
| `hyprland` | Linux + Hyprland |

## Requirements

- Rust (installed by the install script)
- `git`
