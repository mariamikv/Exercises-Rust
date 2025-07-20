use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let sorted_word = normalize(&*word.to_lowercase());
    let mut result = HashSet::new();

    for &candidate in possible_anagrams {
        if candidate.eq_ignore_ascii_case(&*word.to_lowercase()) {
            continue;
        }
        if normalize(&*candidate.to_lowercase()) == sorted_word {
            result.insert(candidate);
        }
    }

    result
}

fn normalize(s: &str) -> Vec<char> {
    let mut chars: Vec<char> = s.to_lowercase().chars().collect();
    chars.sort_unstable();
    chars
}
