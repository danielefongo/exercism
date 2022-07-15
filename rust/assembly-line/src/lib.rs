// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let max_cars_produced = speed as f64 * 221.0;
    match speed {
        1..=4 => max_cars_produced,
        5..=8 => max_cars_produced * 0.9,
        _ => max_cars_produced * 0.77
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0 )as u32
}
