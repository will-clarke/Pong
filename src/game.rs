use board::Board;
use state::State;
use score::Score;
use ui;
use io::Input;
use ui::Drawable;
use ncurses::*;

use std::{thread, time, process};

pub struct Game<'a> {
    board: Board,
    score: Score,
    input: Input,
    state: State<'a>,
}

impl<'a> Game<'a> {
    pub fn tick(&mut self, tick_count: i32) {
        self.input.update_input();
        self.board.update_game_state(&mut self.input, &mut self.state, &mut self.score, tick_count);

        clear();
        self.score.draw();
        self.state.draw(); // mmmmph... not great.. just to draw intersection lines
        self.board.draw();
        refresh();

        if self.input.quit == true {
            endwin();
            process::exit(0);
        }

        // todo.. calculate dt
        thread::sleep(time::Duration::from_millis(1));
    }

    pub fn new() -> Game<'a> {
        ui::init_ui();
        Game {
            board: Board::new(),
            score: Score::new(),
            input: Input::new(),
            state: State::new(),
        }
    }
}

impl<'a> Drop for Game<'a> {
    fn drop(&mut self) {
        ui::end_ui();
    }
}
