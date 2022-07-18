pub fn count(lines: &[&str]) -> u32 {
    let mut points: Vec<(usize, usize)> = Vec::new();

    lines.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, chr)| {
            if chr == '+' {
                points.push((col, row));
            }
        })
    });

    cartesian(&points, &points)
        .iter()
        .filter(|(first, second)| second.0 > first.0 && second.1 > first.1)
        .filter(|(first, second)| is_rectangle(lines, &first, &second))
        .count() as u32
}

fn is_rectangle(
    lines: &[&str],
    &(min_x, min_y): &(usize, usize),
    &(max_x, max_y): &(usize, usize),
) -> bool {
    (min_x..=max_x).all(|x| is_horizontal_symbol(lines, x, min_y))
        && (min_x..=max_x).all(|x| is_horizontal_symbol(lines, x, max_y))
        && (min_y..=max_y).all(|y| is_vertical_symbol(lines, min_x, y))
        && (min_y..=max_y).all(|y| is_vertical_symbol(lines, max_x, y))
        && is_vertex_symbol(lines, min_x, max_y)
        && is_vertex_symbol(lines, max_x, min_y)
}

fn is_vertex_symbol(lines: &[&str], x: usize, y: usize) -> bool {
    is_char(lines, x, y, &['+'])
}

fn is_vertical_symbol(lines: &[&str], x: usize, y: usize) -> bool {
    is_char(lines, x, y, &['+', '|'])
}

fn is_horizontal_symbol(lines: &[&str], x: usize, y: usize) -> bool {
    is_char(lines, x, y, &['+', '-'])
}

fn is_char(lines: &[&str], x: usize, y: usize, chrs: &[char]) -> bool {
    let chr = lines[y].chars().nth(x).unwrap();
    chrs.iter().any(|&c| c == chr)
}

fn cartesian<'a, T>(first_vec: &'a Vec<T>, second_vec: &'a Vec<T>) -> Vec<(&'a T, &'a T)> {
    first_vec
        .iter()
        .flat_map(|first| second_vec.iter().map(move |second| (first, second)))
        .collect()
}
