# Configuration

This document assumes you have [installed](INSTALLATION.md) the daemon.

## Editing the config file

Macro definitions are stored in `~/.config/g11-macro-daemon/key_bindings.ron` (or equivalent [XDG_CONFIG](https://specifications.freedesktop.org/basedir-spec/latest/) location).

This file will be automatically created the first time that the daemon runs.

After making changes to the file, you must restart the service to apply them:
```bash
systemctl --user restart g11-macro-daemon
```

## Defining macros

Within `key_bindings.ron`, you may create an entry for each individual 'G' key you wish to program.
* The overall format of the file is a [ron](https://docs.rs/ron/latest/ron) List of [KeyBinding](https://docs.rs/g11_macro_daemon/latest/config/struct.KeyBinding.html) entries.
* If more than one entry is defined for a given M/G combination, only the last one will be used.

Consider the following example, which programs the G13 key in the M1 bank to simulate Ctrl+w:
```ron
KeyBinding(
    m: 1,
    g: 13,
    on: Press,
    script: [
        Key(Control, Press),
        Key(Unicode('w'), Click),
        Key(Control, Release),
    ],
),
```
Note that when scripting modifier keys like Control/Shift/etc., you should include a step for each to be released at the end.

While keystrokes are the most obvious candidates for steps in your script,
you may include any type of [Enigo Token](https://docs.rs/enigo/0.3.0/enigo/agent/enum.Token.html). For example:
```ron
KeyBinding(
    m: 2,
    g: 16,
    on: Press,
    script: [
        Text("Hello World"),
        Scroll(1, Vertical),
        Button(Right, Click),
    ],
),
```

## Appendix: Troubleshooting
* You can check the status of the service by running:
  ```bash
  systemctl --user status g11-macro-daemon
  ```
* You can inspect the service logs by running:
  ```bash
  journalctl --user -u g11-macro-daemon
  ```
