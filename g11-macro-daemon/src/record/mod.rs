use std::time::{Duration, Instant};
use enigo::Direction;
use hidapi::{HidApi, HidDevice, HidError, HidResult};
use log::{error, warn};
use g11_macro_keys::{usb_id, Action, Event};
use crate::{config::KeyBinding, steps::Step};

mod standard_keys;
mod mapping;

const MR_LED_BLINK_PERIOD: Duration = Duration::from_millis(500);

/// Places the application in 'record' state, where the user must:
/// 1. Press a 'G' key, for which the recorded macro will be associated
/// 2. Perform any number of regular keyboard interactions that will be used as the script
/// 3. Press the 'MR' key to stop recording
///
/// LED feedback:
/// 1. The `MR` LED remains solid until the user chooses a 'G' key
/// 2. The `MR` LED will then blink continuously during the recording of regular keys
/// 3. The `MR` LED is extinguished before returning from this method
pub fn run_event_loop(
    api: &HidApi,
    hid_macro: &HidDevice,
    state_macro: &mut g11_macro_keys::State,
    m: u8,
) -> Option<KeyBinding> {
    let _ = state_macro.light_led(g11_macro_keys::Key::MR)
        .and_then(|usb_report| hid_macro.send_feature_report(&usb_report).inspect_err(warn_led_failure).ok());

    let key_binding =
        choose_g_key(hid_macro, state_macro)
            .inspect_err(|err| error!("Aborting macro recording due to an error when choosing the G key: {err:#?}"))
            .ok().flatten()
            .and_then(|g|
                record_script(api, hid_macro, state_macro)
                    .inspect_err(|err| error!("Aborting macro recording due to an error when scripting: {err:#?}"))
                    .ok().flatten()
                    .map(|script| KeyBinding { m, g, on: Direction::Press, script })
            );

    let _ = state_macro.extinguish_led(g11_macro_keys::Key::MR)
        .and_then(|usb_report| hid_macro.send_feature_report(&usb_report).inspect_err(warn_led_failure).ok());

    key_binding
}

fn choose_g_key(hid_macro: &HidDevice, state_macro: &mut g11_macro_keys::State) -> HidResult<Option<u8>> {
    let mut usb_buf = [0_u8; 9];
    loop {
        assert_eq!(hid_macro.read(&mut usb_buf).expect("could not read from device"), 9);
        match state_macro.try_consume_event(&usb_buf) {
            Ok(Event { action: Action::Released, key: g11_macro_keys::Key::G(g_key) }) => return Ok(Some(g_key)),
            Ok(Event { action: Action::Released, key: g11_macro_keys::Key::MR }) => return Ok(None),
            _ => {} //Ignore all other keys at this time
        }
    }
}

fn record_script(api: &HidApi, hid_macro: &HidDevice, state_macro: &mut g11_macro_keys::State) -> HidResult<Option<Vec<Step>>> {
    let mut state_104key = standard_keys::State::new();
    let hid_104key = api.open(usb_id::VENDOR_LOGITECH, usb_id::PRODUCT_G11_STANDARD)
        .and_then(|device| device.set_blocking_mode(false).map(|()| device))?;
    let mut usb_buf = [0_u8; 9];

    let (mut next_blink, mut next_blink_at) = (false, Instant::now());

    let mut script = vec![];
    loop {
        if Instant::now() >= next_blink_at {
            let op =
                if next_blink { state_macro.light_led(g11_macro_keys::Key::MR) }
                else { state_macro.extinguish_led(g11_macro_keys::Key::MR) };
            let _ = op.and_then(|usb_report| hid_macro.send_feature_report(&usb_report).inspect_err(warn_led_failure).ok());

            next_blink = !next_blink;
            next_blink_at += MR_LED_BLINK_PERIOD;
        }

        if hid_104key.read(&mut usb_buf)? == 8 {
            let step = state_104key.try_consume_event(&usb_buf)
                .map_err(|err| HidError::HidApiError { message: err.to_string() })?;

            match (script.last_mut(), step) {
                (Some(Step::Key(prev_key, prev_dir@Direction::Press)), Some(Step::Key(new_key, Direction::Release)))
                  if *prev_key == new_key =>
                    *prev_dir = Direction::Click,
                (_, Some(step)) =>
                    script.push(step),
                (_, None) => {}
            }
        }

        if hid_macro.read_timeout(&mut usb_buf, 10)? == 9
            && matches!(state_macro.try_consume_event(&usb_buf), Ok(Event { action: Action::Released, key: g11_macro_keys::Key::MR })) {
            break;
        }
    }
    Ok(if script.is_empty() { None } else { Some(script) })
}

fn warn_led_failure(err: &HidError) {
    warn!("Ignoring failure to blink the MR LED while recording a macro: {err:#?}", );
}
