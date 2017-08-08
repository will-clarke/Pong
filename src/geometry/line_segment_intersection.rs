use geometry::line_segment::LineSegment;
use geometry::vector::Vector;

extern crate num;

#[derive(PartialEq,Debug)]
enum Orientation {
    Clockwise,
    Anticlockwise,
    Colinear
}



//////////////////////////////////////////////////////////////
//////// MAYBE A BETTER ALGORITHM?? BENCHMARK IT!!!
//////////////////////////////////////////////////////////////

// TODO: make this an implementation / method of a LineSegment? Rather than a function..
pub fn intersects(line_1: &LineSegment, line_2: &LineSegment) -> Option<Vector> {
    let p = line_1.0;
    let q = line_2.0;
    // let r = line_1.0 - line_1.1;
    // let s = line_2.0 - line_2.1;
    let r = line_1.1 - line_1.0;
    let s = line_2.1 - line_2.0;

    // t & u are the scalar values to multiply by r & s
    let t = (q - p) * s / (r * s);
    let u = (q - p) * r / (r * s);

    // println!("r * s == {:?}",r * s );
    // println!("t == {}", t);
    // println!("u == {}", u);
    if r * s == 0.0 && (p - q) * r == 0.0 {
        // colinear => Can be more complicated...
        // but let's just say it doesn't intersect
        return None;
    } else if r * s == 0.0 && (q - p) * r != 0.0 {
        // paralell (and therefore non-intersecting)
        return None;
    } else if r * s != 0.0 && t >= 0.0 && t <= 1.0 &&
        u >= 0.0 && u <= 1.0 {
            // segment intersects at point
            // p + tr == q + us
            return Some(p + Vector::new(r.x * t, r.y * t))
        }

    None
}



////////////////////////////////////////////////////////////////

pub fn check_intersection(line_1: &LineSegment, line_2: &LineSegment) -> bool {
    // Not needed.. but potentially faster than not having it.. maybe.. ¯\_(ツ)_/¯
    if !check_bounding_boxes_intersect(line_1, line_2) {
        return false;
    }
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

#[inline]
fn check_bounding_boxes_intersect(line_1: &LineSegment, line_2: &LineSegment) -> bool {
    let line_1_min_x = line_1.0.x.min(line_1.1.x);
    let line_1_min_y = line_1.0.y.min(line_1.1.y);
    let line_1_max_x = line_1.0.x.max(line_1.1.x);
    let line_1_max_y = line_1.0.y.max(line_1.1.y);
    let line_2_min_x = line_2.0.x.min(line_2.1.x);
    let line_2_min_y = line_2.0.y.min(line_2.1.y);
    let line_2_max_x = line_2.0.x.max(line_2.1.x);
    let line_2_max_y = line_2.0.y.max(line_2.1.y);
    line_1_min_x <= line_2_max_x &&
        line_1_max_x >= line_2_min_x &&
        line_1_min_y <= line_2_max_y &&
        line_1_max_y >= line_2_min_y
}

#[inline]
fn orientation(p: &Vector, q: &Vector, r: &Vector) -> Orientation {
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

#[inline]
fn on_segment(p: &Vector, q: &Vector, r: &Vector) -> bool
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
fn test_intersection_checking() {
    let original_line = LineSegment(Vector::new(0,0), Vector::new(10,0));
    let above_line = LineSegment(Vector::new(0,1), Vector::new(10,1));
    let colinear_line = LineSegment(Vector::new(8,0), Vector::new(13,0));
    let vertical_line = LineSegment(Vector::new(-1,-1), Vector::new(3,3));
    let random_line = LineSegment(Vector::new(-1,-1), Vector::new(-3,-3));

    assert!(!check_intersection(&original_line, &above_line));
    assert!(!check_intersection(&original_line, &random_line));
    assert!(check_intersection(&original_line, &colinear_line));
    assert!(check_intersection(&original_line, &vertical_line));

    let a = LineSegment(Vector::new(-8000,25000), Vector::new(-5290.945,12198.925));
    let b = LineSegment(Vector::new(-7000,19000), Vector::new(-5202.545,12041.925));
    assert!(check_intersection(&a, &b));
}

#[test]
fn orientation_test() {
    let p = Vector::new(1, 1);
    let q = Vector::new(5, 5);
    let r = Vector::new(3, -2); // to the 'right hand' side of the other points
    let orientation_r = orientation(&p, &q, &r);
    assert_eq!(orientation_r, Orientation::Clockwise);

    let new_r = Vector::new(3, 10); // to the 'left hand' side of the other points
    let orientation_new_r = orientation(&r, &q, &new_r);
    assert_eq!(orientation_new_r, Orientation::Anticlockwise);
}

#[test]
fn test_bounding_boxes() {
    let original_line = LineSegment(Vector::new(0,0), Vector::new(10,0));
    let above_line = LineSegment(Vector::new(0,1), Vector::new(10,1));
    let colinear_line = LineSegment(Vector::new(8,0), Vector::new(13,0));
    let vertical_line = LineSegment(Vector::new(-1,-1), Vector::new(3,3));
    let random_line = LineSegment(Vector::new(-1,-1), Vector::new(-3,-3));

    assert!(!check_bounding_boxes_intersect(&original_line, &above_line));
    assert!(!check_bounding_boxes_intersect(&original_line, &random_line));
    assert!(check_bounding_boxes_intersect(&original_line, &colinear_line));
    assert!(check_bounding_boxes_intersect(&original_line, &vertical_line));
}

#[test]
fn test_intersection() {
    let original_line = LineSegment(Vector::new(0,0), Vector::new(10,0));
    let above_line = LineSegment(Vector::new(0,1), Vector::new(10,1));
    let colinear_line = LineSegment(Vector::new(8,0), Vector::new(13,0));
    let random_line = LineSegment(Vector::new(-1,-1), Vector::new(-3,-3));
    let vertical_line = LineSegment(Vector::new(1,-1), Vector::new(1,3));

    assert_eq!(intersects(&original_line, &above_line), None);
    assert_eq!(intersects(&original_line, &random_line), None);
    assert_eq!(intersects(&original_line, &colinear_line), None); // we're curently saying that colinear doesn't intersect
    assert_eq!(intersects(&original_line, &vertical_line), Some(Vector::new(1, 0)));
    // TODO Add more test variations here...
}

// TODO: wrap all these tests into a module to reuse variables or something
