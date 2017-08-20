use geometry::line_segments::{LineSegments,LineSegmentRefs};
use geometry::line_segment::LineSegment;
use paddle::Paddle;
use board::Board;
use score::Score;
use self::PotentialIntersectionLines::{BoundaryLines,LPaddleLine,ShapeLines};

pub enum PotentialIntersectionLines {
    BoundaryLines,
    LPaddleLine,
    ShapeLines,
}

pub struct IntersectionLineTypes(pub Vec<PotentialIntersectionLines>);

impl IntersectionLineTypes {
    pub fn without_shape() -> Self {
        IntersectionLineTypes(vec!(BoundaryLines,
                               LPaddleLine,
        ))
    }
    pub fn with_shape() -> Self {
        IntersectionLineTypes(vec!(BoundaryLines,
                               LPaddleLine,
                               ShapeLines,
        ))
    }
}

pub struct State {
    pub intersection_line_types: IntersectionLineTypes,
    pub pause_toggle: bool,
    pub shape_toggle: bool,
    pub paddle_line: LineSegment,
    pub boundary_lines: LineSegments,
    pub score: Score,
    pub shape: LineSegments,
    // paddle_size: f64,
}

impl State {
    pub fn new() -> Self {
        State {
            intersection_line_types: IntersectionLineTypes::without_shape(),
            pause_toggle: false,
            shape_toggle: false,
            boundary_lines:  LineSegments::new_top_and_bottom_guards(),
            paddle_line:  Paddle::new().line_segment(),
            score: Score::new(),
            shape: Board::starting_triangle(),
            // paddle_size: f64,
        }

    }

    pub fn board_line_segments(&self) -> LineSegments {
        // todo fix clones here... ¯\_(ツ)_/¯
        let mut line_segments = LineSegments(vec!());
        for line_type in &self.intersection_line_types.0 {
            match line_type {
                &BoundaryLines => {
                    for line in self.boundary_lines.0.clone() {
                        line_segments.0.push(line);
                    }
                },
                &ShapeLines => {
                    for line in self.shape.0.clone() {
                        line_segments.0.push(line);
                    }
                },
                &LPaddleLine => {
                    line_segments.0.push(self.paddle_line.clone())},
            }
        };
        line_segments
    }
}
