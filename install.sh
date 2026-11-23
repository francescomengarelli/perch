#!/bin/sh
set -e

BASE="https://get.mengarelli.eu/perch"
BIN="perch"
INSTALL_DIR="${HOME}/.local/bin"

# --- Detect OS ---
case "$(uname -s)" in
  Linux)  OS="linux" ;;
  Darwin) OS="macos" ;;
  *)
    echo "unsupported OS: $(uname -s)"
    exit 1
    ;;
esac

# --- Detect architecture ---
case "$(uname -m)" in
  x86_64)           ARCH="x86_64" ;;
  arm64 | aarch64)  ARCH="aarch64" ;;
  *)
    echo "unsupported architecture: $(uname -m)"
    exit 1
    ;;
esac

# --- Map to release target triple ---
if [ "$OS" = "linux" ]; then
  # must match the targets built in .github/workflows/release.yml
  case "$ARCH" in
    x86_64)  TARGET="x86_64-unknown-linux-musl" ;;
    aarch64) TARGET="aarch64-unknown-linux-gnu" ;;
  esac
elif [ "$OS" = "macos" ]; then
  TARGET="${ARCH}-apple-darwin"
fi

# --- Resolve latest release tag ---
LATEST=$(curl -fsSL "${BASE}/latest" || true)

if [ -z "$LATEST" ]; then
  echo "couldn't find the latest release — is ${BASE} reachable?"
  exit 1
fi

ASSET="${BIN}-${LATEST}-${TARGET}"
URL="${BASE}/download/${LATEST}/${ASSET}"

# --- Download ---
echo "fetching ${BIN} ${LATEST} for ${TARGET}..."
mkdir -p "$INSTALL_DIR"
curl -fsSL "$URL" -o "${INSTALL_DIR}/${BIN}"
chmod +x "${INSTALL_DIR}/${BIN}"
echo "${BIN} ${LATEST} is in place at ${INSTALL_DIR}/${BIN}"

# --- PATH hint ---
case ":${PATH}:" in
  *":${INSTALL_DIR}:"*)
    ;;
  *)
    echo ""
    echo "${INSTALL_DIR} isn't in your PATH yet."
    echo "add this to your shell config (~/.bashrc, ~/.zshrc, etc.):"
    echo ""
    echo '  export PATH="${HOME}/.local/bin:${PATH}"'
    echo ""
    ;;
esac

echo "all settled. run: ${BIN}"
