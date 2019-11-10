use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let sorted_word = sort_word(word.to_lowercase());
    possible_anagrams
        .iter()
        .cloned()
        .filter(|anagram| *anagram.to_lowercase() != word.to_lowercase())
        .filter(|anagram| {
            let sorted_anagram = sort_word(anagram.to_lowercase());
            sorted_anagram == sorted_word
        })
        .collect()
}

fn sort_word(word: String) -> String {
    let mut graphemes = word.graphemes(true).collect::<Vec<_>>();
    graphemes.sort();
    graphemes.iter().cloned().collect::<String>()
}
