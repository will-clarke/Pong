use config::Config;

pub struct Paddle {
    pub x: f64,
    pub length: f64,
    pub max_y: f64,
}

impl Paddle {
    pub fn new(config: &Config) -> Paddle {
        Paddle {
            x: (config.window_height - config.paddle_size) / 2.0,
            length: config.paddle_size,
            max_y: config.window_height,
        }
    }
}
