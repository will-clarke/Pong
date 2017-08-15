use config::Config;
use io::Input;
use io::Direction;
use geometry::line_segment::LineSegment;

static PADDLE_MOVEMENT: f64 = 1.0;

pub struct Paddle {
    pub y: f64,
    pub length: f64,
    pub max_y: f64,
    pub line_segment: LineSegment,
}

impl Paddle {
    pub fn new(config: &Config) -> Paddle {
        Paddle {
            y: (config.window_height - config.paddle_size) / 2.0,
            length: config.paddle_size,
            max_y: config.window_height,
            line_segment: LineSegment::new(config),
        }
    }

    pub fn update(&mut self, input: &Input) {
        match input.l_player {
            Some(Direction::Up) =>  {
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
