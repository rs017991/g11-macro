//! Runtime representation of 'G-key to script' mappings

use enigo::Direction;
use log::warn;
use smallvec::SmallVec;
use g11_macro_keys::{Action, Event, Key};
use crate::config::KeyBinding;

pub struct BindingBanks {
    /// Zero-indexed (respective M key minus one)
    press_banks: [BindingBank; 3],
    /// Zero-indexed (respective M key minus one)
    release_banks: Vec<BindingBank>, //Release is less common; trade a heap lookup for smaller stack
    /// Zero-indexed (respective M key minus one)
    active_bank: u8,
}
impl From<Vec<KeyBinding>> for BindingBanks {
    fn from(bindings: Vec<KeyBinding>) -> Self {
        let mut banks = Self { press_banks: Default::default(), release_banks: Default::default(), active_bank: 0 };
        for binding in bindings {
            banks.replace(binding);
        }
        banks
    }
}
impl BindingBanks {
    /// Ignores invalid M Keys
    pub fn activate_bank(&mut self, m_key: u8) {
        if let Some(bank_index) = Self::bank_index(m_key) {
            self.active_bank = bank_index as u8;
        }
    }

    pub fn script_for(&self, g_key_event: Event) -> Option<impl IntoIterator<Item = &enigo::agent::Token>> {
        match g_key_event {
            Event { key: Key::G(g_key), action: Action::Pressed } =>
                self.press_banks[self.active_bank as usize].script_for(g_key),
            Event { key: Key::G(g_key), action: Action::Released } =>
                self.release_banks.get(self.active_bank as usize)?.script_for(g_key),
            _ => None,
        }

    }

    fn bank_index(m_key: u8) -> Option<usize> {
        match m_key {
            0 | 4.. => None,
            _ => Some(m_key as usize - 1),
        }
    }
    fn replace(&mut self, binding: KeyBinding) {
        match (Self::bank_index(binding.m), binding.on) {
            (None, _) => warn!("Ignoring invalid KeyBinding (there is no M{} key)", binding.m),
            (Some(bank_index), Direction::Press) => self.press_banks[bank_index].replace(binding),
            (Some(bank_index), _) => self.ensure_release_bank(bank_index).replace(binding),
        }
    }
    fn ensure_release_bank(&mut self, bank_index: usize) -> &mut BindingBank {
        if bank_index >= self.release_banks.len() {
            self.release_banks.resize_with(bank_index + 1, Default::default);
        }
        &mut self.release_banks[bank_index]
    }
}

/// Optimised for the typical binding: a one-modifier click
pub type Script = SmallVec<[enigo::agent::Token; 3]>;

/// All G-key mappings under a specific M-key.
/// Zero-indexed (respective G key minus one)
#[derive(Default)]
struct BindingBank(Vec<Script>);
impl BindingBank {
    fn script_index(m_key: u8) -> Option<usize> {
        match m_key {
            0 | 19.. => None,
            _ => Some(m_key as usize - 1),
        }
    }

    fn script_for(&self, g_key: u8) -> Option<&Script> {
        Self::script_index(g_key)
            .and_then(|index| self.0.get(index))
    }

    fn replace(&mut self, binding: KeyBinding) {
        if let Some(script_index) = Self::script_index(binding.g) {
            if script_index >= self.0.len() {
                self.0.resize_with(script_index + 1, Default::default);
            }
            self.0[script_index] = binding.script.into_iter().collect();
        } else {
            warn!("Ignoring invalid KeyBinding (there is no G{} key)", binding.g);
        }
    }
}
