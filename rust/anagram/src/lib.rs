use std::collections::{BTreeMap, HashSet};

fn get_map(word: &str) -> BTreeMap<char, usize> {
    word.chars().fold(BTreeMap::new(), |mut acc, ch| {
        *acc.entry(ch).or_insert(0) += 1;
        acc
    })
}

#[must_use]
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let mapped_word = get_map(&word);
    possible_anagrams
        .iter()
        .filter(|possible_anagram| {
            let possible_anagram = possible_anagram.to_lowercase();
            possible_anagram != word && get_map(&possible_anagram) == mapped_word
        })
        .copied()
        .collect()
}
