use num::Float;

/// Structure used to represent an isotope.
#[derive(Debug)]
pub struct Isotope<T: Float> {
    /// Global identifier of the isotope in NuclearData.
    pub gid: usize,
    /// Atomic fraction.
    atom_fraction: T,
}

impl<T: Float> Default for Isotope<T> {
    fn default() -> Self {
        todo!()
    }
}

/// Structure used to store a material's information
#[derive(Debug)]
pub struct Material<T: Float> {
    /// Name of the material
    name: String,
    /// Mass of the material (kg).
    mass: T,
    /// List of present isotopes.
    iso: Vec<Isotope<T>>, // originally a qs_vector
}

impl<T: Float> Material<T> {
    /// Constructor.
    pub fn new(name: &str) -> Self {
        todo!()
    }

    /// Adds an [Isotope] to the internal list.
    pub fn add_isotope(&mut self, isotope: Isotope<T>) {
        todo!()
    }
}

impl<T: Float> Default for Material<T> {
    fn default() -> Self {
        todo!()
    }
}

/// Top level structure used to store each material's information.
/// change to an alias?
#[derive(Debug)]
pub struct MaterialDatabase<T: Float> {
    /// List of materials.
    pub mat: Vec<Material<T>>, // originally a qs_vector
}

impl<T: Float> MaterialDatabase<T> {
    /// Adds a [Material] to the internal list.
    pub fn add_material(&mut self, material: Material<T>) {
        todo!()
    }

    /// Returns the index of the material passed as argument.
    /// !!!! Undefined behavior if absent or duplicate !!!!
    pub fn find_material(&self, name: &str) -> usize {
        todo!()
    }
}
