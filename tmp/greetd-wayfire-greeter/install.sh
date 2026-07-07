#!/bin/sh
set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

PREFIX="${PREFIX:-/usr/local}"
LIBDIR="${LIBDIR:-/usr/lib/momo-greeter}"
LIBEXECDIR="${LIBEXECDIR:-/usr/libexec}"
GREETD_CONFIG="${GREETD_CONFIG:-/etc/greetd/config.toml}"
GREETD_USER="${GREETD_USER:-_greetd}"

GREETER_SOURCE="${GREETER_SOURCE:-}"
SHELL_SOURCE="${SHELL_SOURCE:-}"

find_binary() {
  NAME="$1"

  if [ -x "${SCRIPT_DIR}/${NAME}" ]; then
    printf '%s\n' "${SCRIPT_DIR}/${NAME}"
    return 0
  fi

  if [ -x "${SCRIPT_DIR}/bin/${NAME}" ]; then
    printf '%s\n' "${SCRIPT_DIR}/bin/${NAME}"
    return 0
  fi

  return 1
}

if [ -z "${GREETER_SOURCE}" ]; then
  GREETER_SOURCE="$(find_binary momo-greeter)" || {
    echo "error: momo-greeter not found next to install.sh or in bin/" >&2
    exit 1
  }
fi

if [ -z "${SHELL_SOURCE}" ]; then
  SHELL_SOURCE="$(find_binary momo-shell)" || {
    echo "error: momo-shell not found next to install.sh or in bin/" >&2
    exit 1
  }
fi

if [ "$(id -u)" -ne 0 ]; then
  echo "error: run as root, for example: sudo ./install.sh" >&2
  exit 1
fi

GREETER_TARGET="${PREFIX}/bin/momo-greeter"
SHELL_TARGET="${PREFIX}/bin/momo-shell"
WRAPPER_TARGET="${LIBEXECDIR}/momo-greeter-wayfire"
WAYFIRE_CONFIG_TARGET="${LIBDIR}/wayfire.ini"

echo "Installing momo greeter test session"
echo "  momo-greeter: ${GREETER_SOURCE} -> ${GREETER_TARGET}"
echo "  momo-shell:   ${SHELL_SOURCE} -> ${SHELL_TARGET}"
echo "  wrapper:      ${WRAPPER_TARGET}"
echo "  wayfire ini:  ${WAYFIRE_CONFIG_TARGET}"
echo "  greetd conf:  ${GREETD_CONFIG}"
echo "  greetd user:  ${GREETD_USER}"

rm -f /usr/local/libexec/momo-greeter-wayfire
rm -f /usr/local/libexec/momo-greeter-client
rm -f "${LIBEXECDIR}/momo-greeter-client"

install -d -m 0755 "${PREFIX}/bin" "${LIBDIR}" "${LIBEXECDIR}" "$(dirname -- "${GREETD_CONFIG}")"
install -m 0755 "${GREETER_SOURCE}" "${GREETER_TARGET}"
install -m 0755 "${SHELL_SOURCE}" "${SHELL_TARGET}"
install -m 0755 "${SCRIPT_DIR}/libexec/momo-greeter-wayfire" "${WRAPPER_TARGET}"
install -m 0644 "${SCRIPT_DIR}/wayfire/wayfire.ini" "${WAYFIRE_CONFIG_TARGET}"

if [ -f "${GREETD_CONFIG}" ]; then
  BACKUP="${GREETD_CONFIG}.bak.$(date +%Y%m%d%H%M%S)"
  cp "${GREETD_CONFIG}" "${BACKUP}"
  echo "Backed up existing greetd config to ${BACKUP}"
fi

cat >"${GREETD_CONFIG}" <<EOF
[terminal]
vt = 7

[default_session]
command = "${WRAPPER_TARGET}"
user = "${GREETD_USER}"
EOF

echo
echo "Installed. To test now:"
echo "  systemctl restart greetd"
echo "  journalctl -u greetd -b --no-pager -o cat"
echo "  cat /tmp/momo-greeter.log"
echo
echo "This installer does not enable or restart greetd automatically."
