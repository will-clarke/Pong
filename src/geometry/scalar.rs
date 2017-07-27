use geometry::vector::Vector;
use std::ops;

#[allow(dead_code)]
pub struct Scalar(pub f64);

impl ops::Mul<Vector> for Scalar {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Vector {
        Vector {
            x: vector.x * self.0,
            y: vector.y * self.0,
        }
    }
}
