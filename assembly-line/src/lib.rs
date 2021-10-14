pub fn production_rate_per_hour(speed: u8) -> f64 {
    let rate: f64 = 221.0;
    match speed {
        1..=4 => speed as f64 * rate,
        5..=8 => speed as f64 * rate*0.9,
        9..=10 => speed as f64 * rate*0.77,
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let rate_per_hour = production_rate_per_hour(speed);
    return (rate_per_hour/60.0) as u32;
}
