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
* The overall format of the file is a [ron](https://docs.rs/ron/latest/ron) List of [KeyBinding](https://github.com/rs017991/g11-macro/blob/eaba13e0adfa73fa4d0023d55426d748caa84b30/g11-macro-daemon/src/config.rs#L21-L30) entries.
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
you may include any type of [Enigo Token](https://docs.rs/enigo/0.5.*/enigo/agent/enum.Token.html). For example:
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
  journalctl --user -u g11-macro-daemon -r
  ```
  * `-r` reverses the order, so newest lines are on top.
  * Keep an eye out for any line like _"Unable to load config: Parsing"_.
    This indicates that there is a mistake in your config file. Study the line to understand where the problem lies.
    You might have to scroll to the right to see the full line.
* Tip: If you are working through problems with your bindings,
  it may be more convenient to execute the binary directly at `~/.cargo/bin/g11-macro-daemon` in your terminal foreground,
  rather than constantly restarting the service and having to pull up the logs.
