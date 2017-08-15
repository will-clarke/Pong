use geometry::line_segments::LineSegments;
use geometry::line_segment::LineSegment;
use geometry::vector::Vector;
use ball::Ball;
use paddle::Paddle;
use config::Config;
use io::Input;
use score::Score;
use ui;

// TODO: add r_paddle to board struct
pub struct Board {
    pub ball: Ball,
    pub reflective_lines: LineSegments,
    pub l_paddle: Paddle,
    // pub r_paddle: Paddle,
}

impl Board {
    pub fn new(config: &Config) -> Board {
        let l_paddle = Paddle::new(config);
        let mut line_segments =  LineSegments::new_top_and_bottom_guards();
        line_segments.0.push(LineSegment(Vector::new(0,0), Vector::new(0,0)));

        Board {
            ball: Ball::new(),
            reflective_lines: line_segments,
            l_paddle: l_paddle,
            // r_paddle: Paddle::new(config),
        }
    }

    pub fn update(&mut self, input: &mut Input, score: &mut Score) {
        if input.shape_toggle {
            self.update_shape();
        }
        self.l_paddle.update(input);

        // TODO: Do stuff here next... to update the paddle that's in the reflective_lines.
        // self.l_paddle.update_position(&mut self.reflective_lines);

        // let paddle_segment = LineSegment(
        //     Vector { x: 0.0, y: self.l_paddle.y },
        //     Vector { x: 0.0, y: self.l_paddle.y + self.l_paddle.length }
        // );

        // TODO: fix nasty hack to pop last element, which happens to be previous paddle...
        // {
        //     let mut reflective_lines = &mut self.reflective_lines;
        //     reflective_lines.0.pop();
        //     reflective_lines.0.push(paddle_segment);
        // }
        self.ball = self.ball.update_position(&self.reflective_lines, input, score);
    }

    fn update_shape(&mut self) {
        let vec_a = Vector { x: (*ui::MAX_X / 3) as f64, y: (*ui::MAX_Y / 3) as f64 };
        let vec_b = Vector { x: (*ui::MAX_X / 3) as f64 * 2.0, y: (*ui::MAX_Y / 3) as f64 };
        let vec_c = Vector { x: (*ui::MAX_X / 3) as f64, y: (*ui::MAX_Y / 3) as f64 * 2.0 };

        let a = LineSegment(vec_a, vec_b);
        let b = LineSegment(vec_b, vec_c);
        let c = LineSegment(vec_c, vec_a);
        let triangle = LineSegments(vec!(a, b, c));
    }

}
