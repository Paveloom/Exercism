#[cfg(feature = "grapheme")]
use unicode_reverse::reverse_grapheme_clusters_in_place;

#[cfg(not(feature = "grapheme"))]
#[must_use]
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

#[cfg(feature = "grapheme")]
#[must_use]
pub fn reverse(input: &str) -> String {
    let mut input = input.to_owned();
    reverse_grapheme_clusters_in_place(&mut input);
    input
}
