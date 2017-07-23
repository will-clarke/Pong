extern crate num;
use std::ops;

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new<T: num::ToPrimitive>(x: T, y: T) -> Point {
        Point {
            x: x.to_f64().unwrap(),
            y: y.to_f64().unwrap(),
        }
    }
}

impl ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Mul for Point {
    // we're doing some funky vector cross products here.
    // No idea why.... ¯\_(ツ)_/¯
    type Output = f64;

    fn mul(self, other: Point) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

// impl ops::Add for Point {
//     type output = Point;
//     fn add(self, other: Point) -> Point {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }


impl ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// std::cmp::PartialEq` might be missing for `std::option::Option<geometry::point::Point>`
