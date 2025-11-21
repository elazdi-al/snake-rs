use std::{
    io::{Write, stdout},
    thread::sleep,
    time::{self, Duration, Instant},
};
use termion::{clear, color, cursor::Goto, terminal_size};
const WAIT: Duration = Duration::from_millis(10); //wait between frames

fn main() {
    let mut stdout = stdout();
    let (width, height) = terminal_size().unwrap();

    loop {
        write!(stdout, "{}", color::Bg(color::White)).unwrap();

        for y in 1..=height {
            for x in 1..=width {
                write!(stdout, "{} ", Goto(x, y)).unwrap();
            }
        }

        stdout.flush().unwrap();
        write!(stdout, "{}{}", clear::All, color::Bg(color::Reset)).unwrap();
        sleep(WAIT);
    }
}
