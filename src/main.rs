use nix::{libc::dup, unistd::dup2};
use std::fs::File;
use std::io::Write;
use termion::{input::TermRead, raw::IntoRawMode, screen::AlternateScreen};
use trie_rs::TrieBuilder;
mod abbrev;

#[cfg(unix)]
use std::os::unix::io::AsRawFd;

fn main() -> Result<(), std::io::Error> {
    let mut items = vec![];
    let mut line_buffer = String::new();
    while let Ok(n) = std::io::stdin().read_line(&mut line_buffer) {
        if n == 0 {
            break;
        }
        items.push(line_buffer.trim_end().to_string());
        line_buffer.clear();
    }

    if items.is_empty() {
        return Ok(());
    }

    let abbrevs = abbrev::assign_abbrevs(items);
    let mut trie_builder = TrieBuilder::new();
    for (abbrev, _) in &abbrevs {
        trie_builder.push(abbrev);
    }
    let trie = trie_builder.build();

    let stdout_fd = std::io::stdout().as_raw_fd();

    let prev_stdout = unsafe { dup(stdout_fd) };
    dup2(File::create("/dev/tty")?.as_raw_fd(), stdout_fd)?;

    if let Some(item) = pick_item(abbrevs, trie)? {
        dup2(prev_stdout, stdout_fd)?;
        println!("{}", item);
    }
    Ok(())
}

fn pick_item(
    abbrevs: std::collections::HashMap<String, abbrev::AbbrevString>,
    trie: trie_rs::Trie<u8>,
) -> Result<Option<String>, std::io::Error> {
    let tty = termion::get_tty()?;
    let screen = AlternateScreen::from(&tty);
    let mut screen = screen.into_raw_mode()?;
    let mut keys = screen.keys();
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
                let item = abbrevs.get(&input_abbrev).unwrap();
                break Some(item.original.clone());
            }
            _ => {
                write!(
                    screen,
                    "{}{}",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1)
                )?;
                for abbrev in predicted_abbrevs {
                    let abbrev_data = &abbrevs.get(&abbrev).unwrap();
                    print_abbrev(&mut screen, abbrev_data, &input_abbrev)?;
                    write!(screen, "\n\r")?;
                }
                screen.flush()?;
            }
        };

        let key = keys.next().expect("Failed getting next key")?;
        match key {
            termion::event::Key::Char(c) => {
                input_abbrev.push(c);
            }
            termion::event::Key::Esc => break None,
            _ => {}
        }
    })
}

fn print_abbrev(
    screen: &mut AlternateScreen<&File>,
    abbrev_data: &&abbrev::AbbrevString,
    input_abbrev: &String,
) -> Result<(), std::io::Error> {
    for (i, c) in abbrev_data.original.char_indices() {
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
