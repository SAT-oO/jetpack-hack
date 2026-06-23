//! HF470 / KooSync UART framing for Kaotik The Jetpack.
//!
//! Product-specific constants and candidate frames — not part of the generic `ble-hack-skill`.

use ble_hack_skill::crc::{frame_with_aa, frame_with_crc};
use uuid::Uuid;

/// Advertised / control GATT service UUIDs seen on some Kaotik peripherals.
pub const KAOTIK_ADV_SERVICE: Uuid = Uuid::from_u128(0xdaf55d01_0000_1000_8000_00805f9b34fb);
pub const KAOTIK_CTRL_SERVICE: Uuid = Uuid::from_u128(0x015df5da_0000_1000_8000_00805f9b34fb);

/// Optional 3-frame session init (KooSync / Svakom UART family).
pub const HANDSHAKE: [&[u8]; 3] = [
    &[0x55, 0x04, 0x00, 0x00, 0x01, 0xFF, 0xAA],
    &[0x55, 0x04, 0x00, 0x00, 0x00, 0x00, 0xAA],
    &[0x55, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00],
];

/// Battery / status query used between sweep probes.
pub const BATTERY_QUERY: [u8; 7] = [0x55, 0x02, 0x00, 0x00, 0x00, 0x00, 0xFC];

/// Extra scan score when Kaotik GATT services appear in advertisements.
pub fn kaotik_scan_bonus(services: &[Uuid]) -> i32 {
    if services
        .iter()
        .any(|u| *u == KAOTIK_ADV_SERVICE || *u == KAOTIK_CTRL_SERVICE)
    {
        70
    } else {
        0
    }
}

/// Boost scale sweep + stretch/M-mode CRC candidates for FFE1 motor probing.
pub fn uart_burst_candidates() -> Vec<(&'static str, Vec<u8>)> {
    let mut frames = Vec::new();

    for (scale, label) in [
        (0x00u8, "boost_00"),
        (0x20, "boost_20"),
        (0x40, "boost_40"),
        (0x80, "boost_80"),
        (0xFF, "boost_FF"),
    ] {
        frames.push((
            label,
            frame_with_aa([0x55, 0x04, 0x00, 0x00, 0x00, scale]).to_vec(),
        ));
    }

    let crc_bodies: [([u8; 6], &str); 11] = [
        ([0x55, 0x08, 0x00, 0x00, 0x01, 0x01], "stretch_l1"),
        ([0x55, 0x08, 0x00, 0x01, 0x00, 0x00], "stretch_stop"),
        ([0x55, 0x08, 0x00, 0x03, 0x01, 0x01], "m1_t1"),
        ([0x55, 0x08, 0x00, 0x03, 0x01, 0x05], "m1_t5"),
        ([0x55, 0x08, 0x00, 0x03, 0x02, 0x05], "m2_t5"),
        ([0x55, 0x08, 0x00, 0x03, 0x03, 0x03], "m3_t3"),
        ([0x55, 0x08, 0x00, 0x03, 0x04, 0x05], "m4_t5"),
        ([0x55, 0x08, 0x00, 0x03, 0x05, 0x05], "m5_t5"),
        ([0x55, 0x08, 0x00, 0x03, 0x06, 0x05], "m6_t5"),
        ([0x55, 0x08, 0x00, 0x03, 0x07, 0x05], "m7_t5"),
        ([0x55, 0x08, 0x00, 0x03, 0x08, 0x05], "m8_t5"),
    ];
    for (bytes, label) in crc_bodies {
        frames.push((label, frame_with_crc(bytes).to_vec()));
    }

    frames.push((
        "legacy_vibe",
        frame_with_crc([0x55, 0x03, 0x00, 0x00, 0x01, 0x01]).to_vec(),
    ));
    frames.push((
        "battery_query",
        frame_with_crc([0x55, 0x02, 0x00, 0x00, 0x00, 0x00]).to_vec(),
    ));

    frames
}

#[cfg(test)]
mod tests {
    use ble_hack_skill::crc::frame_with_crc;

    #[test]
    fn stretch_level1_crc() {
        assert_eq!(
            frame_with_crc([0x55, 0x08, 0x00, 0x00, 0x01, 0x01]),
            [0x55, 0x08, 0x00, 0x00, 0x01, 0x01, 0xFC]
        );
    }

    #[test]
    fn m1_travel5_crc() {
        assert_eq!(
            frame_with_crc([0x55, 0x08, 0x00, 0x03, 0x01, 0x05]),
            [0x55, 0x08, 0x00, 0x03, 0x01, 0x05, 0xF4]
        );
    }

    #[test]
    fn m8_travel5_crc() {
        assert_eq!(
            frame_with_crc([0x55, 0x08, 0x00, 0x03, 0x08, 0x05]),
            [0x55, 0x08, 0x00, 0x03, 0x08, 0x05, 0xFE]
        );
    }
}
