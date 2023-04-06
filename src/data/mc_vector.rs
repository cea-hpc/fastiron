//! 3D vectors
//!
//! This modules contains a custom type for 3D vectors.

use std::fmt::Debug;

use num::FromPrimitive;

use crate::constants::{sim::TINY_FLOAT, CustomFloat};

/// Custom type for vector representation.
///
/// # Examples
///
/// ```rust
/// use fastiron::data::mc_vector::MCVector;
///
/// let v = MCVector {x: 1.0, y: 1.0, z: 1.0};
/// let mut w = MCVector {x: 2.0, y: 2.0, z: 2.0};
///
/// // v + w == (3.0, 3.0, 3.0)
/// assert_eq!(v + w, MCVector {x: 3.0, y: 3.0, z: 3.0});
/// // w/2 == v
/// w /= 2.0;
/// assert!(w.is_almost_equal(&v));
///
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MCVector<T: CustomFloat> {
    /// x axis coordinate.
    pub x: T,
    /// y axis coordinate.
    pub y: T,
    /// z axis coordinate.
    pub z: T,
}

impl<T: CustomFloat> MCVector<T> {
    /// Returns true if the vector is almost the zero element. This method is
    /// necessary because of floating-point errors.
    pub fn is_almost_zero(&self) -> bool {
        let threshold: T = FromPrimitive::from_f64(TINY_FLOAT).unwrap();
        (self.x.abs() < threshold) & (self.y.abs() < threshold) & (self.z.abs() < threshold)
    }

    /// Returns true if the vectors are almost equal. This method is
    /// necessary because of floating-point errors.
    pub fn is_almost_equal(&self, vv: &MCVector<T>) -> bool {
        (*self - *vv).is_almost_zero()
    }

    /// Return the vector's euclidian norm.
    pub fn length(&self) -> T {
        // using num implem might be the safest since x,y,z are T: Float
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Return the distance from this vector to another point,
    /// i.e. the distance between the two points.
    pub fn distance(&self, vv: &MCVector<T>) -> T {
        // distance is the norm of the difference
        // need to test whether this is better/different/worse than a regular computation
        (*self - *vv).length()
    }

    /// Return the scalar product with the specified vector.
    pub fn dot(&self, vv: &MCVector<T>) -> T {
        self.x * vv.x + self.y * vv.y + self.z * vv.z
    }

    /// Return the vector product with the specified vector.
    pub fn cross(&self, vv: &MCVector<T>) -> MCVector<T> {
        MCVector {
            x: self.y * vv.z - self.z * vv.y,
            y: self.z * vv.x - self.x * vv.z,
            z: self.x * vv.y - self.y * vv.x,
        }
    }
}

// Operators

impl<T: CustomFloat> core::ops::Add<MCVector<T>> for MCVector<T> {
    type Output = MCVector<T>;
    fn add(self, vv: MCVector<T>) -> Self::Output {
        MCVector {
            x: self.x + vv.x,
            y: self.y + vv.y,
            z: self.z + vv.z,
        }
    }
}

impl<T: CustomFloat> core::ops::Sub<MCVector<T>> for MCVector<T> {
    type Output = MCVector<T>;
    fn sub(self, vv: MCVector<T>) -> Self::Output {
        MCVector {
            x: self.x - vv.x,
            y: self.y - vv.y,
            z: self.z - vv.z,
        }
    }
}

impl<T: CustomFloat> core::ops::Mul<T> for MCVector<T> {
    type Output = MCVector<T>;
    fn mul(self, f: T) -> Self::Output {
        MCVector {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}

// Assign-operations implems

impl<T: CustomFloat> core::ops::AddAssign<MCVector<T>> for MCVector<T> {
    fn add_assign(&mut self, vv: MCVector<T>) {
        self.x += vv.x;
        self.y += vv.y;
        self.z += vv.z;
    }
}

impl<T: CustomFloat> core::ops::SubAssign<MCVector<T>> for MCVector<T> {
    fn sub_assign(&mut self, vv: MCVector<T>) {
        self.x -= vv.x;
        self.y -= vv.y;
        self.z -= vv.z;
    }
}

impl<T: CustomFloat> core::ops::MulAssign<T> for MCVector<T> {
    fn mul_assign(&mut self, f: T) {
        self.x *= f;
        self.y *= f;
        self.z *= f;
    }
}

impl<T: CustomFloat> core::ops::DivAssign<T> for MCVector<T> {
    fn div_assign(&mut self, f: T) {
        // cant make *= 1.0/f work :(
        assert!(!f.is_zero());
        self.x /= f;
        self.y /= f;
        self.z /= f;
    }
}
