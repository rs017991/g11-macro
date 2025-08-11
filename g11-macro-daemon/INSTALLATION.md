# Installation

## 0. Prerequisites

This application is built with [Rust](https://www.rust-lang.org/); while you need not download the source,
you must have certain packages installed.

The Linux distributions listed here are merely the ones I tested personally.
Others ought to work just fine, so long as you can locate the respective packages.

### Debian
Tested on Linux Mint 22.1 (Ubuntu 24.04):
```bash
sudo apt install rustup libudev-dev
rustup default stable
```

### Fedora
Tested on Fedora 42:
```bash
sudo dnf install rust cargo systemd-devel libxkbcommon-devel
```


## 1. Device Permissions
To be able to interact with an HID device, you must create a `udev` rule:
1. Create a file within `/etc/udev/rules.d` (can be named anything ending with `.rules`) with the following contents:
   ```udev
   SUBSYSTEM=="hidraw", ATTRS{idVendor}=="046d", ATTRS{idProduct}=="c225", MODE="0666", ACTION=="add", TAG+="systemd", ENV{SYSTEMD_USER_WANTS}+="g11-macro-daemon.service"
   
   #The following rule is only necessary if you want to be able to record macros with the `MR` key:
   SUBSYSTEM=="hidraw", ATTRS{idVendor}=="046d", ATTRS{idProduct}=="c221", MODE="0666"
   ```
2. In a terminal, run:
   ```bash
   sudo udevadm control --reload-rules && sudo udevadm trigger
   ```


## 2. Binary Installation

You may install the application without downloading the source code by running:
```bash
cargo install g11-macro-daemon
```
Or if you want to build it from source (perhaps with changes of your own), run from this directory:
```bash
cargo install --path .
```
In either case, it will install the binary to `~/.cargo/bin/g11-macro-daemon`


## 3. Linux Service
While you _could_ run the above binary in the foreground of a terminal (perhaps useful for troubleshooting),
it makes more sense to run this as a daemon.
1. Create a file at `~/.config/systemd/user/g11-macro-daemon.service` with the following contents (filling out `<your userid>`):
   ```systemd
   [Unit]
   Description=Logitech G11 Macro Key Daemon
   StartLimitIntervalSec=10
   
   [Service]
   ExecStart=/home/<your userid>/.cargo/bin/g11-macro-daemon
   Environment="RUST_LOG=WARN,g11=INFO"
   Restart=always
   ```
2. In a terminal, run:
   ```bash
   systemctl --user daemon-reload
   ```
3. The service will now launch whenever you plug in a G11 keyboard.
   To get it to start the first time, you _could_ unplug/replug in your keyboard, or run:
   ```bash
   systemctl --user start g11-macro-daemon
   ```
   At this point, you should see the 'M1' LED light up, but otherwise will be pretty boring without any macro bindings.
   See [CONFIGURATION.md](CONFIGURATION.md) to get started.
   


## Appendix: Troubleshooting
* If your version of Rust is too old, yet is not convenient for you to upgrade, you may try installing an older version of the daemon:
  ```bash
  cargo install g11-macro-daemon --version '0.1.*'
  ```
* You can check the status of the service by running:
  ```bash
  systemctl --user status g11-macro-daemon
  ```
* You can inspect the service logs by running:
  ```bash
  journalctl --user -u g11-macro-daemon
  ```
