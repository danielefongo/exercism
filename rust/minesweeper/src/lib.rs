static OFFSETS: [(i32, i32); 9] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn is_bomb(minefield: &[&str], row: i32, col: i32) -> bool {
    if row < 0 || col < 0 {
        return false;
    }
    minefield.iter().nth(row as usize).map_or(false, |it| {
        it.as_bytes()
            .iter()
            .nth(col as usize)
            .map_or(false, |chr| chr == &b'*')
    })
}

fn count_bombs(minefield: &[&str], row: i32, col: i32) -> u8 {
    OFFSETS
        .iter()
        .map(|(r_offset, c_offset)| (r_offset + row, c_offset + col))
        .filter(|(r_offset, c_offset)| is_bomb(minefield, *r_offset, *c_offset))
        .count() as u8
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len() as i32;
    (0..height)
        .map(|row| {
            let width = minefield[row as usize].as_bytes().len() as i32;
            (0..width)
                .map(|col| {
                    if is_bomb(minefield, row, col) {
                        '*'
                    } else {
                        match count_bombs(minefield, row, col) {
                            0 => ' ',
                            n => (n + b'0') as char,
                        }
                    }
                })
                .collect()
        })
        .collect()
}
