/// Structure representing a plane of equation `A*x + B*y + C*z + D = 0`
/// (A,B,C) is normalized.
#[derive(Debug)]
pub struct MCGeneralPlane {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
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
pub type MCFacetGeometryCell = Vec<MCGeneralPlane>;
