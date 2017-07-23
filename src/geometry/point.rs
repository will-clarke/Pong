extern crate num;

pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    pub fn new<T: num::ToPrimitive>(x: T, y: T) -> Point {
        Point {
            x: x.to_f64().unwrap(),
            y: y.to_f64().unwrap(),
        }
    }
}
