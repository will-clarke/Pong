use geometry::point::Point;
use geometry::slope::Slope;

pub struct LineSegment(pub Point, pub Point);

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
}

#[test]
fn test_initalize() {
    let p1 = Point::new(1, 2);
    let p2 = Point::new(3, 4);
    let line = LineSegment(p1, p2);
    assert_eq!(line.0.x, 1.0);
    assert_eq!(line.0.y, 2.0);
    assert_eq!(line.1.x, 3.0);
    assert_eq!(line.1.y, 4.0);
}

#[test]
fn test_to_slope() {
    let p1 = Point::new(-1, 5);
    let p2 = Point::new(3, 7);
    let line = LineSegment(p1, p2);
    let expected_slope = Slope {
        x_multiplier: 0.5,
        y_intercept: 5.5,
    };
    assert_eq!(line.to_slope(), expected_slope);
}
