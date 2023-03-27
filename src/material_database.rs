use num::FromPrimitive;

use crate::constants::CustomFloat;

/// Structure used to represent an isotope.
#[derive(Debug, Default)]
pub struct Isotope<T: CustomFloat> {
    /// Global identifier of the isotope in NuclearData.
    pub gid: usize,
    /// Atomic fraction.
    pub atom_fraction: T,
}

/// Structure used to store a material's information
#[derive(Debug)]
pub struct Material<T: CustomFloat> {
    /// Name of the material
    pub name: String,
    /// Mass of the material (kg).
    pub mass: T,
    /// List of present isotopes.
    pub iso: Vec<Isotope<T>>,
}

impl<T: CustomFloat> Material<T> {
    /// Constructor.
    pub fn new(name: &str) -> Self {
        Material {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Adds an [Isotope] to the internal list.
    pub fn add_isotope(&mut self, isotope: Isotope<T>) {
        self.iso.push(isotope);
    }
}

impl<T: CustomFloat> Default for Material<T> {
    fn default() -> Self {
        let m: T = FromPrimitive::from_f32(1000.0).unwrap();
        Self {
            name: "0".to_string(),
            mass: m,
            iso: Vec::new(),
        }
    }
}

/// Top level structure used to store each material's information.
#[derive(Debug, Default)]
pub struct MaterialDatabase<T: CustomFloat> {
    /// List of materials.
    pub mat: Vec<Material<T>>,
}

impl<T: CustomFloat> MaterialDatabase<T> {
    /// Adds a [Material] to the internal list.
    pub fn add_material(&mut self, material: Material<T>) {
        self.mat.push(material);
    }

    /// Returns the index of the material passed as argument.
    /// If there is a duplicate, only the first one in the list
    /// will be "visible".
    pub fn find_material(&self, name: &str) -> Option<usize> {
        self.mat.iter().position(|m| m.name == name)
    }
}
