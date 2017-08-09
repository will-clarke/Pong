use config::Config;
use board::Board;
use score::Score;
use ui;
use io::Input;
use io::Drawable;
use ncurses::endwin;

use std::{thread, time, process};

pub struct Game {
    config: Config,
    board: Board,
    score: Score,
    input: Input,
}

impl Game {
    pub fn tick(&mut self) {
        self.input.update();
        self.board.draw();
        self.board.update(&mut self.input);

        if self.input.quit == true {
            endwin();
            process::exit(0);
        }

        // todo.. calculate dt
        thread::sleep(time::Duration::from_millis(10));
    }

    pub fn new() -> Game {
        ui::init_ui();
        let config = Config::new();
        Game {
            config: config,
            board: Board::new(&config),
            score: Score::new(),
            input: Input::new(),
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        ui::end_ui();
    }
}
