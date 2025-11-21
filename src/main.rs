use std::io::{Write, stdout};
use termion::{cursor::Goto, terminal_size};

fn main() {
    let mut stdout = stdout();
    let (width, height) = terminal_size().unwrap();
    for y in 1..=height {
        for x in 1..=width {
            write!(stdout, "{}+", Goto(x, y)).unwrap();
        }
    }
    stdout.flush().unwrap();
}
