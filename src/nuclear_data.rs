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

/// Structure used to hold a list of reactions. Not defined
/// via an alias because we need an impl block.
#[derive(Debug)]
pub struct NuclearDataSpecies<T: Float> {
    pub reactions: Vec<NuclearDataReaction<T>>,
}

/// Structure used to store cross sections for a given isotope?
pub type NuclearDataIsotope<T: Float> = Vec<NuclearDataSpecies<T>>;

/// Top level structure used to handle all things related to
/// nuclear data.
#[derive(Debug)]
pub struct NuclearData {}
