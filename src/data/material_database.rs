//! Material-related data and modelling
//!
//! This module contains code used to store material-related data. It is mainly used when
//! initializing the simulation, in order to build the environment of the problem.

use crate::constants::CustomFloat;

/// Structure used to represent an isotope of a material.
#[derive(Debug, Default)]
pub struct Isotope<T: CustomFloat> {
    /// Global identifier of the isotope in NuclearData.
    pub gid: usize,
    /// Atomic fraction of the isotope in the material. Not to be confused with
    /// the related quantity _atomic ratio_.
    pub atom_fraction: T,
}

/// Structure used to store a material's information.
#[derive(Debug)]
pub struct Material<T: CustomFloat> {
    /// Name of the material.
    pub name: String,
    /// Mass of the material in grams.
    pub mass: T,
    /// List of the isotopes making up the material.
    pub iso: Vec<Isotope<T>>,
}

impl<T: CustomFloat> Material<T> {
    /// Adds an [Isotope] to the internal list.
    pub fn add_isotope(&mut self, isotope: Isotope<T>) {
        self.iso.push(isotope);
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
