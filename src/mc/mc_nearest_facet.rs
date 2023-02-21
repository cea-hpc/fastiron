use num::Float;

#[derive(Debug, Clone)]
pub struct MCNearestFacet<T: Float> {
    pub facet: u32,
    pub distance_to_facet: T,
    pub dot_product: T,
}

impl<T: Float> Default for MCNearestFacet<T> {
    fn default() -> Self {
        todo!()
    }
}
