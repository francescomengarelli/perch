#!/bin/sh
set -e

# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"

# build and install dot
git clone https://github.com/francescomengarelli/dot /tmp/dot
cargo install --path /tmp/dot --root "$HOME/.local"
rm -rf /tmp/dot

echo ""
echo "dot installed to ~/.local/bin/dot"
echo ""
echo "Next steps:"
echo "  1. Add ~/.local/bin to your PATH if it isn't already"
echo "  2. Clone your dotfiles: git clone <your-dotfiles-repo> ~/dotfiles"
echo "  3. Run: ~/.local/bin/dot sync"
