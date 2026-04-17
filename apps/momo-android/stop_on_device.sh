#!/usr/bin/env bash
set -euo pipefail

PACKAGE_NAME="${1:-com.deko.demo}"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Missing required command: $1" >&2
    exit 1
  fi
}

require_command "adb"

if ! adb devices | awk 'NR > 1 && $2 == "device" { found = 1 } END { exit(found ? 0 : 1) }'; then
  echo "No connected adb device in \"device\" state." >&2
  echo "Check with: adb devices" >&2
  exit 1
fi

echo "Stopping app: ${PACKAGE_NAME}"
adb shell am force-stop "${PACKAGE_NAME}"
echo "App stopped."
