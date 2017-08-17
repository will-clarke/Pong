use config::Config;
use board::Board;
use state::State;
use score::Score;
use ui;
use io::Input;
use ui::Drawable;
use ncurses::*;

use std::{thread, time, process};

pub struct Game {
    board: Board,
    score: Score,
    input: Input,
    state: State,
}

impl Game {
    pub fn tick(&mut self, tick_count: i32) {
        self.input.update();
        self.board.update(&mut self.input, &mut self.state, &mut self.score, tick_count);

        clear();
        self.score.draw();
        self.board.draw();
        refresh();

        if self.input.quit == true {
            endwin();
            process::exit(0);
        }

        // todo.. calculate dt
        thread::sleep(time::Duration::from_millis(1));
    }

    pub fn new() -> Game {
        ui::init_ui();
        let config = Config::new();
        Game {
            board: Board::new(&config),
            score: Score::new(),
            input: Input::new(),
            state: State::new(),
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        ui::end_ui();
    }
}
