#[derive(Clone,Copy)]
pub struct Config {
    pub window_width: f64,
    pub window_height: f64,
    pub paddle_size: f64,
}

impl Config {
    pub fn new(max_x: i32, max_y: i32) -> Config {
        Config {
            window_width: max_x as f64,
            window_height: max_y as f64,
            paddle_size: 8.0,
        }
    }
}
