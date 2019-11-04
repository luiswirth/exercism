use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let sorted_word = word.to_lowercase().to_owned();
    let mut sorted_word = sorted_word.graphemes(true).collect::<Vec<_>>();
    sorted_word.sort();

    possible_anagrams
        .iter()
        .cloned()
        .filter(|pa| *pa.to_lowercase() != word.to_lowercase())
        .filter(|pa| {
            let sorted_anagram = pa.to_lowercase().to_owned();
            let mut sorted_anagram = sorted_anagram.graphemes(true).collect::<Vec<_>>();
            sorted_anagram.sort();
            sorted_anagram == sorted_word
        })
        .collect()
}
