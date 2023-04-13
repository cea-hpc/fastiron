//! Code for simulation-related statistics
//!
//! This module contains all code used to record and count events happening
//! during the simulation. The cyclic summary is printed using the recorded data.
//!
//! Note that this module isn't used to compute time-related data, this is done in
//! the [utils::mc_fast_timer][crate::utils::mc_fast_timer] module.

use std::fmt::Debug;

use num::zero;

use crate::{
    constants::CustomFloat,
    geometry::mc_domain::MCDomain,
    montecarlo::MonteCarlo,
    parameters::BenchType,
    utils::mc_fast_timer::{self, Section},
};

use super::energy_spectrum::EnergySpectrum;

/// Enum representing a tally event.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum MCTallyEvent {
    /// Value for a collision event.
    Collision,
    /// Value for a facet crossing event resulting in a cell exit.
    FacetCrossingTransitExit,
    /// Value for a census event.
    #[default]
    Census,
    /// Value for a facet crossing event resulting in an escape from the problem.
    FacetCrossingEscape,
    /// Value for a facet crossing event resulting in a reflection on the facet.
    FacetCrossingReflection,
    /// Value for a facet crossing event resulting in a cell exit to an
    /// off-processor cell.
    FacetCrossingCommunication,
}

//========
// Fluence
//========

/// Structure used to compute fluence.
///
/// The data of each cell is grouped by domains using the [FluenceDomain]
/// sub-structure.
#[derive(Debug, Default)]
pub struct Fluence<T: CustomFloat> {
    pub domain: Vec<FluenceDomain<T>>,
}

impl<T: CustomFloat> Fluence<T> {
    pub fn compute(&mut self, domain_idx: usize, scalar_flux_domain: &ScalarFluxDomain<T>) {
        let n_cells = scalar_flux_domain.cell.len();
        while self.domain.len() <= domain_idx {
            let new_domain: FluenceDomain<T> = FluenceDomain {
                cell: vec![zero(); n_cells],
            };
            self.domain.push(new_domain);
        }
        (0..n_cells).for_each(|cell_idx| {
            let n_groups = scalar_flux_domain.cell[cell_idx].len();
            (0..n_groups).for_each(|group_idx| {
                self.domain[domain_idx]
                    .add_to_cell(cell_idx, scalar_flux_domain.cell[cell_idx][group_idx]);
            });
        });
    }
}

/// Domain-sorted fluence-data-holding sub-structure.
#[derive(Debug, Default)]
pub struct FluenceDomain<T: CustomFloat> {
    pub cell: Vec<T>,
}

impl<T: CustomFloat> FluenceDomain<T> {
    pub fn add_to_cell(&mut self, index: usize, val: T) {
        self.cell[index] += val;
    }

    pub fn size(&self) -> usize {
        self.cell.len()
    }
}

//========
// Balance
//========

/// Structure used to keep track of the number of event in the simulation.
///
/// During the simulation, each time an event of interest occurs, the counters
/// are incremented accordingly. In a parallel context, this structure should be
/// operated on using atomic operations.
///
/// CANDIDATE FOR VECTORIZATION: use a `Vec<u64>` & write `add()` method in a way
/// that allow vectorization (no bound checks)
#[derive(Debug, Default, Clone)]
pub struct Balance {
    /// Number of particles absorbed.
    pub absorb: u64,
    /// Number of particles that enter census.
    pub census: u64,
    /// Number of particles that escape.
    pub escape: u64,
    /// Number of collisions.
    pub collision: u64,
    /// Number of particles at end of cycle.
    pub end: u64,
    /// Number of fission events.
    pub fission: u64,
    /// Number of particles created by collisions.
    pub produce: u64,
    /// Number of scatters.
    pub scatter: u64,
    /// Number of particles at beginning of cycle.
    pub start: u64,
    /// Number of particles sourced in.
    pub source: u64,
    /// Number of particles Russian Rouletted in population control.
    pub rr: u64,
    /// Number of particles split in population control.
    pub split: u64,
    /// Number of segements.
    pub num_segments: u64,
}

impl Balance {
    /// Reset fields to their default value i.e. `0`.
    pub fn reset(&mut self) {
        *self = Self::default(); // is the old value correctly dropped or just shadowed?
    }

    /// Add another [Balance]'s value to its own.
    pub fn add(&mut self, bal: &Balance) {
        self.absorb += bal.absorb;
        self.census += bal.census;
        self.escape += bal.escape;
        self.collision += bal.collision;
        self.end += bal.end;
        self.fission += bal.fission;
        self.produce += bal.produce;
        self.scatter += bal.scatter;
        self.start += bal.start;
        self.source += bal.source;
        self.rr += bal.rr;
        self.split += bal.split;
        self.num_segments += bal.num_segments;
    }
}

//=================
// Scalar flux data
//=================

/// Cell-specific scalar flux data.
///
/// Each element of the vector is corresponds to a cell's data.
type ScalarFluxCell<T> = Vec<T>;

/// Task-sorted _scalar-flux-data-holding_ sub-structure.
#[derive(Debug, Clone)]
pub struct ScalarFluxDomain<T: CustomFloat> {
    pub cell: Vec<ScalarFluxCell<T>>,
}

impl<T: CustomFloat> ScalarFluxDomain<T> {
    /// Constructor.
    pub fn new(domain: &MCDomain<T>, num_groups: usize) -> Self {
        // originally uses BulkStorage object for contiguous memory
        let cell = vec![vec![zero::<T>(); num_groups]; domain.cell_state.len()];
        Self { cell }
    }

    /// Reset fields to their default value i.e. `0`.
    pub fn reset(&mut self) {
        self.cell.iter_mut().for_each(|sf_cell| {
            sf_cell.fill(zero());
        });
    }

    /// Add another [ScalarFluxTask]'s value to its own.
    ///
    /// CANDIDATE FOR VECTORIZATION: Rewrite to avoid bound checks
    pub fn add(&mut self, scalar_flux_domain: &ScalarFluxDomain<T>) {
        let n_groups = self.cell[0].len();
        (0..self.cell.len()).for_each(|cell_idx| {
            (0..n_groups).for_each(|group_idx| {
                self.cell[cell_idx][group_idx] += scalar_flux_domain.cell[cell_idx][group_idx];
            })
        });
    }
}

//================
// Cell tally data
//================

/// Task-specific _cell-tallied-data-holding_ sub-structure.
#[derive(Debug, Default, Clone)]
pub struct CellTallyDomain<T: CustomFloat> {
    pub cell: Vec<T>,
}

impl<T: CustomFloat> CellTallyDomain<T> {
    /// Constructor.
    pub fn new(domain: &MCDomain<T>) -> Self {
        Self {
            cell: vec![zero(); domain.cell_state.len()],
        }
    }

    /// Reset fields to their default value i.e. 0.
    pub fn reset(&mut self) {
        self.cell = vec![zero(); self.cell.len()];
    }

    /// Add another [CellTallyTask]'s value to its own.
    ///
    /// CANDIDATE FOR VECTORIZATION: Rewrite to avoid bound checks
    pub fn add(&mut self, cell_tally_domain: &CellTallyDomain<T>) {
        (0..self.cell.len()).for_each(|ii| self.cell[ii] += cell_tally_domain.cell[ii]);
    }
}

//========
// Tallies
//========

/// Super-structure holding all recorded data besides time statistics.
#[derive(Debug)]
pub struct Tallies<T: CustomFloat> {
    /// Balance used for cumulative and centralized statistics.
    pub balance_cumulative: Balance,
    /// Task-specific cyclic balances.
    pub balance_cycle: Balance,
    /// Top-level structure holding scalar flux data.
    pub scalar_flux_domain: Vec<ScalarFluxDomain<T>>,
    /// Top-level structure holding cell tallied data.
    pub cell_tally_domain: Vec<CellTallyDomain<T>>,
    /// Top-level structure used to compute fluence data.
    pub fluence: Fluence<T>,
    /// Energy spectrum of the problem.
    pub spectrum: EnergySpectrum,
    /// Number of balance tallies for parallel processing. `1` means no replication.
    pub num_balance_replications: u32,
    /// Number of flux tallies for parallel processing. `1` means no replication.
    pub num_flux_replications: u32,
    /// Number of cell tallies for parallel processing. `1` means no replication.
    pub num_cell_tally_replications: u32,
}

impl<T: CustomFloat> Tallies<T> {
    /// Constructor.
    pub fn new(
        bal_rep: u32,
        flux_rep: u32,
        cell_rep: u32,
        spectrum_name: String,
        spectrum_size: usize,
    ) -> Self {
        let spectrum = EnergySpectrum::new(spectrum_name, spectrum_size);
        Self {
            balance_cumulative: Default::default(),
            balance_cycle: Default::default(),
            scalar_flux_domain: Default::default(),
            cell_tally_domain: Default::default(),
            fluence: Default::default(),
            spectrum,
            num_balance_replications: bal_rep,
            num_flux_replications: flux_rep,
            num_cell_tally_replications: cell_rep,
        }
    }

    /// Prepare the tallies for use.
    pub fn initialize_tallies(
        &mut self,
        domain: &[MCDomain<T>],
        num_energy_groups: usize,
        balance_replications: u32,
        flux_replications: u32,
        cell_replications: u32,
    ) {
        self.num_balance_replications = balance_replications;
        self.num_flux_replications = flux_replications;
        self.num_cell_tally_replications = cell_replications;

        // Initialize the cell tallies
        if self.cell_tally_domain.is_empty() {
            if self.cell_tally_domain.capacity() == 0 {
                self.cell_tally_domain.reserve(domain.len());
            }

            (0..domain.len()).for_each(|domain_idx| {
                self.cell_tally_domain
                    .push(CellTallyDomain::new(&domain[domain_idx]));
            });
        }

        // Initialize the scalar flux tallies
        if self.scalar_flux_domain.is_empty() {
            if self.scalar_flux_domain.capacity() == 0 {
                self.scalar_flux_domain.reserve(domain.len());
            }

            (0..domain.len()).for_each(|domain_idx| {
                self.scalar_flux_domain.push(ScalarFluxDomain::new(
                    &domain[domain_idx],
                    num_energy_groups,
                ));
            });
        }
    }

    /// Prints summarized data recorded by the tallies.
    ///
    /// TODO: add a model of the produced output
    pub fn print_summary(&self, mcco: &MonteCarlo<T>) {
        if mcco.time_info.cycle == 0 {
            // print header
            println!("[Tally Summary]");
            println!(
                "{:<7} | {:>8} {:>10} {:>10} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12} {:>13} {:>15} {:>18} {:>18}",
                "cycle", "start", "source", "rr", "split", "absorb", "scatter", "fission", "produce", "collision", 
                "escape", "census", "num_seg", "scalar_flux", "cycleInit (s)", "cycleTracking (s)", "cycleFinalize (s)"
            );
        }
        let cy_init = mc_fast_timer::get_last_cycle(mcco, Section::CycleInit);
        let cy_track = mc_fast_timer::get_last_cycle(mcco, Section::CycleTracking);
        let cy_fin = mc_fast_timer::get_last_cycle(mcco, Section::CycleFinalize);
        let sf_sum = self.scalar_flux_sum();
        let bal = &self.balance_cycle;
        println!("{:>7} | {:>8} {:>10} {:>10} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12}    {:.6e} {:>11e} {:>18e} {:>18e}",
            mcco.time_info.cycle,
            bal.start,
            bal.source,
            bal.rr,
            bal.split,
            bal.absorb,
            bal.scatter,
            bal.fission,
            bal.produce,
            bal.collision,
            bal.escape,
            bal.census,
            bal.num_segments,
            sf_sum,
            cy_init,
            cy_track,
            cy_fin,
        );
    }

    /// Computes the global scalar flux value of the problem.
    pub fn scalar_flux_sum(&self) -> T {
        let mut sum: T = zero();

        let n_domain = self.scalar_flux_domain.len();
        // for all domains
        (0..n_domain).for_each(|domain_idx| {
            let n_cells = self.scalar_flux_domain[domain_idx].cell.len();
            // for each cell
            (0..n_cells).for_each(|cell_idx| {
                let n_groups = self.scalar_flux_domain[domain_idx].cell[cell_idx].len();
                // for each energy group
                (0..n_groups).for_each(|group_idx| {
                    sum += self.scalar_flux_domain[domain_idx].cell[cell_idx][group_idx];
                })
            })
        });

        sum
    }

    /// Print stats of the current cycle and update the cumulative counters.
    pub fn cycle_finalize(&mut self, bench_type: BenchType) {
        self.balance_cumulative.add(&self.balance_cycle);

        let new_start: u64 = self.balance_cycle.end;
        self.balance_cycle.reset();
        self.balance_cycle.start = new_start;

        (0..self.scalar_flux_domain.len()).for_each(|domain_idx| {
            if bench_type != BenchType::Standard {
                self.fluence
                    .compute(domain_idx, &self.scalar_flux_domain[domain_idx]);
            }
            self.cell_tally_domain[domain_idx].reset();
            self.scalar_flux_domain[domain_idx].reset();
        });
    }
}
