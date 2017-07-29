use config::Config;

pub struct Paddle {
    x: f64,
    length: f64,
}

impl Paddle {
    pub fn new(config: &Config) -> Paddle {
        Paddle {
            x: (config.window_height - config.paddle_size) / 2.0,
            length: config.paddle_size
        }
    }
}
