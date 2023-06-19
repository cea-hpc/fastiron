//! Code for simulation-related statistics
//!
//! This module contains all code used to record and count events happening
//! during the simulation. The cyclic summary is printed using the recorded data.
//!
//! Note that this module isn't used to compute time-related data, this is done in
//! the [utils::mc_fast_timer][crate::utils::mc_fast_timer] module.

use std::{
    fmt::Debug,
    fs::OpenOptions,
    io::Write,
    iter::zip,
    ops::{Index, IndexMut},
    sync::atomic::Ordering,
};

use atomic::Atomic;
use num::zero;

use crate::{
    constants::CustomFloat,
    utils::mc_fast_timer::{self, MCFastTimerContainer, Section},
};

/// Enum representing a tally event.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Default)]
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

/// Domain-sorted fluence-data-holding sub-structure.
#[derive(Debug, Default)]
pub struct FluenceDomain<T: CustomFloat> {
    pub cell: Vec<T>,
}

impl<T: CustomFloat> FluenceDomain<T> {
    pub fn compute(&mut self, scalar_flux_domain: &ScalarFluxDomain<T>) {
        let cell_iter = zip(
            self.cell.iter_mut(),
            scalar_flux_domain
                .cell
                .chunks(scalar_flux_domain.num_groups),
        );
        cell_iter.for_each(|(fl_cell, sf_cell)| {
            let sum = sf_cell.iter().map(|val| val.load(Ordering::Relaxed)).sum();
            *fl_cell += sum;
        })
    }

    pub fn size(&self) -> usize {
        self.cell.len()
    }
}

//========
// Balance
//========

pub const N_TALLIED_EVENT: usize = 14;

#[derive(Debug)]
pub enum TalliedEvent {
    Absorb,
    Census,
    Escape,
    Collision,
    End,
    Fission,
    Produce,
    Scatter,
    Start,
    Source,
    OverRr,
    WeightRr,
    Split,
    NumSegments,
}

/// Structure used to keep track of the number of event in the simulation.
///
/// During the simulation, each time an event of interest occurs, the counters
/// are incremented accordingly. In a parallel context, this structure should be
/// operated on using atomic operations.
#[derive(Debug, Default, Clone, Copy)]
pub struct Balance {
    /// Array used to store tallied event. See [TalliedEvent] for more information.
    pub data: [u64; N_TALLIED_EVENT],
}

impl Balance {
    /// Reset fields to their default value i.e. `0`.
    pub fn reset(&mut self) {
        self.data.fill(0_u64);
    }

    /// Add another [Balance]'s value to its own.
    pub fn add_to_self(&mut self, bal: &Balance) {
        self.data
            .iter_mut()
            .zip(bal.data.iter())
            .for_each(|(lhs, rhs)| *lhs += *rhs);
    }
}

// Indexing operators

impl Index<TalliedEvent> for Balance {
    type Output = u64;

    fn index(&self, index: TalliedEvent) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl IndexMut<TalliedEvent> for Balance {
    fn index_mut(&mut self, index: TalliedEvent) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}

// Add op (useful when folding)

impl std::ops::Add<Balance> for Balance {
    type Output = Balance;

    fn add(self, rhs: Balance) -> Self::Output {
        let mut tmp = self;
        tmp.add_to_self(&rhs);
        tmp
    }
}

impl std::iter::Sum<Balance> for Balance {
    fn sum<I: Iterator<Item = Balance>>(iter: I) -> Self {
        iter.fold(Self::default(), |b1, b2| b1 + b2)
    }
}

//=================
// Scalar flux data
//=================

/// Domain-sorted _scalar-flux-data-holding_ sub-structure.
#[derive(Debug, Default)]
pub struct ScalarFluxDomain<T: CustomFloat> {
    pub num_groups: usize,
    pub cell: Vec<Atomic<T>>,
}

impl<T: CustomFloat> ScalarFluxDomain<T> {
    /// Constructor.
    pub fn new(n_cells: usize, num_groups: usize) -> Self {
        // originally uses BulkStorage object for contiguous memory
        let cell = (0..n_cells * num_groups)
            .map(|_| Atomic::new(zero()))
            .collect();
        Self { num_groups, cell }
    }

    /// Reset fields to their default value i.e. `0`.
    pub fn reset(&mut self) {
        self.cell
            .iter_mut()
            .for_each(|elem| elem.store(zero(), Ordering::Relaxed))
    }
}

// maybe make theses accesses unchecked?
impl<T: CustomFloat> Index<(usize, usize)> for ScalarFluxDomain<T> {
    type Output = Atomic<T>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.cell[index.0 * self.num_groups + index.1]
    }
}

impl<T: CustomFloat> IndexMut<(usize, usize)> for ScalarFluxDomain<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.cell[index.0 * self.num_groups + index.1]
    }
}

//========
// Tallies
//========

/// Super-structure holding all recorded data besides time statistics.
#[derive(Debug, Default)]
pub struct Tallies<T: CustomFloat> {
    /// Cyclic balances.
    pub balance_cycle: Balance,
    /// Top-level structure holding scalar flux data.
    pub scalar_flux_domain: ScalarFluxDomain<T>,
}

impl<T: CustomFloat> Tallies<T> {
    /// Prepare the tallies for use.
    pub fn initialize_tallies(&mut self, n_cells: usize, num_energy_groups: usize) {
        self.scalar_flux_domain = ScalarFluxDomain::new(n_cells, num_energy_groups);
    }

    /// Prints summarized data recorded by the tallies.
    ///
    /// This function prints the number of recorded events & additionnal data
    /// at each cycle of the simulation. After five cycle, the printed output
    /// would look like the following:
    ///
    /// ```shell
    /// cycle   |  start |   source |       rr |      split |     absorb |    scatter |    fission |    produce |  collision |     escape |     census |    num_seg |   scalar_flux | ppControl (s) | cycleTracking (s) | cycleSync (s)
    ///       0 |      0 |    10000 |        0 |      90000 |      97237 |     711673 |      86904 |      86904 |     895814 |          0 |       2763 |    2245733 |   8.984036e11 |  1.757e-2     |     1.01038e0     |  2.200e-4
    ///       1 |   2763 |    10000 |        0 |      87202 |      97576 |     715193 |      86951 |      86951 |     899720 |          0 |       2389 |    2250433 |   9.191721e11 |  1.337e-2     |     1.02491e0     |  7.382e-4
    ///       2 |   2389 |    10000 |        0 |      87625 |      97569 |     717781 |      87733 |      87733 |     903083 |          0 |       2445 |    2262159 |   9.303649e11 |  9.174e-3     |     1.03161e0     |  7.335e-4
    ///       3 |   2445 |    10000 |     1468 |      87569 |      96180 |     704095 |      85839 |      85839 |     886114 |          0 |       2366 |    2217454 |   9.227719e11 |  1.119e-2     |     1.01220e0     |  6.674e-4
    ///       4 |   2366 |    10000 |      331 |      87599 |      97132 |     716577 |      87708 |      87708 |     901417 |          0 |       2502 |    2256889 |   9.255832e11 |  1.124e-2     |     1.02922e0     |  6.787e-4
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
    pub fn print_summary(
        &self,
        timer_container: &mut MCFastTimerContainer,
        step: usize,
        csv: bool,
    ) {
        if step == 0 {
            // print header
            println!("[Tally Summary]");
            println!(
                "{:<7} | {:>8} {:>10} {:>10} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12} {:>15} {:>13} {:>19} {:>13}",
                "cycle", "start |", "source |", "rr |", "split |", "absorb |", "scatter |", "fission |", "produce |", "collision |", 
                "escape |", "census |", "num_seg |", "scalar_flux |", "ppControl (s) |", "cycleTracking (s) |", "cycleSync (s)"
            );
            if csv {
                // write column name
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open("tallies_report.csv")
                    .unwrap();
                writeln!(file, "cycle;start;source;rr;split;absorb;scatter;fission;produce;collision;escape;census;num_seg;scalar_flux;ppControl(s);cycleTracking(s);cycleSync(s)").unwrap();
            }
        }
        let cy_init = mc_fast_timer::get_last_cycle(timer_container, Section::PopulationControl);
        let cy_track = mc_fast_timer::get_last_cycle(timer_container, Section::CycleTracking);
        let cy_fin = mc_fast_timer::get_last_cycle(timer_container, Section::CycleSync);
        let sf_sum = self.scalar_flux_sum();
        let bal = &self.balance_cycle;
        println!("{:>7} |{:>7} |{:>9} |{:>9} |{:>11} |{:>11} |{:>11} |{:>11} |{:>11} |{:>11} |{:>11} |{:>11} |{:>11} |{:>14.6e} |{:>10.3e}     |{:>14.5e}     |{:>10.3e}",
            step,
            bal[TalliedEvent::Start],
            bal[TalliedEvent::Source],
            bal[TalliedEvent::OverRr] + bal[TalliedEvent::WeightRr],
            bal[TalliedEvent::Split],
            bal[TalliedEvent::Absorb],
            bal[TalliedEvent::Scatter],
            bal[TalliedEvent::Fission],
            bal[TalliedEvent::Produce],
            bal[TalliedEvent::Collision],
            bal[TalliedEvent::Escape],
            bal[TalliedEvent::Census],
            bal[TalliedEvent::NumSegments],
            sf_sum,
            cy_init,
            cy_track,
            cy_fin,
        );

        if csv {
            let mut file = OpenOptions::new()
                .append(true)
                .open("tallies_report.csv")
                .unwrap();
            writeln!(
                file,
                "{};{};{};{};{};{};{};{};{};{};{};{};{};{:e};{:e};{:e};{:e}",
                step,
                bal[TalliedEvent::Start],
                bal[TalliedEvent::Source],
                bal[TalliedEvent::OverRr] + bal[TalliedEvent::WeightRr],
                bal[TalliedEvent::Split],
                bal[TalliedEvent::Absorb],
                bal[TalliedEvent::Scatter],
                bal[TalliedEvent::Fission],
                bal[TalliedEvent::Produce],
                bal[TalliedEvent::Collision],
                bal[TalliedEvent::Escape],
                bal[TalliedEvent::Census],
                bal[TalliedEvent::NumSegments],
                sf_sum,
                cy_init,
                cy_track,
                cy_fin,
            )
            .unwrap();
        }
    }

    /// Computes the global scalar flux value of the problem.
    pub fn scalar_flux_sum(&self) -> T {
        self.scalar_flux_domain
            .cell
            .iter()
            .map(|sf_cell| sf_cell.load(Ordering::Relaxed))
            .sum::<T>()
    }

    /// Print stats of the current cycle and update the cumulative counters.
    pub fn cycle_finalize(&mut self) {
        let new_start: u64 = self.balance_cycle[TalliedEvent::End];
        self.balance_cycle.reset();
        self.balance_cycle[TalliedEvent::Start] = new_start;

        self.scalar_flux_domain.reset();
    }
}
