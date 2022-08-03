const BRACKETS: [char; 6] = ['(', ')', '[', ']', '{', '}'];

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let only_bracket_chars = string.chars().filter(|c| BRACKETS.iter().any(|b| b == c));

    for c in only_bracket_chars {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' if is_bracket(&mut stack, '(') => (),
            ']' if is_bracket(&mut stack, '[') => (),
            '}' if is_bracket(&mut stack, '{') => (),
            _ => return false,
        }
    }

    stack.is_empty()
}

fn is_bracket(stack: &mut Vec<char>, expected_chr: char) -> bool {
    stack.pop() == Some(expected_chr)
}
