use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct AbbrevPhrase<'a> {
    pub phrase: &'a str,
    pub abbrev: String,
    pub indices: Vec<usize>,
}

pub fn assign_abbrevs<'a>(phrases: &[&'a str]) -> HashMap<String, AbbrevPhrase<'a>> {
    let mut abbrev_by_phrase = HashMap::new();

    let max_len = phrases
        .iter()
        .fold(0, |max_len, phrase| max_len.max(phrase.len()));

    let mut remaining_phrases = phrases.into_iter().collect::<HashSet<_>>();

    for col in 0..max_len {
        let char_phrase_pairs = remaining_phrases
            .clone()
            .into_iter()
            .filter_map(|phrase| phrase.chars().nth(col).map(|char| (char, phrase)))
            .filter(|(char, _phrase)| char != &' ')
            .collect::<Vec<_>>();

        let char_occurrences =
            count_char_occurrences(char_phrase_pairs.iter().map(|(char, _phrase)| *char));

        // TODO implement this,
    }

    abbrev_by_phrase
        .into_values()
        .map(|abbrev_phrase| (abbrev_phrase.abbrev.clone(), abbrev_phrase))
        .collect()
}

fn count_char_occurrences(chars: impl Iterator<Item = char>) -> HashMap<char, u32> {
    let mut occurences = HashMap::new();
    for char in chars {
        let entry = occurences.entry(char).or_insert(0);
        *entry += 1;
    }
    occurences
}
