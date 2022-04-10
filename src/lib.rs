use std::collections::HashMap;
use std::io::{Read, Write};
use termion::{input::TermRead, screen::AlternateScreen};
use trie_rs::{Trie, TrieBuilder};

mod abbrev;

use crate::abbrev::Abbrev;
use termion::raw::IntoRawMode;

#[derive(Debug)]
struct AmenError<'a>(&'a str);
impl std::fmt::Display for AmenError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for AmenError<'_> {}

pub fn run_amen<'a, W: Write, R: Read>(
    input: R,
    output: W,
    phrases: &[&'a str],
) -> Result<&'a str, Box<dyn std::error::Error>> {
    let screen = AlternateScreen::from(output);
    let output_terminal = screen.into_raw_mode()?;

    let abbrevs = abbrev::assign_abbrevs(phrases);
    let trie = collect_abbrevs_to_trie(&abbrevs);

    if let Some(phrase) = pick_phrase(input, output_terminal, abbrevs, trie)? {
        Ok(phrase)
    } else {
        Err(Box::new(AmenError("No item selected")))
    }
}

fn collect_abbrevs_to_trie(abbrevs: &HashMap<String, Abbrev>) -> Trie<u8> {
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
    abbrevs: HashMap<String, Abbrev>,
    trie: Type,
) -> Result<Option<&str>, std::io::Error> {
    let mut key_iter = input.keys();
    let mut input_abbrev = String::new();
    Ok(loop {
        let predicted_abbrevs = if input_abbrev.is_empty() {
            let mut abbrevs: Vec<String> = abbrevs.keys().map(|s| s.to_string()).collect();
            abbrevs.sort();
            abbrevs
        } else {
            trie.predictive_search(&input_abbrev)
                .into_iter()
                .map(|u8s| std::str::from_utf8(&u8s).unwrap().to_string())
                .collect::<Vec<_>>()
        };
        match predicted_abbrevs.len() {
            0 => {
                input_abbrev.pop();
            }
            1 => {
                let item = abbrevs.get(&*input_abbrev).unwrap();
                break Some(item.phrase);
            }
            _ => {
                write!(
                    output,
                    "{}{}",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1)
                )?;
                for abbrev in predicted_abbrevs {
                    let abbrev_data = &abbrevs.get(&*abbrev).unwrap();
                    print_abbrev(&mut output, abbrev_data, &input_abbrev)?;
                    write!(output, "\n\r")?;
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
    abbrev_data: &Abbrev,
    input_abbrev: &str,
) -> Result<(), std::io::Error> {
    for (i, c) in abbrev_data.phrase.char_indices() {
        let not_already_typed =
            input_abbrev.is_empty() || i > abbrev_data.indices[input_abbrev.len() - 1];

        if abbrev_data.indices.contains(&i) && not_already_typed {
            write!(
                screen,
                "{}{}{}{}{}",
                termion::color::Fg(termion::color::LightCyan),
                termion::style::Bold,
                c,
                termion::style::Reset,
                termion::color::Fg(termion::color::Reset),
            )?;
        } else {
            write!(screen, "{}", c)?;
        };
    }
    Ok(())
}
