use std::{
    thread,
    time::{Duration, Instant},
};
use hidapi::HidApi;
use g11_macro_keys::{usb_id, Event, Key, Action::Pressed};

/// Just for fun, this example repurposes the `M` Key LEDs as a binary stopwatch.
/// * It loops back around when it hits 16 seconds
/// * You can press any of `M1`/`M2`/`M3` to pause/unpause the stopwatch
/// * You can press `MR` to reset the stopwatch
///
/// **NOTE: If your macro keys are actually working (like if you are running the daemon),
/// then you probably shouldn't run this. It'll just be confusing as they fight over the LED state.**
///
/// Besides being pointless fun, this example demonstrates how to pair the `g11-macro-keys` library with a HID interface,
/// then allowing you to read key events and set the LED state.
///
/// Prerequisite: To be able to use the G11's HID interface, you must configure `udev` to allow access:
/// 1. Create a file within `/etc/udev/rules.d` (can be named anything ending with `.rules`) with the following contents:
///    > `SUBSYSTEM=="hidraw", ATTRS{idVendor}=="046d", ATTRS{idProduct}=="c225", MODE="0666"`
/// 2. In a terminal, run `sudo udevadm control --reload-rules && sudo udevadm trigger`
fn main() {
    let api = HidApi::new().expect("can acquire HID API");
    let hid = api.open(usb_id::VENDOR_LOGITECH, usb_id::PRODUCT_G11).expect("can open device");
    let mut key_state = g11_macro_keys::State::default();
    let mut usb_buf = [0_u8; 9];

    //Since we're doing all of this in one thread for simplicity, reading the keyboard must not block our thread when unpaused:
    hid.set_blocking_mode(false).expect("can set blocking mode");

    let mut start = Instant::now();
    let mut paused_at: Option<Duration> = None;
    loop {
        let seconds = paused_at.unwrap_or_else(|| start.elapsed()).as_secs() as u8;
        if let Some(usb_report) = key_state.set_exact_lit_leds(&encode_as_keys_to_light(seconds)) {
            hid.send_feature_report(&usb_report).expect("can write to device");
        }

        if paused_at.is_none() {
            thread::sleep(Duration::from_millis(50));
        }

        let bytes_read = hid.read(&mut usb_buf).expect("can read from device");
        match key_state.try_consume_event(&usb_buf[..bytes_read]) {
            Ok(Event { action: Pressed, key: Key::MR }) =>
                match &mut paused_at {
                    Some(accumulated) => *accumulated = Default::default(),
                    None => start = Instant::now(),
                },
            Ok(Event { action: Pressed, key: Key::M(_) }) => {
                if let Some(accumulated) = paused_at.take() {
                    start = Instant::now() - accumulated;
                } else {
                    paused_at = Some(start.elapsed());
                }
                //If it's paused, then just wait for the next keystroke:
                hid.set_blocking_mode(paused_at.is_some()).expect("can reset blocking mode");
            }
            _ => {}
        }
    }

    fn encode_as_keys_to_light(number: u8) -> Vec<Key> {
        let mut keys = vec![];
        if number      & 1 == 1 { keys.push(Key::MR) };
        if number >> 1 & 1 == 1 { keys.push(Key::M(3)) };
        if number >> 2 & 1 == 1 { keys.push(Key::M(2)) };
        if number >> 3 & 1 == 1 { keys.push(Key::M(1)) };
        keys
    }
}
