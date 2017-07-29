use geometry::line_segment::LineSegments;
use ball::Ball;
use paddle::Paddle;
use config::Config;

pub struct Board {
    ball: Ball,
    reflective_lines: LineSegments,
    l_paddle: Paddle,
    r_paddle: Paddle,
}


impl Board {
    pub fn new(config: &Config) -> Board {
        Board {
            ball: Ball::new(config),
            reflective_lines: LineSegments::new(config),
            l_paddle: Paddle::new(config),
            r_paddle: Paddle::new(config),
        }
    }
}
