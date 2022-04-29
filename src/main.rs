use amen::error::AmenError;
use amen::{create_layout, run_amen};
use nix::unistd::{dup, dup2};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::process;
use termion::cursor::DetectCursorPos;
use termion::raw::IntoRawMode;

fn main() -> Result<(), Box<dyn Error>> {
    let phrases = read_input_phrases()?;

    let stdout_fd = std::io::stdout().as_raw_fd();
    let prev_stdout = dup(stdout_fd)?;
    dup2(File::create("/dev/tty")?.as_raw_fd(), stdout_fd)?;

    let phrases = &*phrases.iter().map(AsRef::as_ref).collect::<Vec<_>>();

    let mut tty = &termion::get_tty()?;
    let terminal = tty.into_raw_mode()?;
    let term_size = termion::terminal_size()?;
    let mut layout = create_layout(phrases, term_size)?;

    write!(tty, "{}{}", termion::cursor::Hide, termion::cursor::Save)?;
    let remaining_lines = term_size.1 - tty.cursor_pos()?.1;
    let scroll_amount = layout.size.1 - remaining_lines.min(layout.size.1);
    layout.offset((0, tty.cursor_pos()?.1 - scroll_amount));
    write!(tty, "{}", termion::scroll::Up(scroll_amount))?;

    let result = run_amen(tty, terminal, phrases, &layout);

    write!(tty, "{}", termion::scroll::Down(scroll_amount))?;
    write!(tty, "{}{}", termion::cursor::Restore, termion::cursor::Show)?;

    dup2(prev_stdout, stdout_fd)?;

    match result {
        Ok(phrase) => {
            println!("{}", phrase);
            Ok(())
        }
        Err(AmenError::NoneSelected) => process::exit(1),
        Err(AmenError::Internal(error)) => Err(error),
    }
}

fn read_input_phrases() -> Result<Vec<String>, Box<dyn Error>> {
    let mut phrases = vec![];
    let mut line_buffer = String::new();
    while let Ok(n) = std::io::stdin().read_line(&mut line_buffer) {
        if n == 0 {
            break;
        }
        phrases.push(line_buffer.trim_end().to_string());
        line_buffer.clear();
    }

    if phrases.is_empty() {
        return Ok(vec![]);
    }

    Ok(phrases)
}
