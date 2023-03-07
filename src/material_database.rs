use num::{zero, Float, FromPrimitive};

/// Structure used to represent an isotope.
#[derive(Debug)]
pub struct Isotope<T: Float> {
    /// Global identifier of the isotope in NuclearData.
    pub gid: usize,
    /// Atomic fraction.
    pub atom_fraction: T,
}

impl<T: Float> Default for Isotope<T> {
    fn default() -> Self {
        Self {
            gid: 0,
            atom_fraction: zero(),
        }
    }
}

/// Structure used to store a material's information
#[derive(Debug)]
pub struct Material<T: Float> {
    /// Name of the material
    pub name: String,
    /// Mass of the material (kg).
    pub mass: T,
    /// List of present isotopes.
    pub iso: Vec<Isotope<T>>, // originally a qs_vector
}

impl<T: Float + FromPrimitive> Material<T> {
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

impl<T: Float + FromPrimitive> Default for Material<T> {
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
/// change to an alias?
#[derive(Debug, Default)]
pub struct MaterialDatabase<T: Float + FromPrimitive> {
    /// List of materials.
    pub mat: Vec<Material<T>>, // originally a qs_vector
}

impl<T: Float + FromPrimitive> MaterialDatabase<T> {
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
