// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const SINGLE_SPEED: u8 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let rate = match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=u8::MAX => 0.77,
    };
    (speed as f64 * SINGLE_SPEED as f64) * rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
