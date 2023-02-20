use num::Float;

#[derive(Debug)]
pub struct Isotope<T: Float> {
    // Global id in NuclearData of the isotope.
    pub gid: usize,
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
    name: String,
    mass: T,
    iso: Vec<Isotope<T>>, // originally a qs_vector
}

impl<T: Float> Material<T> {
    pub fn new(name: &str) -> Self {
        todo!()
    }

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
    pub mat: Vec<Material<T>>, // originally a qs_vector
}

impl<T: Float> MaterialDatabase<T> {
    pub fn add_material(&mut self, material: Material<T>) {
        todo!()
    }

    pub fn find_material(&self, name: &str) -> usize {
        todo!()
    }
}
