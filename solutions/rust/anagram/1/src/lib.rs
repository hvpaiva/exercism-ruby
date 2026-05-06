use std::collections::HashSet;

use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();

    let word_lower = word.to_lowercase();
    let word_canonical = canonical(&word_lower);

    for &candidate in possible_anagrams {
        if candidate.len() != word.len() {
            continue;
        }

        let candidate_lower = candidate.to_lowercase();
        if word_lower == candidate_lower {
            continue;
        }

        let candidate_canonical = canonical(&candidate_lower);

        if candidate_canonical == word_canonical {
            anagrams.insert(candidate);
        }
    }

    anagrams
}

fn canonical(s: &str) -> Vec<&str> {
    let mut grapheme = s.graphemes(true).collect::<Vec<_>>();

    grapheme.sort_unstable();

    grapheme
}
