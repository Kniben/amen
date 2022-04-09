use amen::run_amen;
use nix::unistd::{dup, dup2};
use std::fs::File;
use std::os::unix::io::AsRawFd;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

fn main() -> Result<(), std::io::Error> {
    let phrases = read_input_phrases()?;

    let stdout_fd = std::io::stdout().as_raw_fd();
    let prev_stdout = dup(stdout_fd)?;
    dup2(File::create("/dev/tty")?.as_raw_fd(), stdout_fd)?;
    let tty = &termion::get_tty()?;
    let screen = AlternateScreen::from(tty);
    let terminal = screen.into_raw_mode()?;

    let result = run_amen(terminal, phrases)?;

    dup2(prev_stdout, stdout_fd)?;

    println!("{}", result);
    Ok(())
}

fn read_input_phrases() -> Result<Vec<String>, std::io::Error> {
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
