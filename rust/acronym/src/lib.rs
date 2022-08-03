pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c| [' ', '-', '_'].iter().any(|it| it == &c))
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .collect::<String>()
        .to_uppercase()
}
