use geometry::line_segments::LineSegments;
use ui;

// #[derive(Clone,Copy)]
pub struct Config {
    pub window_width: f64,
    pub window_height: f64,
    pub paddle_size: f64,
    pub paused: bool,
    pub reset: bool,
    pub shape: bool,
    pub intersection_lines: LineSegments,
}

impl Config {
    pub fn new() -> Config {
        Config {
            window_width: *ui::MAX_X as f64,
            window_height: *ui::MAX_Y as f64,
            paddle_size: 8.0,
            paused: false,
            reset: false,
            shape: false,
            intersection_lines: LineSegments::new_top_and_bottom_guards(),
        }
    }
}
