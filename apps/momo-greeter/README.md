Run with `--mock-users` to mock users when developing. This also uses mock authentication.

Authentication defaults to greetd unless mock authentication is explicitly requested with
`--mock-auth` or `--mock-users`.

The user session command defaults to `wayfire`. Override it with
`--session-command "command arg"`.

For Linux layout testing without a Wayland compositor, launch the greeter as a
regular window:

```sh
momo-greeter --standalone-test --mock-users
```
