use config::Config;
use io::Input;
use io::Direction;
use geometry::line_segment::LineSegment;
use geometry::vector::Vector;
use ui;

static PADDLE_MOVEMENT: f64 = 1.0;

pub struct Paddle {
    pub y: f64,
    pub length: f64,
    pub max_y: f64,
}

impl Paddle {
    pub fn new() -> Paddle {
        let default_length: f64 = 8.0;
        Paddle {
            y: (*ui::MAX_Y as f64 - default_length) / 2.0,
            length: default_length,
            max_y: *ui::MAX_Y as f64,
        }
    }

    pub fn line_segment(&mut self) -> LineSegment {
        LineSegment( Vector { x: 0.0, y: self.y },
                     Vector { x: 0.0, y: self.y + self.length } )
    }

    pub fn update(&mut self, input: &Input) {
        match input.l_player {
            Some(Direction::Up) => {
                if self.y > 0.0 {
                    self.y -= PADDLE_MOVEMENT;
                }
            },
            Some(Direction::Down) => {
                if self.max_y > self.y + self.length {
                    self.y += PADDLE_MOVEMENT;
                }
            },
            _ => {}
        }
    }
}
