use error::AmenError;
use std::collections::{BTreeSet, HashMap};
use std::io::{Read, Write};
use termion::input::TermRead;
use trie_rs::{Trie, TrieBuilder};

mod abbrev;
pub mod error;
mod layout;

use crate::abbrev::AbbrevPhrase;

pub fn run_amen<'a, W: Write, R: Read>(
    input: R,
    output: W,
    phrases: &[&'a str],
    menu_start_row: u16,
    term_size: (u16, u16),
) -> Result<&'a str, AmenError> {
    let abbrev_phrases = abbrev::assign_abbrevs(phrases);
    let trie = collect_abbrevs_to_trie(&abbrev_phrases);

    let pick_phrase = pick_phrase(
        input,
        output,
        abbrev_phrases,
        trie,
        menu_start_row,
        term_size,
    )
    .map_err(|err| AmenError::Internal(Box::new(err)));

    if let Some(phrase) = pick_phrase? {
        Ok(phrase)
    } else {
        Err(AmenError::NoneSelected)
    }
}

fn collect_abbrevs_to_trie(abbrevs: &HashMap<String, AbbrevPhrase>) -> Trie<u8> {
    let mut trie_builder = TrieBuilder::new();
    for (abbrev, _) in abbrevs.iter() {
        trie_builder.push(abbrev);
    }
    trie_builder.build()
}

type Type = trie_rs::Trie<u8>;

fn pick_phrase<W: Write, R: Read>(
    input: R,
    mut output: W,
    abbrev_phrases: HashMap<String, AbbrevPhrase>,
    trie: Type,
    menu_start_row: u16,
    term_size: (u16, u16),
) -> Result<Option<&str>, std::io::Error> {
    let mut key_iter = input.keys();
    let mut input_abbrev = String::new();

    let mut layout = layout::Layout::new();
    let phrases: BTreeSet<_> = abbrev_phrases.values().map(|ap| ap.phrase).collect();
    layout.update(&phrases, phrases.len(), (1, menu_start_row), term_size)?;

    Ok(loop {
        let predicted_abbrevs: BTreeSet<_> = if input_abbrev.is_empty() {
            abbrev_phrases.keys().map(|s| s.to_string()).collect()
        } else {
            trie.predictive_search(&input_abbrev)
                .into_iter()
                .map(|u8s| std::str::from_utf8(&u8s).unwrap().to_string())
                .collect()
        };
        match predicted_abbrevs.len() {
            0 => {
                input_abbrev.pop();
            }
            1 => {
                let item = abbrev_phrases.get(&*input_abbrev).unwrap();
                break Some(item.phrase);
            }
            _ => {
                let predicted_phrases = predicted_abbrevs
                    .iter()
                    .map(|abbrev| abbrev_phrases.get(abbrev).unwrap().phrase)
                    .collect();
                let eliminated_phrases = phrases.difference(&predicted_phrases);

                // Clear eliminated phrases
                for phrase in eliminated_phrases {
                    write!(
                        output,
                        "{}{}",
                        layout.goto_text_position(phrase),
                        " ".repeat(layout.column_width as usize)
                    )?;
                }

                for abbrev in predicted_abbrevs.iter() {
                    let abbrev_data = &abbrev_phrases.get(&*abbrev).unwrap();
                    write!(output, "{}", layout.goto_text_position(abbrev_data.phrase))?;
                    print_abbrev(&mut output, abbrev_data, &input_abbrev)?;
                }

                output.flush()?;
            }
        };

        let key = key_iter.next().expect("Failed getting next key")?;
        match key {
            termion::event::Key::Char(c) => {
                input_abbrev.push(c);
            }
            termion::event::Key::Esc => break None,
            _ => {}
        }
    })
}

fn print_abbrev<T: Write>(
    screen: &mut T,
    abbrev_data: &AbbrevPhrase,
    input_abbrev: &str,
) -> Result<(), std::io::Error> {
    for (i, c) in abbrev_data.phrase.char_indices() {
        let not_already_typed =
            input_abbrev.is_empty() || i > abbrev_data.indices[input_abbrev.len() - 1];

        if abbrev_data.indices.contains(&i) && not_already_typed {
            write!(
                screen,
                "{}{}{}{}{}",
                termion::color::Fg(termion::color::Cyan),
                termion::style::Bold,
                c,
                termion::style::Reset,
                termion::color::Fg(termion::color::Reset)
            )?;
        } else {
            write!(
                screen,
                "{}{}{}",
                termion::color::Fg(termion::color::Magenta),
                c,
                termion::color::Fg(termion::color::Reset),
            )?;
        };
    }
    Ok(())
}
