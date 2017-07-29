#[derive(Clone,Copy)]
pub struct Config {
    pub window_width: f64,
    pub window_height: f64,
    pub paddle_size: f64,
}

impl Config {
    pub fn new() -> Config {
        Config {
            window_width: 80.0,
            window_height: 40.0,
            paddle_size: 8.0,
        }
    }
}
