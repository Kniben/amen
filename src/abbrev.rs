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

        for (occurred_char, occurrences) in char_occurrences {
            match occurrences {
                1 => {
                    let phrase = char_phrase_pairs
                        .iter()
                        .find(|(char, _phrase)| *char == occurred_char)
                        .map(|(_char, phrase)| **phrase)
                        .unwrap();

                    let entry = abbrev_by_phrase.entry(phrase).or_insert(AbbrevPhrase {
                        phrase,
                        abbrev: String::new(),
                        indices: vec![],
                    });
                    (*entry).abbrev.push(occurred_char);
                    (*entry).indices.push(col);
                    remaining_phrases.remove(&phrase);
                }
                _ => {
                    let terminated_phrase = char_phrase_pairs
                        .iter()
                        .filter(|(char, _phrase)| *char == occurred_char)
                        .map(|(_char, phrase)| **phrase)
                        .find(|phrase| phrase.len() == col + 1);

                    if let Some(phrase) = terminated_phrase {
                        let entry = abbrev_by_phrase.entry(phrase).or_insert(AbbrevPhrase {
                            phrase,
                            abbrev: String::new(),
                            indices: vec![],
                        });
                        (*entry).abbrev.push(occurred_char);
                        (*entry).indices.push(col);
                        remaining_phrases.remove(&phrase);
                    }
                }
            }
        }
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
