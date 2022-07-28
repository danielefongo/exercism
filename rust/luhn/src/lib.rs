/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|it| *it != ' ')
        .map(|it| it.to_digit(10))
        .rev()
        .enumerate()
        .map(|(idx, it)| match (idx, it) {
            (_, None) => None,
            (idx, Some(val)) if idx % 2 != 0 && val > 4 => Some(val * 2 - 9),
            (idx, Some(val)) if idx % 2 != 0 => Some(val * 2),
            (_idx, Some(val)) => Some(val),
        })
        .collect::<Option<Vec<u32>>>()
        .filter(|it| it.len() > 1)
        .map_or(false, |it| it.iter().sum::<u32>() % 10 == 0)
}
