use geometry::line::Line;
use geometry::point::Point;

extern crate num;

#[derive(PartialEq,Debug)]
enum Orientation {
    Clockwise,
    Anticlockwise,
    Colinear
}

pub fn check_intersection(line_1: &Line, line_2: &Line) -> bool {
    let o1 = orientation(&line_1.0, &line_1.1, &line_2.0);
    let o2 = orientation(&line_1.0, &line_1.1, &line_2.1);
    let o3 = orientation(&line_2.0, &line_2.1, &line_1.0);
    let o4 = orientation(&line_2.0, &line_2.1, &line_1.1);

    // General Case
    if o1 != o2 && o3 != o4 {
        return true;
    }
    // Special Cases; the lines are colinear!
    if o1 == Orientation::Colinear && on_segment(&line_1.0, &line_2.0, &line_1.1) {
        return true;
    }
    if o2 == Orientation::Colinear && on_segment(&line_1.0, &line_2.1, &line_1.1) {
        return true;
    }
    if o3 == Orientation::Colinear && on_segment(&line_2.0, &line_1.0, &line_2.1) {
        return true;
    }
    if o4 == Orientation::Colinear && on_segment(&line_2.0, &line_1.1, &line_2.1) {
        return true;
    }

    false
}

fn orientation(p: &Point, q: &Point, r: &Point) -> Orientation {
    let orientation_value = (q.y - p.y) * (r.x - q.x) -
        (q.x - p.x) * (r.y - q.y);
    if orientation_value > 0.0 {
        Orientation::Clockwise
    } else if orientation_value < 0.0 {
        Orientation::Anticlockwise
    } else { // orientation_value == 0
        Orientation::Colinear
    }
}

fn on_segment(p: &Point, q: &Point, r: &Point) -> bool
{
    if q.x <= p.x.max(r.x) &&
        q.x >= p.x.min(r.x) &&
        q.y <= p.y.max(r.y) &&
        q.y >= p.y.min(r.y) {
            true
        } else {
            false
        }
}

#[test]
fn tmp_delete() {
    assert!(Orientation::Clockwise == Orientation::Clockwise);
}

#[test]
fn test_intersection() {
    let original_line = Line(Point::new(0,0), Point::new(10,0));
    let above_line = Line(Point::new(0,1), Point::new(10,1));
    let colinear_line = Line(Point::new(8,0), Point::new(13,0));
    let vertical_line = Line(Point::new(-1,-1), Point::new(3,3));
    let random_line = Line(Point::new(-1,-1), Point::new(-3,-3));

    assert!(!check_intersection(&original_line, &above_line));
    assert!(!check_intersection(&original_line, &random_line));
    assert!(check_intersection(&original_line, &colinear_line));
    assert!(check_intersection(&original_line, &vertical_line));
    assert!(check_intersection(&original_line, &vertical_line));
}
#[test]
fn orientation_test() {
    let p = Point::new(1, 1);
    let q = Point::new(5, 5);
    let r = Point::new(3, -2); // to the 'right hand' side of the other points
    let orientation_r = orientation(&p, &q, &r);
    assert_eq!(orientation_r, Orientation::Clockwise);

    let new_r = Point::new(3, 10); // to the 'left hand' side of the other points
    let orientation_new_r = orientation(&r, &q, &new_r);
    assert_eq!(orientation_new_r, Orientation::Anticlockwise);
}
