use board::Board;
use geometry::line_segment::LineSegments;
use ball::Ball;
use paddle::Paddle;
use ncurses::*;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Input {
    // TODO: Add r_player:
    // r_player: Direction,
    l_player: Option<Direction>,
    quit: bool,
}

pub trait Drawable {
    fn draw(&self);
}

impl Input {
    // TODO: how do we deal with multiple keys at once??
    pub fn new() -> Self {
        Input {
            l_player: None,
            quit: false,
        }
    }
    pub fn update(&mut self) {
        mvaddch(3, 3, 'x' as u32);
        let ch = getch();
        match ch
        {
            113 => { self.quit = true },
            KEY_LEFT  => { self.l_player = Some(Direction::Left); },
            KEY_RIGHT => { self.l_player = Some(Direction::Right); },
            KEY_UP    => { self.l_player = Some(Direction::Up); },
            KEY_DOWN  => { self.l_player = Some(Direction::Down); },
            _ => { }
        }

    }
}

impl Drawable for Board {
    fn draw(&self) {
        self.reflective_lines.draw();
        self.r_paddle.draw();
        self.l_paddle.draw();
        self.ball.draw();
        refresh();
    }
}

impl Drawable for LineSegments {
    fn draw(&self) {}
}

impl Drawable for Paddle {
    fn draw(&self) {}
}

impl Drawable for Ball {
    fn draw(&self) {
        mvaddch(10, 10, 'X' as u32);
    }
}
