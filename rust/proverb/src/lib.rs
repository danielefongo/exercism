pub fn build_proverb(list: &[&str]) -> String {
    list.iter()
        .enumerate()
        .map(|(idx, subject)| {
            if let Some(next_subject) = list.iter().nth(idx + 1) {
                format!("For want of a {} the {} was lost.", subject, next_subject)
            } else {
                format!("And all for the want of a {}.", list.first().unwrap())
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}
