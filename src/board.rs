use geometry::line_segments::{LineSegments,LineSegmentRefs};
use geometry::line_segment::LineSegment;
use geometry::vector::Vector;
use ball::Ball;
use paddle::Paddle;
use state::State;
use io::Input;
use score::Score;
use ui;

// TODO: add r_paddle to board struct
pub struct Board {
    pub ball: Ball,
    // todo: remove this guy as it's just confusing
    pub reflective_lines: LineSegments,
    pub l_paddle: Paddle,
}

impl Board {

    pub fn new() -> Board {
        let l_paddle = Paddle::new();
        let line_segments =  LineSegments::new_top_and_bottom_guards();

        Board {
            ball: Ball::new(),
            reflective_lines: line_segments,
            l_paddle: l_paddle,
        }
    }

    pub fn update_game_state(&mut self, input: &mut Input, state: &mut State, score: &mut Score, tick_count: i32) {

        // todo: refactor these toggles!
        if input.shape_toggle == true {
            state.shape_toggle = true;
        }

        self.l_paddle.update(input);
        let paddle_line_segment = self.l_paddle.line_segment();
        state.paddle_line = paddle_line_segment.clone();

        let mut intersection_lines = LineSegmentRefs::new();

        for line in &state.boundary_lines.0 {
            intersection_lines.0.push(line);
        }

        if state.shape_toggle == true {
            for line in &state.shape.0 {
                intersection_lines.0.push(line);
            }
        }

        intersection_lines.0.push(&paddle_line_segment);

        info!("LINES: count: {:?}, {:?}", intersection_lines.0.len(), intersection_lines);

        self.ball = self.ball.update_position_from_references(&intersection_lines, input, score);
    }

    pub fn starting_triangle() -> LineSegments {
        let vec_a = Vector { x: (*ui::MAX_X / 3) as f64, y: (*ui::MAX_Y / 3) as f64 };
        let vec_b = Vector { x: (*ui::MAX_X / 3) as f64 * 2.0, y: (*ui::MAX_Y / 3) as f64 };
        let vec_c = Vector { x: (*ui::MAX_X / 3) as f64, y: (*ui::MAX_Y / 3) as f64 * 2.0 };
        let a = LineSegment(vec_a, vec_b);
        let b = LineSegment(vec_b, vec_c);
        let c = LineSegment(vec_c, vec_a);
        LineSegments(vec!(a, b, c))
    }

    // pub fn update_shape(&mut self) -> LineSegments {
    //     let vec_a = Vector { x: (*ui::MAX_X / 3) as f64, y: (*ui::MAX_Y / 3) as f64 };
    //     let vec_b = Vector { x: (*ui::MAX_X / 3) as f64 * 2.0, y: (*ui::MAX_Y / 3) as f64 };
    //     let vec_c = Vector { x: (*ui::MAX_X / 3) as f64, y: (*ui::MAX_Y / 3) as f64 * 2.0 };
    //     let a = LineSegment(vec_a, vec_b);
    //     let b = LineSegment(vec_b, vec_c);
    //     let c = LineSegment(vec_c, vec_a);
    //     LineSegments(vec!(a, b, c))
    //     // line_references.0.push(&a);
    // }

}
