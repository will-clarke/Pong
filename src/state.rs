use geometry::line_segments::LineSegments;
use geometry::line_segment::LineSegment;
use paddle::Paddle;
use score::Score;

// enum Keypress {
//     Down,
//     Up,
// }

pub struct State {
    pub pause_toggle: bool,
    pub shape_toggle: bool,
    pub intersection_lines: LineSegments,
    pub paddle_line: LineSegment,
    pub score: Score,
    pub shape: Option<LineSegments>,
    // paddle_size: f64,
}

impl State {
    pub fn new() -> Self {
        State {
            pause_toggle: false,
            shape_toggle: false,
            intersection_lines: LineSegments::new_top_and_bottom_guards(),
            paddle_line: Paddle::new().line_segment(),
            score: Score::new(),
            shape: None,
            // paddle_size: f64,
        }

    }
}
