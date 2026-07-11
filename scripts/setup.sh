#!/usr/bin/env bash
#
# This script installs the system dependencies required to build this GPUI app.
# Windows users shouldn't need to run this, just Linux and MacOS users.
#
# This is the single source of truth for native (non-Rust) dependencies.
# It is used both by developers after cloning and by CI (see
# .github/workflows/release.yml). Keep the package lists here in sync with
# whatever the build actually needs, and do not duplicate them elsewhere.
#
# Usage:
#   ./scripts/setup.sh
#
# The script is idempotent: re-running it is safe.

set -euo pipefail

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

log()  { printf '\033[1;34m==>\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33mwarning:\033[0m %s\n' "$*" >&2; }
die()  { printf '\033[1;31merror:\033[0m %s\n' "$*" >&2; exit 1; }

# Run a command as root when we are not already root.
# In CI the runner is usually non-root but has passwordless sudo.
as_root() {
  if [ "$(id -u)" -eq 0 ]; then
    "$@"
  elif command -v sudo >/dev/null 2>&1; then
    sudo "$@"
  else
    die "this step needs root privileges but 'sudo' is not available; re-run as root"
  fi
}

have() { command -v "$1" >/dev/null 2>&1; }

# ---------------------------------------------------------------------------
# Per-platform installers
# ---------------------------------------------------------------------------

install_linux_apt() {
  log "Detected apt (Debian/Ubuntu). Installing dependencies..."
  as_root apt-get update
  as_root apt-get install -y \
    libwayland-dev \
    libxkbcommon-dev libxkbcommon-x11-dev \
    libxcb1-dev libxcb-shape0-dev libxcb-xfixes0-dev libxcb-render0-dev \
    libfontconfig-dev libfreetype6-dev \
    libssl-dev pkg-config
}

install_linux_dnf() {
  log "Detected dnf (Fedora/RHEL). Installing dependencies..."
  as_root dnf install -y \
    wayland-devel \
    libxkbcommon-devel libxkbcommon-x11-devel \
    libxcb-devel xcb-util-devel \
    fontconfig-devel freetype-devel \
    openssl-devel pkgconf-pkg-config
}

install_linux_pacman() {
  log "Detected pacman (Arch). Installing dependencies..."
  as_root pacman -Sy --needed --noconfirm \
    wayland \
    libxkbcommon libxkbcommon-x11 \
    libxcb \
    fontconfig freetype2 \
    openssl pkgconf
}

install_linux_zypper() {
  log "Detected zypper (openSUSE). Installing dependencies..."
  as_root zypper install -y \
    wayland-devel \
    libxkbcommon-devel libxkbcommon-x11-devel \
    libxcb-devel \
    fontconfig-devel freetype2-devel \
    libopenssl-devel pkg-config
}

install_linux() {
  if have apt-get;   then install_linux_apt
  elif have dnf;     then install_linux_dnf
  elif have pacman;  then install_linux_pacman
  elif have zypper;  then install_linux_zypper
  else
    die "no supported package manager found (need apt-get, dnf, pacman, or zypper).
Install these libraries manually: wayland, libxkbcommon (+x11), libxcb, fontconfig, freetype, openssl, pkg-config."
  fi
}

install_macos() {
  have brew || die "Homebrew is required on macOS. Install it from https://brew.sh"
  log "Detected macOS. Installing dependencies via Homebrew..."
  brew install librsvg
}

install_windows() {
  log "Windows detected. No system packages to install here."
  printf '    %s\n' \
    "GPUI's native dependencies (DirectWrite, Direct3D, the C runtime) ship" \
    "with Windows and the Windows SDK, so there's nothing for this script to do." \
    "" \
    "Make sure you have:" \
    "  - Rust toolchain:   winget install Rustlang.Rustup" \
    "  - MSVC build tools:  Visual Studio 'Desktop development with C++' workload" \
    "" \
    "Then just run: cargo run"
}

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

main() {
  have cargo || warn "cargo not found on PATH. Install Rust from https://rustup.rs"

  case "$(uname -s)" in
    Linux*)  install_linux ;;
    Darwin*) install_macos ;;
    MINGW*|MSYS*|CYGWIN*) install_windows; exit 0 ;;
    *) die "unsupported platform: $(uname -s)" ;;
  esac

  log "System dependencies installed. You can now run: cargo run"
}

main "$@"
