#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd)"

APK_PATH="${REPO_ROOT}/target/release/apk/momo-shell.apk"
PACKAGE_NAME="com.momo.android"

: "${CARGO_APK_RELEASE_KEYSTORE:=${HOME}/.android/debug.keystore}"
: "${CARGO_APK_RELEASE_KEYSTORE_PASSWORD:=android}"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Missing required command: $1" >&2
    exit 1
  fi
}

require_command "cargo"
require_command "adb"

if ! command -v cargo-apk >/dev/null 2>&1 && ! cargo --list 2>/dev/null | awk '$1 == "apk" { found = 1 } END { exit(found ? 0 : 1) }'; then
  echo "Missing required cargo subcommand: cargo-apk" >&2
  echo "Install it with: cargo install cargo-apk" >&2
  exit 1
fi

if [[ ! -f "${CARGO_APK_RELEASE_KEYSTORE}" ]]; then
  echo "Keystore not found at: ${CARGO_APK_RELEASE_KEYSTORE}" >&2
  exit 1
fi

if ! adb devices | awk 'NR > 1 && $NF == "device" { found = 1 } END { exit(found ? 0 : 1) }'; then
  echo "No connected adb device in \"device\" state." >&2
  echo "Check with: adb devices" >&2
  exit 1
fi

cd "${REPO_ROOT}"

echo "Building momo_android release APK..."
CARGO_APK_RELEASE_KEYSTORE="${CARGO_APK_RELEASE_KEYSTORE}" \
CARGO_APK_RELEASE_KEYSTORE_PASSWORD="${CARGO_APK_RELEASE_KEYSTORE_PASSWORD}" \
cargo apk build -p momo_android --release

echo "Installing APK on connected device..."
adb install -r "${APK_PATH}"

echo "Launching app..."
adb shell monkey -p "${PACKAGE_NAME}" 1
