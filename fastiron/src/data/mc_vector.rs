//! 3D vectors
//!
//! This module contains a custom type for 3D vectors.

use std::{fmt::Debug, iter::Sum};

use crate::constants::CustomFloat;

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
/// assert_eq!(v, w);
///
/// ```
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Default)]
pub struct MCVector<T: CustomFloat> {
    /// x axis coordinate.
    pub x: T,
    /// y axis coordinate.
    pub y: T,
    /// z axis coordinate.
    pub z: T,
}

impl<T: CustomFloat> MCVector<T> {
    #[cfg(any(test, doc))]
    /// Returns true if the vector is almost the zero element. This method is
    /// necessary because of floating-point errors.
    pub fn is_almost_zero(&self) -> bool {
        let threshold: T = T::tiny_float();
        (self.x.abs() < threshold) & (self.y.abs() < threshold) & (self.z.abs() < threshold)
    }

    #[cfg(any(test, doc))]
    /// Returns true if the vectors are almost equal. This method is
    /// necessary because of floating-point errors.
    pub fn is_almost_equal(&self, vv: &MCVector<T>) -> bool {
        (*self - *vv).is_almost_zero()
    }

    /// Return the vector's Euclidean norm.
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

impl<'a, T: CustomFloat> Sum<&'a MCVector<T>> for MCVector<T> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |uu, vv| uu + *vv)
    }
}

impl<T: CustomFloat> Sum<MCVector<T>> for MCVector<T> {
    fn sum<I: Iterator<Item = MCVector<T>>>(iter: I) -> Self {
        iter.fold(Self::default(), |uu, vv| uu + vv)
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

// Assign-operations implementations

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

#[cfg(test)]
mod test {
    use super::*;
    use num::Float;

    #[test]
    fn add() {
        let uu = MCVector {
            x: 1.0 / 3.0,
            y: 1.0343253253254332 / 3.0,
            z: -1.0 / 3.0,
        };
        let vv = MCVector {
            x: 2.0 / 3.0,
            y: 31.0 / 3.0,
            z: 2.0 / 3.0,
        };
        let ww = MCVector {
            x: 1.0,
            y: 32.03432532532543 / 3.0,
            z: 1.0 / 3.0,
        };
        assert_eq!(uu + vv, ww);
    }

    #[test]
    fn add_assign() {
        let mut uu = MCVector {
            x: 1.0 / 3.0,
            y: 1.0343253253254332 / 3.0,
            z: -1.0 / 3.0,
        };
        let vv = MCVector {
            x: 2.0 / 3.0,
            y: 31.0 / 3.0,
            z: 2.0 / 3.0,
        };
        let ww = MCVector {
            x: 1.0,
            y: 32.03432532532543 / 3.0,
            z: 1.0 / 3.0,
        };
        uu += vv;
        assert_eq!(uu, ww);
    }

    #[test]
    fn sub() {
        let uu = MCVector {
            x: 1.0 / 3.0,
            y: 1.0343253253254332,
            z: 1.0 / 3.0,
        };
        let vv = MCVector {
            x: 2.0 / 3.0,
            y: 31.0,
            z: -2.0 / 3.0,
        };
        // Exact equality work in this case, but this isn't
        // consistent. For example, dividing by 3 all y coords
        // will make it fail because of error propagation.
        let ww = MCVector {
            x: -1.0 / 3.0,
            //y: -29.9656746746745668,
            //y: -29.965674674674566,
            y: -29.965674674674567,
            z: 1.0,
        };
        assert_eq!(uu - vv, ww);
    }

    #[test]
    fn sub_assign() {
        let mut uu = MCVector {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let vv = MCVector {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let ww = MCVector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        uu -= vv;
        assert_eq!(uu, ww);
    }

    #[test]
    fn mul() {
        let uu = MCVector {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let f = 2.0;
        let ww = MCVector {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };
        assert_eq!(uu * f, ww);
    }

    #[test]
    fn mul_assign() {
        let mut uu = MCVector {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let f = 2.0;
        let ww = MCVector {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };
        uu *= f;
        assert_eq!(uu, ww);
    }

    #[test]
    fn div_assign() {
        let mut uu = MCVector {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let f = 2.0;
        let ww = MCVector {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        };
        uu /= f;
        assert_eq!(uu, ww);
    }

    #[test]
    #[should_panic]
    fn div_assign_zero() {
        let mut uu = MCVector {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        uu /= 0.0;
    }

    // Sums

    #[test]
    fn sum_owned() {
        let vec = [
            MCVector {
                x: 2.0,
                y: 2.0,
                z: 2.0,
            },
            MCVector {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            MCVector {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
            MCVector {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
        ];

        let res: MCVector<f64> = vec.iter().copied().sum();

        assert_eq!(
            res,
            MCVector {
                x: 4.0,
                y: 4.0,
                z: 4.0
            }
        )
    }

    #[test]
    fn sum_borrowed() {
        let vec = [
            MCVector {
                x: 2.0,
                y: 2.0,
                z: 2.0,
            },
            MCVector {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            MCVector {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
            MCVector {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
        ];

        let res: MCVector<f64> = vec.iter().sum();

        assert_eq!(
            res,
            MCVector {
                x: 4.0,
                y: 4.0,
                z: 4.0
            }
        )
    }

    // Pushing the boundaries

    #[test]
    fn floating_point_error() {
        let uu = MCVector {
            x: 1.0 / 3.0,
            y: 1.0343253253254332 / 3.0,
            z: 1.0 / 3.0,
        };
        let vv = MCVector {
            x: 2.0 / 3.0,
            y: 31.0 / 3.0,
            z: -2.0 / 3.0,
        };
        let ww = MCVector {
            x: -1.0 / 3.0,
            //y: -29.9656746746745668/3.0, // error with exact value
            //y: -29.965674674674566/3.0, // error with truncated value
            y: -29.965674674674567 / 3.0, // error with rounded value
            z: 1.0,
        };
        // instead of checking for exact equality, we check that the
        // difference is close enough to zero
        assert!((uu - vv).is_almost_equal(&ww));
    }

    // Methods

    #[test]
    fn length() {
        // trivial case
        let mut uu = MCVector {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        assert_eq!(uu.length(), 3.0.sqrt());

        // negative & non rational coordinates
        uu.x = -1.0;
        uu.y = 23.0.sqrt();
        assert_eq!(uu.length(), 5.0);
    }

    #[test]
    fn distance() {
        let uu = MCVector {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let ww = MCVector {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };
        assert_eq!(uu.distance(&ww), 3.0.sqrt());
    }

    #[test]
    fn dot_product() {
        let uu = MCVector {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let ww = MCVector {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };
        assert_eq!(uu.dot(&ww), 6.0);
    }

    #[test]
    fn cross_product() {
        let uu = MCVector {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let ww = MCVector {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };
        assert_eq!(uu.cross(&ww), MCVector::default()); // default is the zero element
    }
}
