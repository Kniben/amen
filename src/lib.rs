use error::AmenError;
use layout::Layout;
use std::collections::{BTreeSet, HashMap};
use std::io::{Read, Write};
use termion::input::TermRead;
use termion::{color, event, style};
use trie_rs::{Trie, TrieBuilder};

mod abbrev;
pub mod error;
pub mod layout;

use crate::abbrev::AbbrevPhrase;

pub fn run_amen<'a, W: Write, R: Read>(
    input: R,
    output: W,
    phrases: &[&'a str],
    layout: &Layout,
) -> Result<&'a str, AmenError> {
    let abbrev_phrases = abbrev::assign_abbrevs(phrases);
    let trie = collect_abbrevs_to_trie(&abbrev_phrases);

    let pick_phrase = pick_phrase(input, output, abbrev_phrases, trie, layout)
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

fn pick_phrase<'a, W: Write, R: Read>(
    input: R,
    mut output: W,
    abbrev_phrases: HashMap<String, AbbrevPhrase<'a>>,
    trie: Type,
    layout: &Layout,
) -> Result<Option<&'a str>, std::io::Error> {
    let mut key_iter = input.keys();
    let mut input_abbrev = String::new();

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
                write!(output, "{}", layout.clear())?;
                break Some(item.phrase);
            }
            _ => {
                write!(output, "{}", layout.clear())?;

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
            event::Key::Char(c) => {
                input_abbrev.push(c);
            }
            event::Key::Esc => {
                if input_abbrev.is_empty() {
                    break None;
                } else {
                    input_abbrev.clear();
                }
            }
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
                color::Fg(color::Cyan),
                style::Bold,
                c,
                style::Reset,
                color::Fg(color::Reset)
            )?;
        } else {
            write!(
                screen,
                "{}{}{}",
                color::Fg(color::Magenta),
                c,
                color::Fg(color::Reset),
            )?;
        };
    }
    Ok(())
}
