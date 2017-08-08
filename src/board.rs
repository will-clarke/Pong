use geometry::line_segments::LineSegments;
use ball::Ball;
use paddle::Paddle;
use config::Config;

pub struct Board {
    pub ball: Ball,
    pub reflective_lines: LineSegments,
    pub l_paddle: Paddle,
    pub r_paddle: Paddle,
}

impl Board {
    pub fn new(config: &Config) -> Board {
        Board {
            ball: Ball::new(config),
            reflective_lines: LineSegments::new_top_and_bottom_guards(config),
            l_paddle: Paddle::new(config),
            r_paddle: Paddle::new(config),
        }
    }

    pub fn update(&mut self) {
        self.ball = self.ball.update_position(&self.reflective_lines);
    }

}
