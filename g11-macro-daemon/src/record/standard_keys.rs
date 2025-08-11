//! Mapping/tracking of regular (104-key) state

use std::collections::HashSet;
use enigo::Direction;
use keycode::KeyMap;
use g11_macro_keys::EventError;
use crate::steps::Step;
use super::mapping::*;

/// Keeps track of the known device state,
/// so that an individual event [`Step`] may be isolated from each set of new bytes received over USB.
///
/// You must keep this object up-to-date by feeding all of the bytes read from the G11's HID interface through [`State::try_consume_event`].
#[derive(Debug, Clone)]
pub struct State(keycode::KeyModifiers, HashSet<u8>);
impl State {
    pub fn new() -> Self { Self(keycode::KeyModifiers::empty(), Default::default()) }

    /// Updates the [`State`] by inspecting the given bytes (which should have been acquired from the G11's HID interface).
    /// This, combined with the previously known state, will allow an event [`Step`] to be inferred as the signal's meaning.
    ///
    /// Note: The G11 keyboard interface emits HID packets of 8 bytes. Anything less will produce an [`EventError`]
    /// The provided buffer may be larger than that, but only the first 8 bytes will be inspected.
    pub fn try_consume_event(&mut self, usb_bytes: &[u8]) -> Result<Option<Step>, EventError> {
        let (modifiers, keypresses) = match usb_bytes {
            [_, _, 1 ..= 3, ..] => Err(EventError::UnreconcilableState), //(KB errors)
            [modifiers, _, keypresses @ ..] => Ok((modifiers, keypresses.iter().take(6))),
            _ => Err(EventError::InvalidBytes),
        }?;

        let new_modifiers = keycode::KeyModifiers::from_bits(*modifiers).ok_or(EventError::InvalidBytes)?;
        let old_modifiers = std::mem::replace(&mut self.0, new_modifiers);
        let changed_modifier = new_modifiers.symmetric_difference(old_modifiers);
        if let Some(key) = key_modifier_to_enigo_key(changed_modifier) {
            let direction = if old_modifiers.contains(changed_modifier) { Direction::Release } else { Direction::Press };
            return Ok(Some(Step::Key(key, direction)));
        }

        let new_keypresses: HashSet<u8> = keypresses.copied()
            .take_while(|keycode| keycode != &0)
            .collect();
        let old_keypresses = &self.1;
        if let Some(changed_keypress) = new_keypresses.symmetric_difference(old_keypresses).next().copied() {
            let result =
                KeyMap::from_usb_code(0x07, u16::from(changed_keypress)).ok()
                .map(|keycode| {
                    let direction = if old_keypresses.contains(&changed_keypress) { Direction::Release } else { Direction::Press };
                    keycode_to_enigo_key(keycode)
                        .map(|key| Step::Key(key, direction))
                        .or(Some(Step::Raw(keycode.xkb, direction)))
                })
                .ok_or(EventError::UnreconcilableState);

            self.1 = new_keypresses;

            result
        }
        else { Ok(None) }
    }
}
