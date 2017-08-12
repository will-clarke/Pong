use board::Board;
use geometry::line_segments::LineSegments;
use paddle::Paddle;
use ball::Ball;
use score::Score;

use ncurses::*;

lazy_static! {
    pub static ref MAX_Y: i32 = max_y();
    pub static ref MAX_X: i32 = max_x();
}

pub fn init_ui() {
    initscr();
    noecho();
    timeout(0);
    raw();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

pub fn end_ui() {
    // full_screen_message(*MAX_Y, *MAX_X, "GAME OVER LOSER!");
    endwin();
}

fn max_y_and_max_x() -> (i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    (max_y, max_x)
}

fn max_y() -> i32 {
    max_y_and_max_x().0
}

fn max_x() -> i32 {
    max_y_and_max_x().1
}

// fn full_screen_message(max_y: i32, max_x: i32, message: &str) {
//     // clear();
//     // refresh();
//     let message_length = message.chars().count() as i32;
//     let message_border_y = 3;
//     let message_border_x = 5;
//     let window_x_size = message_length + message_border_x * 2;
//     let window_y_size = 1 + message_border_y * 2;
//     let window_starting_x = (max_x - window_x_size) / 2;
//     let window_starting_y = (max_y - window_y_size) / 2;
//     let welcome_window = newwin(window_y_size, window_x_size,
//                                 window_starting_y, window_starting_x);
//     box_(welcome_window, 0, 0);
//     mvprintw( window_starting_y + message_border_y,
//               window_starting_x + message_border_x, message);
//     wrefresh(welcome_window);
// }

pub trait Drawable {
    fn draw(&self);
}

impl Drawable for Board {
    fn draw(&self) {
        self.reflective_lines.draw();
        // self.r_paddle.draw();
        self.l_paddle.draw();
        self.ball.draw();
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

impl Drawable for Score {
    fn draw(&self) {
            mvprintw( *MAX_Y - 4,
                      *MAX_X - 13,
                       &format!("SCORE: {}", self.l_score));
    }
}
