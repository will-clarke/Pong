use geometry::vector::Vector;
use geometry::slope::Slope;
use geometry::angle::Angle;
use config::Config;
use std::fmt;

#[derive(Debug,PartialEq)]
pub struct LineSegment(pub Vector, pub Vector);

impl LineSegment {
    pub fn new(config: &Config) -> Self {
        let start_y = (config.window_height - config.paddle_size) / 2.0;
        LineSegment (
            Vector {
                x: 0.0,
                y: start_y,
            },
            Vector {
                x: 0.0,
                y: start_y + config.paddle_size,
            }
        )
    }
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

    pub fn distance(&self) -> f64 {
        let difference = self.relative_delta();
        (difference.x * difference.x +
         difference.y * difference.y).sqrt()
    }

    fn relative_delta(&self) -> Vector {
        Vector {
            x: self.0.x - self.1.x,
            y: self.0.y - self.1.y,
        }
    }
}

impl fmt::Display for LineSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}) <-> ({}, {})", self.0.x, self.0.y,
               self.1.x, self.1.y)
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

#[test]
fn test_distance() {
    let p1 = Vector::new(0, 0);
    let p2 = Vector::new(5, 0);
    let line = LineSegment(p1, p2);
    assert_eq!(line.distance(), 5.0);
    let p1 = Vector::new(0, 0);
    let p2 = Vector::new(0, 5);
    let line = LineSegment(p1, p2);
    assert_eq!(line.distance(), 5.0);
}
