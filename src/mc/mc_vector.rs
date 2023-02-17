use num::Float;

/// Custom type for vector representation.
#[derive(Debug, Clone, PartialEq)]
pub struct MCVector<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> MCVector<T> {
    /// Return the vector's euclidian norm.
    pub fn length(&self) -> T {
        todo!()
    }

    /// Return the distance from this vector to another point,
    /// i.e. the distance between the two points.
    pub fn distance(&self, vv: &MCVector<T>) -> T {
        todo!()
    }

    /// Return the scalar product with the specified vector.
    pub fn dot(tmp: &MCVector<T>) -> T {
        todo!()
    }

    /// Return the vector product with the specified vector.
    pub fn cross(tmp: &MCVector<T>) -> MCVector<T> {
        todo!()
    }
}

impl<T: Float> Default for MCVector<T> {
    fn default() -> Self {
        todo!()
    }
}

// Standard operations implems
// Might need to redefine other functions in the above
// impl block as these ops consume the object in Rust
// but not in C++

impl<T: Float> core::ops::Add<MCVector<T>> for MCVector<T> {
    type Output = MCVector<T>;
    fn add(self, rhs: MCVector<T>) -> Self::Output {
        todo!()
    }
}

impl<T: Float> core::ops::Sub<MCVector<T>> for MCVector<T> {
    type Output = MCVector<T>;
    fn sub(self, rhs: MCVector<T>) -> Self::Output {
        todo!()
    }
}

impl<T: Float> core::ops::Mul<T> for MCVector<T> {
    type Output = MCVector<T>;
    fn mul(self, rhs: T) -> Self::Output {
        todo!()
    }
}

// Assign-operations implems

impl<T: Float> core::ops::AddAssign<MCVector<T>> for MCVector<T> {
    fn add_assign(&mut self, rhs: MCVector<T>) {
        todo!()
    }
}

impl<T: Float> core::ops::SubAssign<MCVector<T>> for MCVector<T> {
    fn sub_assign(&mut self, rhs: MCVector<T>) {
        todo!()
    }
}

impl<T: Float> core::ops::MulAssign<T> for MCVector<T> {
    fn mul_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl<T: Float> core::ops::DivAssign<T> for MCVector<T> {
    fn div_assign(&mut self, rhs: T) {
        todo!()
    }
}
