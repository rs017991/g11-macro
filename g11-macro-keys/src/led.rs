//! Internal module for mapping between the bitwise USB representation of key LEDs
//! and the respective [`Key`] where each LED is located.

#![allow(clippy::default_constructed_unit_structs)]

use bitflags::bitflags;
use super::{Key, UnrecognizedKey};

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Led: u8 {
        const M1 = 1 << 0;
        const M2 = 1 << 1;
        const M3 = 1 << 2;
        const MR = 1 << 3;
    }
}
impl TryFrom<Key> for Led {
    type Error = UnrecognizedKey;

    fn try_from(value: Key) -> Result<Self, Self::Error> {
        match value {
            Key::M(1) => Ok(Self::M1),
            Key::M(2) => Ok(Self::M2),
            Key::M(3) => Ok(Self::M3),
            Key::MR   => Ok(Self::MR),

            _ => Err(Self::Error::default()),
        }
    }
}

impl From<Led> for [u8; 4] {
    fn from(value: Led) -> Self {
        [
            0x02, 0x04, // (always the same, purpose unknown)
            !value.bits() & 0x0F, //(complemented because 0 represents a lit LED)
            0x00,       // (always the same, purpose unknown)
        ]
    }
}
