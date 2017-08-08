use config::Config;
use io::Input;
use io::Direction;

pub struct Paddle {
    pub y: f64,
    pub length: f64,
    pub max_y: f64,
}

impl Paddle {
    pub fn new(config: &Config) -> Paddle {
        Paddle {
            y: (config.window_height - config.paddle_size) / 2.0,
            length: config.paddle_size,
            max_y: config.window_height,
        }
    }

    pub fn update(&mut self, input: &Input) {
        match input.l_player {
            Some(Direction::Up) =>  {
                self.y += 0.5;
            },
            Some(Direction::Down) => {
                self.y -= 0.5;
            },
            _ => {}
        }
    }
}
