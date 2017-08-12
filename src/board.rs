use geometry::line_segments::LineSegments;
use geometry::line_segment::LineSegment;
use geometry::vector::Vector;
use ball::Ball;
use paddle::Paddle;
use config::Config;
use io::Input;
use score::Score;

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
        let mut line_segments =  LineSegments::new_top_and_bottom_guards(config);
        line_segments.0.push(LineSegment(Vector::new(0,0), Vector::new(0,0)));

        Board {
            ball: Ball::new(),
            reflective_lines: line_segments,
            l_paddle: l_paddle,
            // r_paddle: Paddle::new(config),
        }
    }

    pub fn update(&mut self, input: &mut Input, score: &mut Score) {
        self.l_paddle.update(input);
        let paddle_segment = LineSegment(
            Vector { x: 0.0, y: self.l_paddle.y },
            Vector { x: 0.0, y: self.l_paddle.y + self.l_paddle.length }
        );

        // TODO: fix nasty hack to pop last element, which happens to be previous paddle...
        {
            let mut reflective_lines = &mut self.reflective_lines;
            reflective_lines.0.pop();
            reflective_lines.0.push(paddle_segment);
        }
        self.ball = self.ball.update_position(&self.reflective_lines, input, score);
    }

    pub fn check_score(&self, score: &mut Score) {

    }

}
