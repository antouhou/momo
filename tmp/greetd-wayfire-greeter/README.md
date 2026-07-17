# THIS IS A TEMPORARY TEST BUNDLE TO INSTALL ON AN UBUNTU SERVER VM FOR TESTING INTEGRATION.

# Momo Greeter greetd/Wayfire Test Bundle

This bundle runs `momo-greeter` inside a temporary Wayfire greeter compositor.

Layout:

- `/etc/greetd/config.toml`
- `/usr/lib/momo-greeter/wayfire.ini`
- `/usr/lib/momo-shell/wayfire.ini`
- `/usr/libexec/momo-greeter-wayfire`
- `/usr/libexec/momo-shell-wayfire`
- `/usr/libexec/momo-shell-session`
- `/usr/local/bin/momo-greeter`
- `/usr/local/bin/momo-shell`

## Files

- `greetd/config.toml`: greetd session config.
- `wayfire/wayfire.ini`: minimal greeter-only Wayfire config that autostarts `momo-greeter`.
- `wayfire/momo-shell.ini`: user-session Wayfire config that loads the IPC plugins and autostarts Momo Shell.
- `libexec/momo-greeter-wayfire`: starts the temporary Wayfire compositor; Wayfire then starts `momo-greeter`.
- `libexec/momo-shell-wayfire`: starts the authenticated user's Momo Wayfire session.
- `libexec/momo-shell-session`: starts Momo Shell and writes its dedicated process log.

## Install On Target

Put `momo-greeter` and `momo-shell` either next to `install.sh` or in `bin/`
under this directory, then run:

```sh
sudo ./install.sh
```

The installer copies:

- `momo-greeter` to `/usr/local/bin/momo-greeter`
- `momo-shell` to `/usr/local/bin/momo-shell`
- `libexec/momo-greeter-wayfire` to `/usr/libexec/momo-greeter-wayfire`
- `libexec/momo-shell-wayfire` to `/usr/libexec/momo-shell-wayfire`
- `libexec/momo-shell-session` to `/usr/libexec/momo-shell-session`
- `wayfire/wayfire.ini` to `/usr/lib/momo-greeter/wayfire.ini`
- `wayfire/momo-shell.ini` to `/usr/lib/momo-shell/wayfire.ini`
- a generated greetd config to `/etc/greetd/config.toml`

If `/etc/greetd/config.toml` already exists, the installer backs it up to a
timestamped `.bak.*` file before replacing it.

The default greetd user is `_greetd`. Override it if your distro uses another
user:

```sh
sudo GREETD_USER=greeter ./install.sh
```

Override install paths if needed:

```sh
sudo PREFIX=/usr ./install.sh
```

Point to binaries outside this directory if needed:

```sh
sudo GREETER_SOURCE=/path/to/momo-greeter SHELL_SOURCE=/path/to/momo-shell ./install.sh
```

The installer does not enable or restart greetd automatically.

Manual install commands, if needed:

```sh
sudo rm -f /usr/local/libexec/momo-greeter-wayfire
sudo rm -f /usr/local/libexec/momo-greeter-client
sudo rm -f /usr/libexec/momo-greeter-client
sudo mkdir -p /etc/greetd /usr/lib/momo-greeter /usr/lib/momo-shell /usr/libexec
sudo cp greetd/config.toml /etc/greetd/config.toml
sudo cp wayfire/wayfire.ini /usr/lib/momo-greeter/wayfire.ini
sudo cp wayfire/momo-shell.ini /usr/lib/momo-shell/wayfire.ini
sudo install -m 0755 libexec/momo-greeter-wayfire /usr/libexec/momo-greeter-wayfire
sudo install -m 0755 libexec/momo-shell-wayfire /usr/libexec/momo-shell-wayfire
sudo install -m 0755 libexec/momo-shell-session /usr/libexec/momo-shell-session
sudo install -m 0755 momo-greeter /usr/local/bin/momo-greeter
sudo install -m 0755 momo-shell /usr/local/bin/momo-shell
```

Make sure `/etc/greetd/config.toml` uses the greetd service user for your distro.
On this VM that appears to be `_greetd`.

## Test

```sh
sudo systemctl restart greetd
sudo journalctl -u greetd -b --no-pager -o cat
sudo cat /tmp/momo-greeter.log
sudo cat /tmp/momo-shell.log
sudo cat /tmp/momo-wayfire.log
```

Use mock mode while testing by temporarily changing the `momo-greeter` command
in `wayfire/wayfire.ini` to add `--mock-users --mock-auth`.
