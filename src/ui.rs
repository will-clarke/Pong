use ncurses::*;
use std::{thread, time};

lazy_static! {
    static ref MAX_Y: i32 = max_y();
    static ref MAX_X: i32 = max_x();
}

pub fn init_ui() -> (i32, i32) {
    initscr();
    noecho();
    timeout(0);
    raw();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    max_y_and_max_x()
}

pub fn end_ui() {
    // full_screen_message(*MAX_Y, *MAX_X, "GAME OVER LOSER!");
    endwin();
}

fn full_screen_message(max_y: i32, max_x: i32, message: &str) {
    // clear();
    // refresh();
    let message_length = message.chars().count() as i32;
    let message_border_y = 3;
    let message_border_x = 5;

    let window_x_size = message_length + message_border_x * 2;
    let window_y_size = 1 + message_border_y * 2;

    let window_starting_x = (max_x - window_x_size) / 2;
    let window_starting_y = (max_y - window_y_size) / 2;

    let welcome_window = newwin(window_y_size, window_x_size,
                                window_starting_y, window_starting_x);
    box_(welcome_window, 0, 0);
    mvprintw( window_starting_y + message_border_y,
              window_starting_x + message_border_x, message);
    wrefresh(welcome_window);
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
