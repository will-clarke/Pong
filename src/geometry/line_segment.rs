use geometry::vector::Vector;
use geometry::slope::Slope;
use geometry::angle::Angle;
use config;
use std::mem;

use ncurses::*;

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
        self.0.iter().flat_map (|line|
                                LineSegments::points_on_a_line(line.0, line.1)).
            collect()
    }

    // Bresenham's line algorithm
    pub fn points_on_a_line(point_a: Vector, point_b: Vector) -> Vec<Vector> {
        let mut x1 = point_a.x;
        let mut y1 = point_a.y;
        let mut x2 = point_b.x;
        let mut y2 = point_b.y;
        let steep = (y2 - y1).abs() > (x2 - x1).abs();

        if steep {
            mem::swap(&mut x1, &mut y1);
            mem::swap(&mut x2, &mut y2);
        }

        if x1 > x2
        {
            mem::swap(&mut x1, &mut x2);
            mem::swap(&mut y1, &mut y2);
        }

        let dx = x2 - x1;
        let dy = (y2 - y1).abs();

        let mut error = dx / 2.0;
        let ystep = if y1 < y2 { 1 } else { -1 };
        let mut y = y1 as i32;

        let max_x = x2 as i32;

        (x1 as i32..max_x).map( |x|
                                 {
                                     let v = if steep {
                                         Vector::new(y, x)
                                     } else {
                                         Vector::new(x, y)
                                     };

                                     error = error - dy;
                                     if error < 0.0 {
                                         y += ystep;
                                         error += dx;
                                     };
                                     v
                                 }
        ).collect::<Vec<Vector>>()
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
