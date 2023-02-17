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
pub struct Polynomial {
    aa: f64,
    bb: f64,
    cc: f64,
    dd: f64,
    ee: f64,
}

/// Lowest-level structure to represent a reaction.
#[derive(Debug)]
pub struct NuclearDataReaction {
    pub cross_section: Vec<f64>,
    pub reaction_type: ReactionType,
    pub nu_bar: f64,
}

/// Structure used to hold a list of reactions. Not defined
/// via an alias because we need an impl block.
#[derive(Debug)]
pub struct NuclearDataSpecies {
    pub reactions: Vec<NuclearDataReaction>,
}

/// Structure used to store cross sections for a given isotope?
pub type NuclearDataIsotope = Vec<NuclearDataSpecies>;

/// Top level structure used to handle all things related to
/// nuclear data.
#[derive(Debug)]
pub struct NuclearData {}
