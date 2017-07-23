use geometry::point::Point;

pub struct Line(pub Point, pub Point);


#[test]
fn test_initalize() {
    let p1 = Point::new(1, 2);
    let p2 = Point::new(3, 4);
    let line = Line(p1, p2);
    assert_eq!(line.0.x, 1.0);
    assert_eq!(line.0.y, 2.0);
    assert_eq!(line.1.x, 3.0);
    assert_eq!(line.1.y, 4.0);
}

#[test]
fn tmp_delete() {
    extern crate num;
    fn omg<T: num::ToPrimitive>(x: T) -> i32 {
        x.to_i32().unwrap()
    }
    assert_eq!(2, omg(2));
    assert_eq!(2, omg(2.0 as f32));
    assert_eq!(2, omg(2.0 as f64));
}
