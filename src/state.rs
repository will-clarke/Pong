use geometry::line_segments::{LineSegments,LineSegmentRefs};
use geometry::line_segment::LineSegment;
use paddle::Paddle;
use score::Score;

// enum Keypress {
//     Down,
//     Up,
// }

pub struct State<'a> {
    pub intersection_lines: LineSegmentRefs<'a>,
    pub pause_toggle: bool,
    pub shape_toggle: bool,
    pub paddle_line: LineSegment,
    pub boundary_lines: LineSegments,
    pub score: Score,
    pub shape: Option<LineSegments>,
    // paddle_size: f64,
}

impl<'a> State<'a> {
    pub fn new() -> Self {
        let boundary_lines = LineSegments::new_top_and_bottom_guards();
        let paddle_line = Paddle::new().line_segment();
        State {
            intersection_lines: LineSegmentRefs(vec!()),
            pause_toggle: false,
            shape_toggle: false,
            boundary_lines: boundary_lines,
            paddle_line: paddle_line,
            score: Score::new(),
            shape: None,
            // paddle_size: f64,
        }

    }
}
