use num::Float;

/// Enum representing a reaction type. Named `Enum` in
/// the original code.
#[derive(Debug)]
pub enum ReactionType {
    Undefined,
    Scatter,
    Absorption,
    Fission,
}

/// Structure used to represent a polynomial function.
/// Private field represent the coefficient, `aa`
/// corresponding to `x^4`, `ee` to `x^0`.
#[derive(Debug)]
pub struct Polynomial<T: Float> {
    aa: T,
    bb: T,
    cc: T,
    dd: T,
    ee: T,
}

/// Lowest-level structure to represent a reaction.
#[derive(Debug)]
pub struct NuclearDataReaction<T: Float> {
    pub cross_section: Vec<T>,
    pub reaction_type: ReactionType,
    pub nu_bar: T,
}

impl<T: Float> NuclearDataReaction<T> {
    pub fn new(
        rtype: ReactionType,
        nu_bar: T,
        energies: &[T],
        polynomial: &Polynomial<T>,
        reaction_cross_section: T,
    ) -> Self {
        todo!()
    }

    pub fn get_cross_section(group: usize) -> T {
        todo!()
    }

    pub fn sample_collision(
        incident_energy: T,
        material_mass: T,
        energy_out: &T,
        angle_out: &T,
        n_out: u32,
        seed: &u64,
        max_production_size: u32,
    ) {
        todo!()
    }
}

/// Structure used to hold a list of reactions. Not defined
/// via an alias because we need an impl block.
#[derive(Debug)]
pub struct NuclearDataSpecies<T: Float> {
    pub reactions: Vec<NuclearDataReaction<T>>,
}

impl<T: Float> NuclearDataSpecies<T> {
    pub fn add_reaction(
        &mut self,
        rtype: ReactionType,
        nu_bar: T,
        energies: &[T],
        polynomial: &Polynomial<T>,
        reaction_cross_section: T,
    ) {
        todo!()
    }
}

/// Structure used to store cross sections for a given isotope?
pub type NuclearDataIsotope<T: Float> = Vec<NuclearDataSpecies<T>>;

/// Top level structure used to handle all things related to
/// nuclear data.
#[derive(Debug)]
pub struct NuclearData<T: Float> {
    pub num_energy_groups: u32, //usize?
    pub isotopes: Vec<NuclearDataIsotope<T>>,
    pub energies: Vec<T>,
}

impl<T: Float> NuclearData<T> {
    pub fn new(num_groups: u32, energy_low: T, energy_high: T) -> Self {
        todo!()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_isotope(
        &mut self,
        n_reactions: u64,
        fission_function: &Polynomial<T>,
        scatter_function: &Polynomial<T>,
        absorption_function: &Polynomial<T>,
        nu_bar: T,
        total_cross_section: T,
        fission_weight: T,
        scatter_weight: T,
        absorption_weight: T,
    ) {
        todo!()
    }

    pub fn get_energy_groups(&self, energy: T) -> u32 {
        //usize?
        todo!()
    }

    pub fn get_number_reactions(&self, isotope_index: usize) -> u32 {
        //usize?
        todo!()
    }

    pub fn get_total_cross_section(&self, isotope_index: usize, group: usize) -> T {
        todo!()
    }

    pub fn get_reaction_cross_section(
        &self,
        react_index: usize,
        isotope_index: usize,
        group: usize,
    ) -> T {
        todo!()
    }
}
