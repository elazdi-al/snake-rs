use std::{
    fmt::Write as FmtWrite,
    io::{self, Write, stdout},
    thread::sleep,
    time::Duration,
};

use termion::{
    async_stdin, clear,
    color::{self, Bg, Red, Reset, White},
    cursor::{Goto, Hide, Show},
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    terminal_size,
};

const WAIT: Duration = Duration::from_millis(33); // ~30 FPS
const PADDING: u16 = 3;

const BACKGROUND_COLOR: Bg<White> = Bg(White);
const RESET_COLOR: Bg<Reset> = Bg(Reset);
const SNAKE_COLOR: Bg<Red> = Bg(Red);
const SNAKE_HEAD: &str = "●";
const SNAKE_BODY: &str = "●";
const SNAKE_TAIL: &str = "●";

struct Position {
    x: u16,
    y: u16,
}
impl Position {
    fn new(x: u16, y: u16) -> io::Result<Self> {
        Ok(Self { x, y })
    }
}
struct Snake {
    positions: Vec<Position>,
}
impl Snake {
    fn new() -> io::Result<Self> {
        let positions = vec![
            Position::new(5, 5).unwrap(),
            Position::new(5, 6).unwrap(),
            Position::new(5, 7).unwrap(),
        ];
        Ok(Self { positions })
    }
}

struct GameView {
    buffer: String,
    width: u16,
    height: u16,
}

impl GameView {
    fn new() -> io::Result<Self> {
        let (width, height) = terminal_size()?;
        let capacity = (width as usize * height as usize) * 2;
        Ok(Self {
            buffer: String::with_capacity(capacity),
            width,
            height,
        })
    }

    fn goto(&mut self, x: u16, y: u16) -> &mut Self {
        let _ = write!(&mut self.buffer, "{}", Goto(x, y));
        self
    }

    fn set_bg_color(&mut self, color: Bg<impl color::Color>) -> &mut Self {
        let _ = write!(&mut self.buffer, "{}", color);
        self
    }

    fn reset_color(&mut self) -> &mut Self {
        let _ = write!(&mut self.buffer, "{}", RESET_COLOR);
        self
    }

    fn draw_box(&mut self, x: u16, y: u16) -> &mut Self {
        self.goto(x, y);
        self.buffer.push(' ');
        self
    }
    fn draw_rect(&mut self, padding: u16) -> &mut Self {
        if self.width <= padding * 2 || self.height <= padding * 2 {
            return self;
        }

        let left = padding + 1;
        let top = padding + 1;
        let right = self.width - padding;
        let bottom = self.height - padding;

        for y in top..=bottom {
            self.goto(left, y);
            for _ in left..=right {
                self.buffer.push(' ');
            }
        }

        self
    }

    fn set_desc(&mut self, description: &str) -> &mut Self {
        self.goto(PADDING, self.height);
        write!(&mut self.buffer, "{}", description).unwrap();
        self.goto(1, 1);
        self
    }
    fn render_snake(&mut self, snake: &Snake) -> &mut Self {
        let positions_size = snake.positions.len();
        for i in 0..positions_size {
            let position: &Position = snake.positions.get(i).unwrap();
            let render = match i {
                0 => SNAKE_HEAD,
                positions_size => SNAKE_TAIL, // must be changed to positions_size - 1
                _ => SNAKE_BODY,
            };
            self.goto(position.x, position.y);
            write!(&mut self.buffer, "{}{}", BACKGROUND_COLOR, render).unwrap();
        }
        self.goto(1, 1);
        self
    }

    fn render<W: Write>(&mut self, stdout: &mut W) -> io::Result<()> {
        write!(stdout, "{}", self.buffer)?;
        stdout.flush()?;
        self.buffer.clear();
        Ok(())
    }
}

fn run() -> io::Result<()> {
    let mut stdout: RawTerminal<_> = stdout().into_raw_mode()?;
    write!(stdout, "{}{}", clear::All, Hide)?;
    stdout.flush()?;

    let mut view = GameView::new()?;
    let mut stdin = async_stdin().keys();
    let mut snake: Snake = Snake::new()?;

    loop {
        if let Some(Ok(key)) = stdin.next() {
            if let Key::Char('q') = key {
                break;
            }
            if let Key::Up = key {}
            if let Key::Down = key {}
            if let Key::Right = key {}
            if let Key::Left = key {}
        }

        view.goto(1, 1)
            .set_bg_color(BACKGROUND_COLOR)
            .draw_rect(PADDING)
            .reset_color()
            .set_desc("press q to quit")
            .set_bg_color(SNAKE_COLOR)
            .draw_box(5, 5)
            .render_snake(&snake);

        view.render(&mut stdout)?;
        sleep(WAIT);
    }

    write!(stdout, "{}{}{}", clear::All, Show, RESET_COLOR)?;
    stdout.flush()?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
    }
}
