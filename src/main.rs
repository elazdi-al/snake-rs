use std::io::{Write, stdout};
use termion::cursor::Goto;

fn main() {
    let mut stdout = stdout();

    let width = 20;
    let height = 10;

    for y in 1..=height {
        for x in 1..=width {
            write!(stdout, "{}+", Goto(x, y)).unwrap();
        }
    }
    stdout.flush().unwrap();
}
