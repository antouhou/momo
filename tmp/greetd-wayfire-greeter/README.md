# THIS IS A TEMPORARY TEST BUNDLE TO INSTALL ON AN UBUNTU SERVER VM FOR TESTING INTEGRATION.

# Momo Greeter greetd/Wayfire Test Bundle

This bundle runs `momo-greeter` inside a temporary Wayfire greeter compositor.

Layout:

- `/etc/greetd/config.toml`
- `/usr/lib/momo-greeter/wayfire.ini`
- `/usr/libexec/momo-greeter-wayfire`
- `/usr/local/bin/momo-greeter`
- `/usr/local/bin/momo-shell`

## Files

- `greetd/config.toml`: greetd session config.
- `wayfire/wayfire.ini`: minimal greeter-only Wayfire config.
- `libexec/momo-greeter-wayfire`: starts temporary Wayfire, waits for it, runs `momo-greeter`, then stops temporary 
Wayfire.

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
- `wayfire/wayfire.ini` to `/usr/lib/momo-greeter/wayfire.ini`
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
sudo mkdir -p /etc/greetd /usr/lib/momo-greeter /usr/libexec
sudo cp greetd/config.toml /etc/greetd/config.toml
sudo cp wayfire/wayfire.ini /usr/lib/momo-greeter/wayfire.ini
sudo install -m 0755 libexec/momo-greeter-wayfire /usr/libexec/momo-greeter-wayfire
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
```

Use mock mode while testing by changing `MOMO_GREETER_AUTH_ARGS` in
`libexec/momo-greeter-wayfire` to `--mock-users --mock-auth`.
