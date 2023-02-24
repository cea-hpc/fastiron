use num::{Float, FromPrimitive, zero};

use crate::mc::mc_rng_state::rng_sample;

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
/// Private fields represent the coefficients, `aa`
/// corresponding to `x^4`, `ee` to `x^0`.
#[derive(Debug)]
pub struct Polynomial<T: Float> {
    aa: T,
    bb: T,
    cc: T,
    dd: T,
    ee: T,
}

impl<T: Float> Polynomial<T> {
    /// Returns the value of the polynomial function in xx.
    pub fn val(&self, xx: T) -> T {
        self.ee + xx * (self.dd + xx * (self.cc + xx * (self.bb + xx * self.aa)))
    }
}

/// Lowest-level structure to represent a reaction.
#[derive(Debug)]
pub struct NuclearDataReaction<T: Float> {
    pub cross_section: Vec<T>,
    pub reaction_type: ReactionType,
    pub nu_bar: T,
}

impl<T: Float + FromPrimitive> NuclearDataReaction<T> {
    /// Constructor.
    pub fn new(
        rtype: ReactionType,
        nu_bar: T,
        energies: &[T],
        polynomial: &Polynomial<T>,
        reaction_cross_section: T,
    ) -> Self {
        let n_groups = energies.len()-1;
        let mut xsection: Vec<T> = Vec::with_capacity(n_groups);

        let mut normal_value: T = zero();
        let one: T = FromPrimitive::from_f32(1.0).unwrap();

        (0..n_groups).into_iter().for_each(|ii| {
            let factor: T = FromPrimitive::from_f32(2.0).unwrap();
            let energy: T = (energies[ii] + energies[ii+1]) / factor;
            // 10^(Poly(log10(energy)))
            let base: T = FromPrimitive::from_f32(10.0).unwrap();
            xsection[ii] = base.powf(polynomial.val(energy.log10()));

            if (normal_value==zero()) & (xsection[ii] > one) {
                normal_value = xsection[ii];
            }

            let scale = reaction_cross_section/normal_value;
            // replace with map later?
            (0..n_groups).into_iter().for_each(|ii| {
                xsection[ii] = xsection[ii]*scale;
            });
        });

        Self { cross_section: xsection, reaction_type: rtype, nu_bar }
    }

    /// Get the cross section for the specified group. Delete and make 
    /// cross_section public?
    pub fn get_cross_section(&self, group: usize) -> T {
        self.cross_section[group]
    }

    /// Uses RNG to get new angle and energy after a reaction. In
    /// case of fission, at most `max_production_size` particles 
    /// can result. Since reaction type is specified when the 
    /// method is called, we assume that the result will be treated
    /// correctly by the calling code.
    pub fn sample_collision(&self, 
        incident_energy: T,
        material_mass: T,
        seed: &mut u64,
        //max_production_size: usize,
    ) -> (Vec<T>, Vec<T>){
        let one: T = FromPrimitive::from_f32(1.0).unwrap();
        let two: T = one+one;
        let mut energy_out: Vec<T> = Vec::new();
        let mut angle_out: Vec<T> = Vec::new();
        match self.reaction_type {
            ReactionType::Scatter => {
                let mut rand_n: T = rng_sample(seed);
                energy_out.push(incident_energy * (one - rand_n*(one/material_mass)));
                rand_n = rng_sample(seed); 
                angle_out.push(rand_n*(two) - one)
            },
            ReactionType::Absorption => (),
            ReactionType::Fission => {
                // the expected behavior of this part in the original code
                // is quite unclear. There is an assert but it only prints 
                // a message, not stop the method
                let num_particle_out = (self.nu_bar + rng_sample(seed)).to_usize().unwrap();
                energy_out.reserve(num_particle_out);
                angle_out.reserve(num_particle_out);
                (0..num_particle_out).into_iter().for_each(|ii| {
                    let mut rand_n: T = rng_sample(seed);
                    rand_n = (rand_n + one) / two;
                    let twenty: T = FromPrimitive::from_f32(20.0).unwrap();
                    energy_out[ii] = twenty * rand_n * rand_n;
                    rand_n = rng_sample(seed);
                    angle_out[ii] = rand_n*two - one;
                })
            },
            ReactionType::Undefined => {
                panic!()
            }
        }
        (energy_out, angle_out)
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
