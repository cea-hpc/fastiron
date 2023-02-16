/// Structure used to store a material's information
#[derive(Debug)]
pub struct Material {}

/// Top level structure used to store each material's information.
/// change to an alias?
#[derive(Debug)]
pub struct MaterialDatabase {
    pub mat: Vec<Material>,
}
