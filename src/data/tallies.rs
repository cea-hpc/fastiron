//! Code for simulation-related statistics
//!
//! This module contains all code used to record and count events happening
//! during the simulation. The cyclic summary is printed using the recorded data.
//!
//! Note that this module isn't used to compute time-related data, this is done in
//! the [utils::mc_fast_timer][crate::utils::mc_fast_timer] module.

use std::{fmt::Debug, iter::zip};

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
    /// Value for a collision event.
    Collision,
    /// Value for a facet crossing event resulting in a cell exit.
    /// Value for a facet crossing event resulting in a cell exit.
    FacetCrossingTransitExit,
    /// Value for a census event.
    /// Value for a census event.
    #[default]
    Census,
    /// Value for a facet crossing event resulting in an escape from the problem.
    FacetCrossingEscape,
    /// Value for a facet crossing event resulting in a reflection on the facet.
    /// Value for a facet crossing event resulting in a reflection on the facet.
    FacetCrossingReflection,
    /// Value for a facet crossing event resulting in a cell exit to an
    /// off-processor cell.
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

/// Domain-sorted fluence-data-holding sub-structure.
#[derive(Debug, Default)]
pub struct FluenceDomain<T: CustomFloat> {
    pub cell: Vec<T>,
}

impl<T: CustomFloat> FluenceDomain<T> {
    pub fn compute(&mut self, scalar_flux_domain: &ScalarFluxDomain<T>) {
        let cell_iter = zip(self.cell.iter_mut(), scalar_flux_domain.cell.iter());
        cell_iter.for_each(|(fl_cell, sf_cell)| {
            *fl_cell += sf_cell.iter().copied().sum();
        })
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
#[derive(Debug, Default, Clone)]
pub struct Balance {
    /// Number of particles absorbed.
    /// Number of particles absorbed.
    pub absorb: u64,
    /// Number of particles that enter census.
    /// Number of particles that enter census.
    pub census: u64,
    /// Number of particles that escape.
    /// Number of particles that escape.
    pub escape: u64,
    /// Number of collisions.
    /// Number of collisions.
    pub collision: u64,
    /// Number of particles at end of cycle.
    /// Number of particles at end of cycle.
    pub end: u64,
    /// Number of fission events.
    /// Number of fission events.
    pub fission: u64,
    /// Number of particles created by collisions.
    /// Number of particles created by collisions.
    pub produce: u64,
    /// Number of scatters.
    /// Number of scatters.
    pub scatter: u64,
    /// Number of particles at beginning of cycle.
    /// Number of particles at beginning of cycle.
    pub start: u64,
    /// Number of particles sourced in.
    /// Number of particles sourced in.
    pub source: u64,
    /// Number of particles Russian Rouletted in population control.
    /// Number of particles Russian Rouletted in population control.
    pub rr: u64,
    /// Number of particles split in population control.
    /// Number of particles split in population control.
    pub split: u64,
    /// Number of segements.
    /// Number of segements.
    pub num_segments: u64,
}

impl Balance {
    /// Reset fields to their default value i.e. `0`.
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
//=================
// Scalar flux data
//=================

/// Cell-specific scalar flux data.
///
/// Each element of the vector is corresponds to a cell's data.
type ScalarFluxCell<T> = Vec<T>;

/// Domain-sorted _scalar-flux-data-holding_ sub-structure.
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
    /// Reset fields to their default value i.e. `0`.
    pub fn reset(&mut self) {
        self.cell.iter_mut().for_each(|sf_cell| {
            sf_cell.fill(zero());
        });
    }

    /// Add another [ScalarFluxDomain]'s value to its own.
    pub fn add(&mut self, other: &ScalarFluxDomain<T>) {
        // zip iterators from the two objects' values.
        let cell_iter = zip(self.cell.iter_mut(), other.cell.iter());
        cell_iter.for_each(|(cell_lhs, cell_rhs)| {
            // zip iterators from the two objects' values.
            let group_iter = zip(cell_lhs.iter_mut(), cell_rhs.iter());
            // sum other to self
            group_iter.for_each(|(group_lhs, group_rhs)| *group_lhs += *group_rhs);
        })
    }
}

//================
// Cell tally data
//================

/// Domain-specific _cell-tallied-data-holding_ sub-structure.
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

    /// Add another [CellTallyDomain]'s value to its own.
    pub fn add(&mut self, other: &CellTallyDomain<T>) {
        // zip iterators from the two objects' values.
        let iter = zip(self.cell.iter_mut(), other.cell.iter());
        // sum other to self
        iter.for_each(|(lhs, rhs)| *lhs += *rhs);
    }
}

//========
// Tallies
//========

/// Super-structure holding all recorded data besides time statistics.
//========
// Tallies
//========

/// Super-structure holding all recorded data besides time statistics.
#[derive(Debug)]
pub struct Tallies<T: CustomFloat> {
    /// Balance used for cumulative and centralized statistics.
    /// Balance used for cumulative and centralized statistics.
    pub balance_cumulative: Balance,
    /// Cyclic balances.
    pub balance_cycle: Balance,
    /// Top-level structure holding scalar flux data.
    pub scalar_flux_domain: Vec<ScalarFluxDomain<T>>,
    /// Top-level structure holding cell tallied data.
    /// Top-level structure holding cell tallied data.
    pub cell_tally_domain: Vec<CellTallyDomain<T>>,
    /// Top-level structure used to compute fluence data.
    /// Top-level structure used to compute fluence data.
    pub fluence: Fluence<T>,
    /// Energy spectrum of the problem.
    /// Energy spectrum of the problem.
    pub spectrum: EnergySpectrum,
}

impl<T: CustomFloat> Tallies<T> {
    /// Constructor.
    pub fn new(spectrum_name: String, spectrum_size: usize) -> Self {
        let spectrum = EnergySpectrum::new(spectrum_name, spectrum_size);
        Self {
            balance_cumulative: Default::default(),
            balance_cycle: Default::default(),
            scalar_flux_domain: Default::default(),
            cell_tally_domain: Default::default(),
            fluence: Default::default(),
            spectrum,
        }
    }

    /// Prepare the tallies for use.
    pub fn initialize_tallies(
        &mut self,
        domain: &[MCDomain<T>],
        num_energy_groups: usize,
        bench_type: BenchType,
    ) {
        self.cell_tally_domain.reserve(domain.len());
        self.scalar_flux_domain.reserve(domain.len());
        domain.iter().for_each(|dom| {
            // Initialize the cell tallies
            self.cell_tally_domain.push(CellTallyDomain::new(dom));
            // Initialize the scalar flux tallies
            self.scalar_flux_domain
                .push(ScalarFluxDomain::new(dom, num_energy_groups));
        });

        // Initialize Fluence if necessary
        if bench_type != BenchType::Standard {
            self.scalar_flux_domain
                .iter()
                .map(|dom| dom.cell.len())
                .for_each(|n_cells| {
                    self.fluence.domain.push(FluenceDomain {
                        cell: vec![zero(); n_cells],
                    })
                });
        }
    }

    /// Prints summarized data recorded by the tallies.
    ///
    /// This function prints the number of recorded events & additionnal data
    /// at each cycle of the simulation. After five cycle, the printed output
    /// would look like the following:
    ///
    /// ```shell
    /// cycle   |    start     source         rr        split       absorb      scatter      fission      produce    collision       escape       census      num_seg   scalar_flux   cycleInit (s)  cycleTracking (s)  cycleFinalize (s)
    ///       0 |        0      10000          0        90000        97237       711673        86904        86904       895814            0         2763      2245733    8.984036e11   1.4745e-2         1.088409e0                0e0
    ///       1 |     2763      10000          0        87202        97576       715193        86951        86951       899720            0         2389      2250433    9.191721e11   1.1338e-2         1.114024e0                0e0
    ///       2 |     2389      10000          0        87625        97569       717781        87733        87733       903083            0         2445      2262159    9.303649e11     8.45e-3         1.125107e0                0e0
    ///       3 |     2445      10000       1468        87569        96180       704095        85839        85839       886114            0         2366      2217454    9.227719e11    9.499e-3         1.106859e0                0e0
    ///       4 |     2366      10000        331        87599        97132       716577        87708        87708       901417            0         2502      2256889    9.255832e11    9.701e-3         1.129408e0                0e0
    /// ```
    ///
    /// - `cycle` column gives the cycle number.
    /// - `start` column gives the number of particle at the start of the cycle,
    ///   before population control algorithms.
    /// - `source`, `rr`, `split` columns count
    ///   [`population_control`][crate::simulation::population_control] events.
    /// - `absorb`, `scatter`, `fission`, `produce`, `collision` columns count
    ///   [collision_event][crate::simulation::collision_event] events.
    /// - `escape`, `census` columns count the remaining possible
    ///   [`outcomes`][crate::simulation::mc_segment_outcome].
    /// - `num_seg` column counts the total number of computed segments.
    /// - `scalar_flux` is the total scalar flux of the problem.
    /// - The last three columns indicate the time spent in each section.
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
        println!("{:>7} | {:>8} {:>10} {:>10} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12}    {:.6e} {:>11.3e} {:>18.5e} {:>18.3e}",
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
        let summ: T = self
            .scalar_flux_domain
            .iter()
            .map(|sf_domain| {
                sf_domain
                    .cell
                    .iter()
                    .map(|sf_cell| sf_cell.iter().copied().sum())
                    .sum()
            })
            .sum();
        summ
    }

    /// Print stats of the current cycle and update the cumulative counters.
    pub fn cycle_finalize(&mut self, bench_type: BenchType) {
        self.balance_cumulative.add(&self.balance_cycle);

        let new_start: u64 = self.balance_cycle.end;
        self.balance_cycle.reset();
        self.balance_cycle.start = new_start;

        if bench_type != BenchType::Standard {
            let fluence_computation_iter = zip(
                self.fluence.domain.iter_mut(),
                self.scalar_flux_domain.iter(),
            );
            fluence_computation_iter.for_each(|(fl_domain, sf_domain)| {
                fl_domain.compute(sf_domain);
            })
        }

        let dom_iter = zip(
            self.cell_tally_domain.iter_mut(),
            self.scalar_flux_domain.iter_mut(),
        );
        dom_iter.for_each(|(ct_domain, sf_domain)| {
            ct_domain.reset();
            sf_domain.reset();
        });
    }
}
