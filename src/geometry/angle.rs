use geometry::line_segment::LineSegment;
use geometry::vector::Vector;
use std::fmt;
use std::ops;
use rand::Rng;
use rand;
use std;

// TODO: dedup definition of tau from vector:
static TAU: f64 = 2.0 * std::f64::consts::PI;

#[derive(Clone,Copy,PartialEq,Debug,PartialOrd)]
pub struct Angle(pub f64);

impl Angle {
    pub fn reflect(&self, line_segment: &LineSegment) -> Angle {
        let mut angle_of_line_segment = line_segment.angle();

        if angle_of_line_segment <= Angle(self.0 - 0.25) ||
            angle_of_line_segment >= Angle(self.0 + 0.25) {
                angle_of_line_segment = Angle((angle_of_line_segment.0 + 1.5).abs())
            };

        Angle((2.0 * angle_of_line_segment.0 - self.0).abs() % 1.0)
    }

    pub fn to_vector(&self) -> Vector {
        Vector {
            x: (self.0 * TAU).sin(),
            y: (self.0 * TAU).cos(),
        }
    }

    pub fn random_start() -> Self {
        let mut rng = rand::thread_rng();
        let random_directions = [
            rng.gen_range(0.1, 0.4),
            rng.gen_range(0.6, 0.9),
        ];
        let random_direction = rng.choose(&random_directions).
            unwrap();
        Angle(*random_direction)
    }
}

impl ops::Add<f64> for Angle {
    type Output = Angle;

    fn add(self, other: f64) -> Angle {
        Angle((self.0 + other) % 1.0)
    }
}

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Angle< {} >", self.0)
    }
}


#[cfg(test)]
mod test {
    use geometry::line_segment::LineSegment;
    use geometry::vector::Vector;
    use geometry::angle::Angle;

    #[test]
    fn test_reflect() {
        let segment = LineSegment(Vector::new(0,0), Vector::new(0, 10));
        let incoming_right_angle = Angle(0.5);
        let outgoing_angle = incoming_right_angle.reflect(&segment);
        assert_eq!(outgoing_angle, Angle(0.5));

        let incoming_sharp_angle = Angle(0.74);
        let outgoing_angle = incoming_sharp_angle.reflect(&segment);
        assert_eq!(outgoing_angle, Angle(0.26));

        let incoming_sharp_angle = Angle(0.76);
        let outgoing_angle = incoming_sharp_angle.reflect(&segment);
        assert_kind_of_equal(outgoing_angle, Angle(0.24));

        let segment = LineSegment(Vector::new(0,0), Vector::new(10, 10));
        let incoming_angle = Angle(0.0);
        let outgoing_angle = incoming_angle.reflect(&segment);
        assert_eq!(outgoing_angle, Angle(0.25));

        let incoming_angle = Angle(0.5);
        let outgoing_angle = incoming_angle.reflect(&segment);
        assert_eq!(outgoing_angle, Angle(0.75));

    }

    // TODO: create test module here for this
    fn assert_kind_of_equal(a: Angle, b: Angle) {
        assert!((a.0 - b.0).abs() < 0.001);
    }


    // TODO: also dedup... maybe make this generic???
    fn assert_vector_eq(a: Vector, b: Vector) {
        assert!((a.x - b.x).abs() < 0.001);
        assert!((a.y - b.y).abs() < 0.001);
    }

    #[test]
    fn test_to_vector() {
        assert_vector_eq(Angle(0.0).to_vector(), Vector { x: 0.0, y: 1.0 });
        assert_vector_eq(Angle(0.25).to_vector(), Vector { x: 1.0, y: 0.0 });
        assert_vector_eq(Angle(0.5).to_vector(), Vector { x: 0.0, y: -1.0 });
    }
}
