use config::Config;
use board::Board;
use score::Score;
use ui;

pub struct Game {
    config: Config,
    board: Board,
    score: Score,
}

impl Game {
    pub fn tick(&self) {
        io::get_input();

    }

    pub fn new() -> Game {
        let config = Config::new();
        ui::init_ui();
        Game {
            config: config,
            board: Board::new(&config),
            score: Score::new()
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        ui::end_ui();
    }
}
