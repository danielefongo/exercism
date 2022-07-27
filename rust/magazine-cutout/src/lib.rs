// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_words: HashMap<&str, u16> = to_hash_map(magazine);
    let note_words: HashMap<&str, u16> = to_hash_map(note);

    note_words
        .keys()
        .all(|key| match (note_words.get(key), magazine_words.get(key)) {
            (Some(note_count), Some(magazine_count)) if note_count <= magazine_count => true,
            _ => false,
        })
}

fn to_hash_map<'a>(words: &[&'a str]) -> HashMap<&'a str, u16> {
    words.iter().fold(HashMap::new(), |mut map, word| {
        map.insert(&word, map.get(word).unwrap_or(&0) + 1);
        map
    })
}
