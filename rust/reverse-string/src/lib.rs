#[cfg(feature = "grapheme")]
pub fn reverse(input: &str) -> String {
    use unicode_segmentation::UnicodeSegmentation;
    input.graphemes(true).rev().collect()
}

#[cfg(not(feature = "grapheme"))]
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
