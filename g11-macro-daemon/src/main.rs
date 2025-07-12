mod config;
mod bindings;

use enigo::{Enigo, Settings, agent::Agent};
use hidapi::HidApi;
use log::error;
use g11_macro_keys::{usb_id, Action, Event};

fn main() {
    env_logger::init();

    let config::Config { key_bindings } = config::ensure_and_load_config_file().expect("Unable to load config");
    let mut binding_banks = bindings::BindingBanks::from(key_bindings);

    let api = HidApi::new().expect("Unable to acquire HID API");
    let mut enigo = Enigo::new(&Settings::default()).expect("Unable to acquire Enigo API");

    let hid = api.open(usb_id::VENDOR_LOGITECH, usb_id::PRODUCT_G11).expect("Unable to open device");
    let mut usb_buf = [0_u8; 9];
    let mut state = g11_macro_keys::State::default();

    //Start it off with the first bank of bindings
    let _ = state.set_exact_lit_leds(&[g11_macro_keys::Key::M(1)])
        .and_then(|usb_report| hid.send_feature_report(&usb_report).ok());

    loop {
        assert_eq!(hid.read(&mut usb_buf).expect("could not read from device"), 9);
        match state.try_consume_event(&usb_buf) {
            Ok(Event { action: Action::Pressed, key: key@g11_macro_keys::Key::M(m_key) }) => {
                binding_banks.activate_bank(m_key);
                if let Some(usb_report) = state.set_exact_lit_leds(&[key]) {
                    let _ = hid.send_feature_report(&usb_report)
                        .inspect_err(|err| error!("Unable to update LEDs! Cause: {err:#?}"));
                }
            }
            Ok(event) =>
                if let Some(script) = binding_banks.script_for(event) {
                    for step in script {
                        let _ = enigo.execute(step)
                            .inspect_err(|err| error!("Unable to execute {step:?}! Cause: {err:#?}"));
                    }
                },
            Err(err) =>
                error!("\n\nError interpreting USB output! {err:#?}; bytes were {usb_buf:?}"),
        }
    }
}
