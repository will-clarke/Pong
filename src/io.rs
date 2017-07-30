use ncurses::*;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Input {
    // TODO: Add r_player:
    // r_player: Direction,
    l_player: Direction,
    quit: bool,
}

trait Drawable {
    fn draw_area(&self, x: i32, y: i32, x_width: i32, y_width: i32, character: char);
}

impl Input {
    // TODO: how do we deal with multiple keys at once??
    pub fn update(&mut self) {
        let mut ch = getch();
        match ch
        {
            113 => { self.quit = true },
            KEY_LEFT => { self.l_player = Direction::Left; },
            KEY_RIGHT => { self.l_player = Direction::Right; },
            KEY_UP => { self.l_player = Direction::Up; },
            KEY_DOWN => { self.l_player = Direction::Down; },
            _ => { }
        }

    }
}
