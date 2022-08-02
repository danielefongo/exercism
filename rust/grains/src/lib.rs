pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => 2.pow(s),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (1..=64).map(square).sum::<u32>()
}
