use geometry::line_segments::LineSegments;
use geometry::line_segment::LineSegment;
use geometry::vector::Vector;
use ball::Ball;
use paddle::Paddle;
use config::Config;
use io::Input;

pub struct Board {
    pub ball: Ball,
    pub reflective_lines: LineSegments,
    pub l_paddle: Paddle,
    // pub r_paddle: Paddle,
}

impl Board {
    pub fn new(config: &Config) -> Board {
        let l_paddle = Paddle::new(config);
        Board {
            ball: Ball::new(config),
            reflective_lines: LineSegments::new_top_and_bottom_guards(config),
            l_paddle: l_paddle,
            // r_paddle: Paddle::new(config),
        }
    }

    pub fn update(&mut self, input: &Input) {
        self.l_paddle.update(input);
        let paddle_segment = LineSegment(
            Vector { x: 0.0, y: self.l_paddle.y },
            Vector { x: 0.0, y: self.l_paddle.y + self.l_paddle.length }
        );

        // TODO: fix this horrobly inefficient clone
        {
            // let mut reflective_lines = &mut self.reflective_lines;
            // reflective_lines.0.push(paddle_segment);
        }
        self.ball = self.ball.update_position(&self.reflective_lines);
    }

}
