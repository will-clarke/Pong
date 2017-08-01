use geometry::vector::Vector;
use geometry::slope::Slope;
use geometry::angle::Angle;
use config;

pub struct LineSegment(pub Vector, pub Vector);
pub struct LineSegments(pub Vec<LineSegment>);

impl LineSegment {
    fn to_slope(&self) -> Slope {
        let x_multiplier = (self.1.y - self.0.y) /
            (self.1.x - self.0.x);
        let y_intercept = self.0.y - x_multiplier * self.0.x;
        Slope {
            x_multiplier: x_multiplier,
            y_intercept: y_intercept,
        }
    }

    pub fn to_angle(&self) -> Angle {
        self.relative_delta().to_angle()
    }

    fn relative_delta(&self) -> Vector {
        Vector {
            x: self.0.x - self.1.x,
            y: self.0.y - self.1.y,
        }
    }
}

impl LineSegments {
    pub fn new(config: &config::Config) -> LineSegments {
        let top = LineSegment(Vector { x: 0.0, y: 0.0 },
                              Vector { x: config.window_width, y: 0.0});
        let bottom = LineSegment(Vector { x: 0.0, y: config.window_height},
                                 Vector { x: config.window_width, y: config.window_height });
        LineSegments(vec!(top, bottom))
    }

    pub fn to_intermediate_vectors(&self) -> Vec<Vector> {
        let mut iterator = self.0.iter().peekable();
        let mut from = iterator.next();
        let mut to = iterator.next();
        let mut output = vec!();
        loop {
            // draw_line_between(from, to);

            // todo: implement me here!



            to = from;
            from =  iterator.next();
            if from.is_none() {
                break;
            }
        }
    }
}

#[test]
fn test_initalize() {
    let p1 = Vector::new(1, 2);
    let p2 = Vector::new(3, 4);
    let line = LineSegment(p1, p2);
    assert_eq!(line.0.x, 1.0);
    assert_eq!(line.0.y, 2.0);
    assert_eq!(line.1.x, 3.0);
    assert_eq!(line.1.y, 4.0);
}

#[test]
fn test_to_slope() {
    let p1 = Vector::new(-1, 5);
    let p2 = Vector::new(3, 7);
    let line = LineSegment(p1, p2);
    let expected_slope = Slope {
        x_multiplier: 0.5,
        y_intercept: 5.5,
    };
    assert_eq!(line.to_slope(), expected_slope);
}
