use ui;

#[derive(Clone,Copy)]
pub struct Config {
    pub window_width: f64,
    pub window_height: f64,
    pub paddle_size: f64,
    pub paused: bool,
    pub reset: bool,
}

impl Config {
    pub fn new() -> Config {
        Config {
            window_width: *ui::MAX_X as f64,
            window_height: *ui::MAX_Y as f64,
            paddle_size: 8.0,
            paused: false,
            reset: false,
        }
    }
}
