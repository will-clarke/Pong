use board::Board;
use geometry::line_segment::{LineSegment,LineSegments};
use ball::Ball;
use paddle::Paddle;
use ncurses::*;



use geometry::vector::Vector;

// use io::Drawable;

use std::{thread, time, process};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Input {
    // TODO: Add r_player:
    // r_player: Direction,
    pub l_player: Option<Direction>,
    pub quit: bool,
    pub paused: bool,
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
            paused: false,
        }
    }
    pub fn update(&mut self) {
        let ch = getch();
        if self.paused == true {
            loop {
                match getch() {
                    112 => { self.paused = false; break; },
                    113 => { endwin(); process::exit(0); },
                    _   => {}
                }
            }
        }
        match ch
        {
            // 97 => { addch('$' as u32); }, // a
            113 => { self.quit = true },
            112 => { //p for pause
                self.paused = true;
            },
            KEY_LEFT => { self.paused = true; printw("OMG"); ; self.l_player = Some(Direction::Left); },
            KEY_RIGHT => { self.l_player = Some(Direction::Right); },
            KEY_UP => { self.l_player = Some(Direction::Up); },
            KEY_DOWN => { self.l_player = Some(Direction::Down); },
            _ => {
                addch(ch as u32);
            }
        }

    }
}

impl Drawable for Board {
    fn draw(&self) {
        clear();

        let one = LineSegment(Vector::new(5, 5), Vector::new(5, 30));
        let two = LineSegment(Vector::new(5, 30), Vector::new(40, 30));
        let three = LineSegment(Vector::new(40, 30), Vector::new(5, 5));
        let segments = LineSegments(vec!(one, two, three));
        segments.draw();
        /// todo tmp:: remove this...

        self.reflective_lines.draw();
        self.r_paddle.draw();
        self.l_paddle.draw();
        self.ball.draw();
        refresh();
    }
}

impl Drawable for LineSegments {
    fn draw(&self) {
        for vec in self.to_intermediate_vectors() {
            mvaddch(vec.y as i32,
                    vec.x as i32,
                    'X' as u32);
        }
    }

}

impl Drawable for Paddle {
    fn draw(&self) {
        for i in 0..self.length as i32 {
            mvaddch((self.y as i32) + i,
                    0,
                    'I' as u32);
            // '█' as u32);
        }
    }
}

impl Drawable for Ball {
    fn draw(&self) {
        mvaddch(self.current_position.y as i32,
                self.current_position.x as i32,
                'o' as u32);
    }
}
