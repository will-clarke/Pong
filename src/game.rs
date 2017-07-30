use config::Config;
use board::Board;
use score::Score;
use ui;
use io::Input;
use io::Drawable;

use std::{thread, time};

pub struct Game {
    config: Config,
    board: Board,
    score: Score,
    input: Input,
}

impl Game {
    pub fn tick(&mut self) {
        self.board.draw();
        self.input.update();
        self.board.update();

        thread::sleep(time::Duration::from_millis(100));
    }

    pub fn new() -> Game {
        let (max_x, max_y) = ui::init_ui();
        let config = Config::new(max_x, max_y);
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
