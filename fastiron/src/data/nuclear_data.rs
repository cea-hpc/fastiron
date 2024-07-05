//! Nuclear-related data and modelling
//!
//! This module contains code used to store, model and compute nuclear data and quantities.

use num::{zero, FromPrimitive};
use rustc_hash::FxHashMap;

use crate::{constants::CustomFloat, parameters::MaterialParameters};

/// Enum used to represent a reaction type.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReactionType {
    /// Value for a scattering reaction.
    Scatter,
    /// Value for an absorption reaction.
    Absorption,
    /// Value for a fission reaction.
    Fission,
}

/// Structure used to represent a polynomial function.
///
/// Private fields represent the coefficients, `aa` corresponding to the leading
/// coefficient , `ee` to the product of all roots.
#[derive(Debug)]
pub struct Polynomial<T: CustomFloat> {
    /// Leading coefficient of the polynomial function.
    pub aa: T,
    /// Degree 3 coefficient of the polynomial function.
    pub bb: T,
    /// Degree 2 coefficient of the polynomial function.
    pub cc: T,
    /// Degree 1 coefficient of the polynomial function.
    pub dd: T,
    /// Degree 0 coefficient of the polynomial function.
    pub ee: T,
}

impl<T: CustomFloat> Polynomial<T> {
    /// Returns the value of the polynomial function in `xx`.
    pub fn val(&self, xx: T) -> T {
        self.ee + xx * (self.dd + xx * (self.cc + xx * (self.bb + xx * self.aa)))
    }
}

/// Lowest-level structure to represent a reaction.
///
/// A reaction is characterized by three elements: a probability density
/// (cross-section), the type of the reaction and a statistical offset
/// (nu bar).
#[derive(Debug)]
pub struct NuclearDataReaction<T: CustomFloat> {
    /// Cross-section of the reaction, i.e. the discretization of its
    /// probability density function. Discretization is done according to
    /// energy levels.
    pub cross_section: Vec<T>,
    /// Type of the reaction.
    pub reaction_type: ReactionType,
    /// Statistical offset. This value is used when sampling for the number
    /// of offspring particles created by a reaction. See the
    /// [`sample_collision()`][crate::particles::mc_particle::MCParticle::sample_collision()] method
    /// for more information.
    pub nu_bar: T,
}

impl<T: CustomFloat> NuclearDataReaction<T> {
    /// Constructor.
    pub fn new(
        rtype: ReactionType,
        nu_bar: T,
        energies: &[T],
        polynomial: &Polynomial<T>,
        reaction_cross_section: T,
    ) -> Self {
        let n_groups = energies.len() - 1;
        let mut xsection: Vec<f64> = vec![zero(); n_groups];
        let mut normal_value: f64 = 0.0;
        let two: T = FromPrimitive::from_f64(2.0).unwrap();

        (0..n_groups).for_each(|ii| {
            let energy: T = (energies[ii] + energies[ii + 1]) / two;
            // 10^(Poly(log10(energy)))
            // this here gives a value too big for f32
            xsection[ii] = 10.0_f64.powf(polynomial.val(energy.log10()).to_f64().unwrap());

            if (energies[ii + 1] >= 1.0.into()) & (normal_value == 0.0) {
                normal_value = xsection[ii];
            }
        });
        let scale = reaction_cross_section.to_f64().unwrap() / normal_value;
        (0..n_groups).for_each(|ii| {
            xsection[ii] *= scale;
        });

        let xsection_t: Vec<T> = xsection
            .iter()
            .map(|ff| FromPrimitive::from_f64(*ff).unwrap())
            .collect();

        Self {
            cross_section: xsection_t,
            reaction_type: rtype,
            nu_bar,
        }
    }
}

/// Structure used to hold a list of reactions.
///
/// The list of reaction held by the structure is specific to a particle species.
/// At the current time there is only one type of particle in the simulation.
#[derive(Debug, Default)]
pub struct NuclearDataSpecies<T: CustomFloat> {
    /// List of reactions.
    pub reactions: Vec<NuclearDataReaction<T>>,
}

impl<T: CustomFloat> NuclearDataSpecies<T> {
    /// Builds a reaction and adds it to the internal list.
    pub fn add_reaction(
        &mut self,
        rtype: ReactionType,
        nu_bar: T,
        energies: &[T],
        polynomial: &Polynomial<T>,
        reaction_cross_section: T,
    ) {
        self.reactions.push(NuclearDataReaction::new(
            rtype,
            nu_bar,
            energies,
            polynomial,
            reaction_cross_section,
        ))
    }
}

/// Structure used to store reactions for a given isotope.
///
/// At the current time there is only one type of particle in the simulation,
/// meaning that there is only one species per isotope.
pub type NuclearDataIsotope<T> = Vec<NuclearDataSpecies<T>>;

/// Top level structure used to handle all things related to
/// nuclear data.
#[derive(Debug, Default)]
pub struct NuclearData<T: CustomFloat> {
    /// Total number of energy groups.
    pub num_energy_groups: usize,
    /// Isotope-sorted reaction list.
    pub isotopes: Vec<NuclearDataIsotope<T>>,
    /// Discrete value of the energy spectrum.
    pub energies: Vec<T>,
}

impl<T: CustomFloat> NuclearData<T> {
    /// Constructor.
    pub fn new(num_groups: usize, energy_low: T, energy_high: T) -> Self {
        let mut energies = vec![zero(); num_groups + 1];
        let length: T = FromPrimitive::from_usize(num_groups + 1).unwrap();
        // complete energy levels
        energies[0] = energy_low;
        energies[num_groups] = energy_high;
        let log_low: T = energy_low.ln();
        let log_high: T = energy_high.ln();
        let delta = (log_high - log_low) / length;

        (1..num_groups).for_each(|ii| {
            let step = FromPrimitive::from_usize(ii).unwrap();
            let log_value: T = log_low + delta * step;
            energies[ii] = log_value.exp();
        });

        Self {
            num_energy_groups: num_groups,
            isotopes: Vec::new(),
            energies,
        }
    }

    /// Adds an isotope to the internal list.
    ///
    /// The isotope is built from the material data and cross-sections
    /// specified in the material.
    pub fn add_isotope(
        &mut self,
        cross_section: &FxHashMap<String, Polynomial<T>>,
        mp: &MaterialParameters<T>,
        nu_bar: T,
    ) -> usize {
        // for readability purposes
        let n_reactions: usize = mp.n_reactions;
        let fission_function: &Polynomial<T> = &cross_section[&mp.fission_cross_section];
        let scatter_function: &Polynomial<T> = &cross_section[&mp.scattering_cross_section];
        let absorption_function: &Polynomial<T> = &cross_section[&mp.absorption_cross_section];
        let total_cross_section: T = mp.total_cross_section;
        let fission_weight: T = mp.fission_cross_section_ratio;
        let scatter_weight: T = mp.scattering_cross_section_ratio;
        let absorption_weight: T = mp.absorbtion_cross_section_ratio;

        self.isotopes.push(vec![NuclearDataSpecies::default()]);
        let total_weight = fission_weight + scatter_weight + absorption_weight;

        let mut n_fission = n_reactions / 3;
        let mut n_scatter = n_reactions / 3;
        let n_absorption = n_reactions / 3;

        // set reaction distribution
        match n_reactions % 3 {
            2 => {
                n_fission += 1;
                n_scatter += 1;
            }
            1 => {
                n_scatter += 1;
            }
            0 => (),
            _ => unreachable!(),
        }
        let mut f: T = FromPrimitive::from_usize(n_fission).unwrap();
        let fission_xsection: T = (total_cross_section * fission_weight) / (f * total_weight);
        f = FromPrimitive::from_usize(n_scatter).unwrap();
        let scatter_xsection: T = (total_cross_section * scatter_weight) / (f * total_weight);
        f = FromPrimitive::from_usize(n_absorption).unwrap();
        let absorption_xsection: T = (total_cross_section * absorption_weight) / (f * total_weight);

        let n = self.isotopes.len();
        self.isotopes[n - 1][0].reactions.reserve(n_reactions);

        (0..n_reactions).for_each(|ii| match ii % 3 {
            0 => self.isotopes[n - 1][0].add_reaction(
                ReactionType::Scatter,
                nu_bar,
                &self.energies,
                scatter_function,
                scatter_xsection,
            ),
            1 => self.isotopes[n - 1][0].add_reaction(
                ReactionType::Fission,
                nu_bar,
                &self.energies,
                fission_function,
                fission_xsection,
            ),
            2 => self.isotopes[n - 1][0].add_reaction(
                ReactionType::Absorption,
                nu_bar,
                &self.energies,
                absorption_function,
                absorption_xsection,
            ),
            _ => unreachable!(),
        });
        self.isotopes.len() - 1
    }

    /// Returns the energy group a specific energy belongs to by using a dichotomy search.
    pub fn get_energy_groups(&self, energy: T) -> usize {
        let num_energies = self.energies.len();

        // extreme low
        if energy <= self.energies[0] {
            return 0;
        }
        // extreme high
        if energy > self.energies[num_energies - 1] {
            return num_energies - 1;
        }

        // dichotomy search
        let mut high = num_energies - 1;
        let mut low: usize = 0;

        while high != low + 1 {
            let mid = (high + low) / 2;
            if energy < self.energies[mid] {
                high = mid;
            } else {
                low = mid;
            }
        }

        low
    }

    /// Returns the total cross-section for a given energy group.
    pub fn get_total_cross_section(&self, isotope_index: usize, group: usize) -> T {
        // sum all reaction's xsection for a given isotope at a given energy level
        self.isotopes[isotope_index][0]
            .reactions
            .iter()
            .map(|reaction| reaction.cross_section[group])
            .sum()
    }
}
