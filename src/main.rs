use std::{
    io::{Write, stdout},
    thread::sleep,
    time::Duration,
};
use termion::{
    async_stdin, clear, color, cursor::Goto, input::TermRead, raw::IntoRawMode, terminal_size,
};

const WAIT: Duration = Duration::from_millis(10);
const BACKGROUND_COLOR: color::Bg<color::White> = color::Bg(color::White);
const RESET_COLOR: color::Bg<color::Reset> = color::Bg(color::Reset);
const PADDING: u16 = 10;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().keys();
    let (width, height) = terminal_size().unwrap();

    write!(stdout, "{}{}", clear::All, termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    loop {
        // Check for 'q' key press to exit
        if let Some(Ok(termion::event::Key::Char('q'))) = stdin.next() {
            break;
        }

        // Build frame in buffer
        let mut buffer = format!("{}{}", Goto(1, 1), BACKGROUND_COLOR);

        for y in 1..=height - PADDING {
            buffer.push_str(&format!("{}", Goto(1, y)));
            buffer.extend(std::iter::repeat(' ').take((width - PADDING) as usize));
        }

        buffer.push_str(&format!("{}", RESET_COLOR));

        write!(stdout, "{}", buffer).unwrap();
        stdout.flush().unwrap();
        sleep(WAIT);
    }

    // Cleanup
    write!(
        stdout,
        "{}{}{}",
        clear::All,
        termion::cursor::Show,
        RESET_COLOR
    )
    .unwrap();
    stdout.flush().unwrap();
}
