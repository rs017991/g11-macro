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
you may include most types of [Enigo Token](https://docs.rs/enigo/0.5.*/enigo/agent/enum.Token.html). For example:
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

You can also run a program as a step, with or without an arguments list.
* One useful candidate for this is `xdg-open`, to which you can pass a local file/directory or url, and it will open it in a browser or whatever.
* **IMPORTANT**: Any process spawned this way becomes a child of g11-macro-daemon, so if you restart the daemon, all child processes will be killed.
* For example:
```ron
KeyBinding(
    m: 3,
    g: 2,
    on: Press,
    script: [
        Run(Program("gnome-calculator")),
        Run(Program("xdg-open", ["https://www.spacejam.com/1996"])),
    ],
),
```


## Recording macros
Steps for recording a macro:
1. Ensure that you have the desired 'M' bank selected (and see its LED lit) before you begin.
2. Press the 'MR' key to begin the recording mode.
   * The 'MR' key will light up solid blue to let you know that it is time to choose a 'G' key.
   * You may cancel recording mode by pressing the 'MR' key again.
3. Press a 'G' key to indicate where the macro will be saved
   * The 'MR' key will begin blinking blue to let you know that it is recording
4. Perform any number of regular keyboard interactions that will be used as the script
5. Press the 'MR' key to stop recording
   * The 'MR' key LED will turn off, letting you know that you are no longer in recording mode
   * The new binding is immediately ready for use, and has also been saved to disk so that it applies to future reboots.

How these are saved on disk:
* Recorded macros get saved to a separate `key_recordings.ron` file in the same directory as your static config.
  * This is to ensure that you have total control over your `key_bindings.ron` (will never be overwritten, unlike `key_recordings.ron`)
* `key_recordings.ron` takes precedence over `key_bindings.ron`, so while you may record new macros that are used instead of your static config, you will never lose the originals.
* You should avoid editing `key_recordings.ron`, except perhaps to move its recorded macro definitions into your `key_bindings.ron` once you are happy with them.


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
* If you are trying to record a macro and the 'MR' key LED goes out as soon as you choose a 'G' key,
  then you are likely missing [one of the udev rules](INSTALLATION.md#1-device-permissions).
