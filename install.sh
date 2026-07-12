#!/bin/sh
set -e

REPO="francescomengarelli/dot"
BIN="dot"
INSTALL_DIR="${HOME}/.local/bin"

# --- Detect OS ---
case "$(uname -s)" in
  Linux)  OS="linux" ;;
  Darwin) OS="macos" ;;
  *)
    echo "Unsupported OS: $(uname -s)"
    exit 1
    ;;
esac

# --- Detect architecture ---
case "$(uname -m)" in
  x86_64)           ARCH="x86_64" ;;
  arm64 | aarch64)  ARCH="aarch64" ;;
  *)
    echo "Unsupported architecture: $(uname -m)"
    exit 1
    ;;
esac

# --- Map to release target triple ---
if [ "$OS" = "linux" ]; then
  TARGET="${ARCH}-unknown-linux-musl"
elif [ "$OS" = "macos" ]; then
  TARGET="${ARCH}-apple-darwin"
fi

# --- Resolve latest release tag ---
LATEST=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
  | grep '"tag_name"' \
  | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')

if [ -z "$LATEST" ]; then
  echo "Could not determine latest release."
  exit 1
fi

ASSET="${BIN}-${LATEST}-${TARGET}"
URL="https://github.com/${REPO}/releases/download/${LATEST}/${ASSET}"

# --- Download ---
echo "Installing ${BIN} ${LATEST} for ${TARGET}..."
mkdir -p "$INSTALL_DIR"
curl -fsSL "$URL" -o "${INSTALL_DIR}/${BIN}"
chmod +x "${INSTALL_DIR}/${BIN}"

# --- PATH hint ---
case ":${PATH}:" in
  *":${INSTALL_DIR}:"*)
    ;;
  *)
    echo ""
    echo "${INSTALL_DIR} is not in your PATH."
    echo "Add this to your shell config (~/.bashrc, ~/.zshrc, etc.):"
    echo ""
    echo '  export PATH="${HOME}/.local/bin:${PATH}"'
    echo ""
    ;;
esac

echo "Done. Run: ${BIN}"
