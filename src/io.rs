use ncurses::*;

use std::process;

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
    pub paused_toggle: bool,
    pub restart_toggle: bool,
}

impl Input {

    // TODO: how do we deal with multiple keys at once??
    pub fn new() -> Self {
        Input {
            l_player: None,
            quit: false,
            paused_toggle: false,
            restart_toggle: false,
        }
    }

    pub fn update(&mut self) {
        let ch = getch();
        if self.paused_toggle == true {
            loop {
                match getch() {
                    112 => { self.paused_toggle = false; break; },
                    113 => { endwin(); process::exit(0); },
                    _   => {}
                }
            }
        }
        match ch
        {
            // 97 => { addch('$' as u32); }, // a

            106 => { self.l_player = Some(Direction::Down) }, // j
            107 => { self.l_player = Some(Direction::Up) }, // k

            114 => { self.restart_toggle = true }, // r

            113 => { self.quit = true },
            112 => { //p for pause
                self.paused_toggle = true;
            },
            KEY_LEFT => { self.paused_toggle = true; printw("OMG"); ; self.l_player = Some(Direction::Left); },
            KEY_RIGHT => { self.l_player = Some(Direction::Right); },
            KEY_UP => { self.l_player = Some(Direction::Up); },
            KEY_DOWN => { self.l_player = Some(Direction::Down); },
            _ => {
                self.l_player = None;
            }
        }

    }
}
