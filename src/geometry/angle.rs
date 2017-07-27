use geometry::line_segment::LineSegment;
use geometry::vector::Vector;

#[derive(PartialEq,Debug,PartialOrd)]
pub struct Angle(pub f64);

impl Angle {
    pub fn reflect(&self, line_segment: &LineSegment) -> Angle {
        let mut angle_of_line_segment = line_segment.to_angle();
        if angle_of_line_segment <= Angle(self.0 - 0.25) ||
            angle_of_line_segment >= Angle(self.0 + 0.25) {
                angle_of_line_segment = Angle((angle_of_line_segment.0 + 1.5).abs())
            };

        Angle((2.0 * angle_of_line_segment.0 - self.0).abs() % 1.0)
    }
}

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
    assert_eq!(outgoing_angle, Angle(0.24));
    // TODO: Write more tests here...
}
