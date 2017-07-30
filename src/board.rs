use geometry::line_segment::LineSegments;
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
            reflective_lines: LineSegments::new(config),
            l_paddle: Paddle::new(config),
            r_paddle: Paddle::new(config),
        }
    }

}
