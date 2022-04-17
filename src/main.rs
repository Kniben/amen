use amen::run_amen;
use nix::unistd::{dup, dup2};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use termion::cursor::DetectCursorPos;
use termion::raw::IntoRawMode;

fn main() -> Result<(), Box<dyn Error>> {
    let phrases = read_input_phrases()?;

    let stdout_fd = std::io::stdout().as_raw_fd();
    let prev_stdout = dup(stdout_fd)?;
    dup2(File::create("/dev/tty")?.as_raw_fd(), stdout_fd)?;

    let phrases = &*phrases.iter().map(AsRef::as_ref).collect::<Vec<_>>();

    let mut tty = &termion::get_tty()?;

    write!(tty, "{}", termion::cursor::Hide)?;
    write!(tty, "{}", termion::cursor::Save)?;

    let terminal = tty.into_raw_mode()?;

    let result = run_amen(tty, terminal, phrases, tty.cursor_pos()?.1);

    write!(tty, "{}", termion::cursor::Restore)?;
    write!(tty, "{}", termion::cursor::Show)?;

    dup2(prev_stdout, stdout_fd)?;

    println!("{}", result?);
    Ok(())
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
