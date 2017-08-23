extern crate num;

use std::ops;
use geometry::angle::Angle;
use std::fmt;
use std;

static TAU: f64 = 2.0 * std::f64::consts::PI;

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new<T: num::ToPrimitive>(x: T, y: T) -> Vector {
        Vector {
            x: x.to_f64().unwrap(),
            y: y.to_f64().unwrap(),
        }
    }

    pub fn angle(&mut self) -> Angle {
        // 0 is upright; .25 is right; .5 is down; .75 is left
        Angle((((f64::atan2(self.x, self.y) / TAU ) + 1.0) % 1.0).abs())
    }

    // pub fn magnitude(&mut self) -> f64 {
    //     f64::sqrt((self.x * self.x) + (self.y * self.y))
    // }

}

impl ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Mul for Vector {
    // we're doing some funky vector cross products here.
    // No idea why.... ¯\_(ツ)_/¯
    type Output = f64;

    fn mul(self, other: Vector) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

impl ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[test]
fn test_multiplication() {
    let a = Vector { x: 10.0, y: 10.0 };
    let b = Vector { x: 7.0,  y: 5.0  };

    // cross product
    assert_eq!(a * b, -20.0);
}


#[test]
fn test_angle() {
    assert_eq!(Vector { x: 0.0, y: 1.0 }.angle(), Angle(0.0));
    assert_eq!(Vector { x: 1.0, y: 0.0 }.angle(), Angle(0.25));
    assert_eq!(Vector { x: 0.5, y: 0.5 }.angle(), Angle(0.125));
    assert_eq!(Vector { x: 1.0, y: 1.0 }.angle(), Angle(0.125));
    assert_eq!(Vector { x: 0.0, y: -1.0 }.angle(), Angle(0.5));
    assert_eq!(Vector { x: -1.0, y: 0.0 }.angle(), Angle(0.75));
}


// #[test]
// fn test_magnitude() {
//     let magnitude = Vector { x: 2.0, y: 5.0 }.magnitude();
//     let expected = 5.385;
//     assert!( magnitude - expected < 0.01 );
// }
