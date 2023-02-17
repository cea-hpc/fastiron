use num::Float;

/// Structure representing a plane of equation `A*x + B*y + C*z + D = 0`
/// (A,B,C) is normalized.
#[derive(Debug)]
pub struct MCGeneralPlane<T: Float> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub d: T,
}

/// I think this is supposed to be a list of MCGeneralPlane in QS,
/// modelling a geometry cell. See original code:
/// ```cpp
/// class MC_Facet_Geometry_Cell
/// {
///   public:
///     MC_General_Plane* _facet;
///     int _size;
/// };
/// ```
/// Other structures have had a similar conversion in this port.
pub type MCFacetGeometryCell<T: Float> = Vec<MCGeneralPlane<T>>;
