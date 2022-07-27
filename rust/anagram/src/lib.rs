use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|it| is_anagram(word, it))
        .cloned()
        .collect()
}

fn is_anagram(word: &str, candidate: &str) -> bool {
    let word_lower = word.to_lowercase();
    let candidate_lower = candidate.to_lowercase();
    sort_str(&word_lower) == sort_str(&candidate_lower) && word_lower != candidate_lower
}

fn sort_str(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    String::from_iter(&chars)
}
