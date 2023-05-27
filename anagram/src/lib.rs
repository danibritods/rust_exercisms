use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
 // unimplemented!(
 // "For the '{word}' word find anagrams among the following words: {possible_anagrams:?}"
 // );
 let word_lower = word.to_lowercase();
 let word_sorted = sort_word(&word_lower);
 let word_length = word.len();
 let mut anagrams = HashSet::<&'a str>::new();

 for candidate in possible_anagrams {
 if candidate.len() == word_length && 
 candidate.to_lowercase() != word_lower && 
 sort_word(&candidate.to_lowercase()) == word_sorted {
 anagrams.insert(candidate);
 }
 }
 anagrams

}

fn sort_word(word: &str) -> Vec<char> {
 let word_vec: Vec<char> = word.to_lowercase().chars().collect();
 word_vec.sort_unstable();
 word_vec
}
