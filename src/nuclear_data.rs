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
    /// Constructor.
    pub fn new(
        rtype: ReactionType,
        nu_bar: T,
        energies: &[T],
        polynomial: &Polynomial<T>,
        reaction_cross_section: T,
    ) -> Self {
        todo!()
    }

    /// Get the cross section for the specified group.
    pub fn get_cross_section(group: usize) -> T {
        todo!()
    }

    /// Uses RNG to get new angle and energy after a reaction. This
    /// needs to be Rustified right away as this is too C-like.
    /// The current function takes to pointer as arguments that act as
    /// arrays (energy/angle_out), a max length for those arrays (max_...)
    /// and a pointer to an integer that gets updated with the actual
    /// length of the arrays (n_out). Change this asap
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

/// Structure used to hold a list of reactions.
#[derive(Debug)]
pub struct NuclearDataSpecies<T: Float> {
    /// List of reactions
    pub reactions: Vec<NuclearDataReaction<T>>,
}

impl<T: Float> NuclearDataSpecies<T> {
    /// Adds a reaction to the internal list.
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
pub type NuclearDataIsotope<T> = Vec<NuclearDataSpecies<T>>;

/// Top level structure used to handle all things related to
/// nuclear data.
#[derive(Debug)]
pub struct NuclearData<T: Float> {
    /// Total number of energy groups?
    pub num_energy_groups: u32, //usize?
    /// Reactions and cross sections are stored by isotopes,
    /// those being stored by species
    pub isotopes: Vec<NuclearDataIsotope<T>>,
    /// Overall energy layout
    pub energies: Vec<T>,
}

impl<T: Float> NuclearData<T> {
    /// Constructor.
    pub fn new(num_groups: u32, energy_low: T, energy_high: T) -> Self {
        todo!()
    }

    /// Adds an isotope to the internal list.
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

    /// Returns the energy group a specific energy belongs to.
    pub fn get_energy_groups(&self, energy: T) -> usize {
        //usize?
        todo!()
    }

    /// Returns the number of reactions for a given isotope.
    pub fn get_number_reactions(&self, isotope_index: usize) -> usize {
        //usize?
        todo!()
    }

    /// Returns the total cross section for a given energy group.
    pub fn get_total_cross_section(&self, isotope_index: usize, group: usize) -> T {
        todo!()
    }

    /// Returns the reaction-specific cross section for a given energy_group.
    pub fn get_reaction_cross_section(
        &self,
        react_index: usize,
        isotope_index: usize,
        group: usize,
    ) -> T {
        todo!()
    }
}
