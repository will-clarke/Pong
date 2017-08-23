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
    pub shape_toggle: bool,
}

impl Input {

    // TODO: how do we deal with multiple keys at once??
    pub fn new() -> Self {
        Input {
            l_player: None,
            quit: false,
            paused_toggle: false,
            restart_toggle: false,
            shape_toggle: false,
        }
    }

    pub fn update_input(&mut self) {
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
        if self.paused_toggle {
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
            // TODO: WHY DOESNT THE DEFAULT KEY_LEFT ETC.. WORK ON MY MAC?
            65 | 107 | KEY_UP => { self.l_player = Some(Direction::Up); }, // UP KEY , k
            66 | 106 | KEY_DOWN => { self.l_player = Some(Direction::Down); }, // DOWN KEY, j
            KEY_LEFT => { self.paused_toggle = true; ; self.l_player = Some(Direction::Left); },
            KEY_RIGHT => { self.l_player = Some(Direction::Right); },
            114 => { self.restart_toggle = true; }, // r
            113 => { self.quit = true; }, //q
            112 => { self.paused_toggle = true; }, //p
            49 => { self.shape_toggle = true; } // 1

            _ => {
                self.l_player = None;
                self.shape_toggle = false;
            }
        }

    }

}
