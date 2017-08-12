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
        // // loop to find out what key characters are
        // loop {
        //     let a = getch();
        //     if a == 113 { // q
        //         break;
        //     }
        //     clear();
        //     mvprintw(5, 5, &format!("Char: {}", a as u32));
        //     refresh();
        // }
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
            65 => { self.l_player = Some(Direction::Up); }, // UP KEY
            66 => { self.l_player = Some(Direction::Down); }, // DOWN KEY
            // TODO: WHY DOESNT THE DEFAULT WORK ON MY MAC?
            KEY_DOWN => { self.l_player = Some(Direction::Down); },
            KEY_LEFT => { self.paused_toggle = true; printw("OMG"); ; self.l_player = Some(Direction::Left); },
            KEY_RIGHT => { self.l_player = Some(Direction::Right); },
            KEY_UP => { self.l_player = Some(Direction::Up); },

            106 => { self.l_player = Some(Direction::Down); }, // j
            107 => { self.l_player = Some(Direction::Up); }, // k

            114 => { self.restart_toggle = true; }, // r
            113 => { self.quit = true; }, //q
            112 => { self.paused_toggle = true; }, //p

            _ => { self.l_player = None; }
        }

    }
}
