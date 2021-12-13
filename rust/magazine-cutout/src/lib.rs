// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

#[must_use]
pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let map = note.iter().fold(HashMap::new(), |mut acc, key| {
        *acc.entry(key).or_insert(0) += 1;
        acc
    });
    map.iter().fold(true, |acc, (key, count)| {
        acc && (magazine.iter().filter(|item| item == key).count() >= *count)
    })
}
