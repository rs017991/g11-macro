//! Internal module for mapping between the bitwise USB representation of keys and individual [`Key`] values

#![allow(clippy::default_constructed_unit_structs)]

use bitflags::bitflags;
use super::{Key, UnrecognizedKey};

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MultiKey: u64 {
        const G1  = 1 << 56;
        const G2  = 1 << 49;
        const G3  = 1 << 42;
        const G4  = 1 << 35;
        const G5  = 1 << 28;
        const G6  = 1 << 21;
        const G7  = 1 << 48;
        const G8  = 1 << 41;
        const G9  = 1 << 34;
        const G10 = 1 << 27;
        const G11 = 1 << 20;
        const G12 = 1 << 13;
        const G13 = 1 << 58;
        const G14 = 1 << 51;
        const G15 = 1 << 44;
        const G16 = 1 << 37;
        const G17 = 1 << 30;
        const G18 = 1 <<  6;

        const M1 = 1 << 16;
        const M2 = 1 <<  9;
        const M3 = 1 <<  2;
        const MR = 1 << 14;

        const Backlight = 1 << 63;
    }
}
impl TryFrom<MultiKey> for Key {
    type Error = UnrecognizedKey;

    fn try_from(value: MultiKey) -> Result<Self, Self::Error> {
        match value {
            MultiKey::G1  => Ok(Self::G( 1)),
            MultiKey::G2  => Ok(Self::G( 2)),
            MultiKey::G3  => Ok(Self::G( 3)),
            MultiKey::G4  => Ok(Self::G( 4)),
            MultiKey::G5  => Ok(Self::G( 5)),
            MultiKey::G6  => Ok(Self::G( 6)),
            MultiKey::G7  => Ok(Self::G( 7)),
            MultiKey::G8  => Ok(Self::G( 8)),
            MultiKey::G9  => Ok(Self::G( 9)),
            MultiKey::G10 => Ok(Self::G(10)),
            MultiKey::G11 => Ok(Self::G(11)),
            MultiKey::G12 => Ok(Self::G(12)),
            MultiKey::G13 => Ok(Self::G(13)),
            MultiKey::G14 => Ok(Self::G(14)),
            MultiKey::G15 => Ok(Self::G(15)),
            MultiKey::G16 => Ok(Self::G(16)),
            MultiKey::G17 => Ok(Self::G(17)),
            MultiKey::G18 => Ok(Self::G(18)),

            MultiKey::M1 => Ok(Self::M(1)),
            MultiKey::M2 => Ok(Self::M(2)),
            MultiKey::M3 => Ok(Self::M(3)),
            MultiKey::MR => Ok(Self::MR),

            MultiKey::Backlight => Ok(Self::Backlight),

            _ => Err(Self::Error::default()),
        }
    }
}
impl TryFrom<Key> for MultiKey {
    type Error = UnrecognizedKey;

    fn try_from(value: Key) -> Result<Self, Self::Error> {
        match value {
            Key::G( 1) => Ok(Self::G1),
            Key::G( 2) => Ok(Self::G2),
            Key::G( 3) => Ok(Self::G3),
            Key::G( 4) => Ok(Self::G4),
            Key::G( 5) => Ok(Self::G5),
            Key::G( 6) => Ok(Self::G6),
            Key::G( 7) => Ok(Self::G7),
            Key::G( 8) => Ok(Self::G8),
            Key::G( 9) => Ok(Self::G9),
            Key::G(10) => Ok(Self::G10),
            Key::G(11) => Ok(Self::G11),
            Key::G(12) => Ok(Self::G12),
            Key::G(13) => Ok(Self::G13),
            Key::G(14) => Ok(Self::G14),
            Key::G(15) => Ok(Self::G15),
            Key::G(16) => Ok(Self::G16),
            Key::G(17) => Ok(Self::G17),
            Key::G(18) => Ok(Self::G18),

            Key::M(1) => Ok(Self::M1),
            Key::M(2) => Ok(Self::M2),
            Key::M(3) => Ok(Self::M3),
            Key::MR   => Ok(Self::MR),

            Key::Backlight => Ok(Self::Backlight),

            _ => Err(Self::Error::default()),
        }
    }
}

impl TryFrom<&[u8]> for MultiKey {
    type Error = UnrecognizedKey;

    fn try_from(usb_bytes: &[u8]) -> Result<Self, Self::Error> {
        match usb_bytes {
            //The first byte is always the same, and the remaining eight are bit flags which represent the currently pressed buttons (up to 5 at once)
            [0x02, data0, data1, data2, data3, data4, data5, data6, data7, ..] => {
                //TODO For some reason, the `1 << 32` bit is sometimes set.
                //     Its behaviour is inconsistent, yet does not appear to be random.
                //     The purpose is unclear, but ignoring it doesn't seem to hurt anything.
                let data3 = data3 & !1;

                let flags = u64::from_be_bytes([*data0, *data1, *data2, data3, *data4, *data5, *data6, *data7]);
                MultiKey::from_bits(flags)
                    .ok_or(Self::Error::default())
            }
            _ => Err(Self::Error::default()),
        }
    }
}
