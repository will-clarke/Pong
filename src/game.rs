use config::Config;
use board::Board;
use score::Score;

pub struct Game {
    config: Config,
    board: Board,
    score: Score,
}

impl Game {
    pub fn new() -> Game {
        let config = Config::new();
        Game {
            config: config,
            board: Board::new(&config),
            score: Score::new()
        }
    }
}
