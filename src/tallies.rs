use std::cell::Cell;

use crate::{energy_spectrum::EnergySpectrum, bulk_storage::BulkStorage, mc::mc_domain::MCDomain, montecarlo::MonteCarlo};

/// Enum representing a tally event.
#[derive(Debug)]
pub enum MCTallyEvent {
    Collision,
    FacetCrossingTransitExit,
    Census,
    FacetCrossingTrackingError,
    FacetCrossingEscape,
    FacetCrossingReflection,
    FacetCrossingCommunication,
}

/// May need to change it to a full-fledged structure later.
pub type Fluence = Vec<FluenceDomain>;

/// Structure used to regulate the number of event in the simulation.
#[derive(Debug)]
pub struct Balance {
    /// Number of particles absorbed
    pub absorb: u64,
    /// Number of particles that enter census
    pub census: u64,
    /// Number of particles that escape
    pub escape: u64,
    /// Number of collisions
    pub collision: u64,
    /// Number of particles at end of cycle
    pub end: u64,
    /// Number of fission events
    pub fission: u64,
    /// Number of particles created by collisions
    pub produce: u64,
    /// Number of scatters
    pub scatter: u64,
    /// Number of particles at beginning of cycle
    pub start: u64,
    /// Number of particles sourced in
    pub source: u64,
    /// Number of particles Russian Rouletted in population control
    pub rr: u64,
    /// Number of particles split in population control
    pub split: u64,
    /// Number of segements
    pub num_segments: u64,
}

impl Balance {
    /// Reset fields to their default value i.e. 0.
    pub fn reset(&mut self) {
        todo!()
    }

    /// Add another [Balance]'s value to its own. Replace by an overload? 
    pub fn add(&mut self, bal: &Balance) {
        todo!()
    }
}

/// May need to change it to a full-fledged structure later.
type ScalarFluxCell = Vec<f64>;

/// ?
#[derive(Debug)]
pub struct CellTallyTask {
    pub cell: Vec<f64>,
}

impl CellTallyTask {
    /// Constructor
    pub fn new(domain: &MCDomain) -> Self {
        todo!()
    }

    /// Reset fields to their default value i.e. 0.
    pub fn reset(&mut self) {
        todo!()
    }
    
    /// Add another [CellTallyTask]'s value to its own. Replace by an overload? 
    pub fn add(&mut self, cell_tally_task: &CellTallyTask) {
        todo!()
    }
}

/// ?
#[derive(Debug)]
pub struct ScalarFluxTask {
    pub cell: Vec<ScalarFluxCell>,
    pub scalar_flux_cell_storage: BulkStorage<f64>,
}

impl ScalarFluxTask {
    /// Constructor
    pub fn new(domain: &MCDomain, num_groups: u32) -> Self {
        todo!()
    }

    /// Reset fields to their default value i.e. 0.
    pub fn reset(&mut self) {
        todo!()
    }

    /// Add another [ScalarFluxTask]'s value to its own. Replace by an overload? 
    pub fn add(&mut self, scalar_flux_task: &ScalarFluxTask) {
        todo!()
    }
}

/// ?
#[derive(Debug)]
pub struct CellTallyDomain {
    pub task: Vec<CellTallyTask>,
}

impl CellTallyDomain {
    /// Constructor
    pub fn new(domain: &MCDomain, cell_tally_replications: u32) -> Self {
        todo!()
    }
}

/// ?
#[derive(Debug)]
pub struct ScalarFluxDomain {
    pub task: Vec<ScalarFluxTask>,
}

impl ScalarFluxDomain {
    // Constructor
    pub fn new(domain: &MCDomain,num_groups: u32, flux_replications: u32) -> Self {
        todo!()
    }
}

/// ?
#[derive(Debug)]
pub struct FluenceDomain {
    pub cell: Vec<f64>,
}

impl FluenceDomain {
    pub fn add_cell(&mut self, index: usize, val: f64) {
        todo!()
    }

    pub fn get_cell(&self, index:usize) -> f64 {
        todo!()
    }

    pub fn size(&self) -> usize {
        todo!()
    }
}

/// Structure used as tallies.
#[derive(Debug)]
pub struct Tallies {
    pub balance_cumulative: Balance,
    pub balance_task: Vec<Balance>,
    pub scalar_flux_domain: Vec<CellTallyDomain>,
    pub fluence: Fluence,
    pub spectrum: EnergySpectrum,
    num_balance_replications: u32,
    num_flux_replications: u32,
    num_cell_tally_replications: u32,
}

impl Tallies {
    /// Constructor.
    pub fn new(bal_rep: u32, flux_rep: u32, cell_rep: u32, spectrum_name: String, spectrum_size: u64) -> Self {
        todo!()
    }

    /// Getter.
    pub fn get_num_balance_replications(&self) -> u32 {
        todo!()
    }
    /// Getter.
    pub fn get_num_flux_replications(&self) -> u32 {
        todo!()
    }
    /// Getter.
    pub fn get_num_cell_tally_replications(&self) -> u32 {
        todo!()
    }

    pub fn initialize_tallies(&mut self, monte_carlo: &MonteCarlo, balance_replications: u32, flux_replications: u32, cell_replications: u32) {
        todo!()
    }

    pub fn cycle_initialize(&mut self, mcco: &MonteCarlo) {
        todo!()
    }

    pub fn sum_tasks(&mut self) {
        todo!()
    }

    pub fn cycle_finalize(&mut self, mcco: &MonteCarlo) {
        todo!()
    }

    pub fn print_summary(&self, mcco: &MonteCarlo) {
        todo!()
    }

    pub fn tally_scalar_flux(&mut self, value: f64, domain: usize, task: usize, cell: usize, group: usize) {
        todo!()
    }

    pub fn tally_cell_value(&mut self, value: f64, domain: usize, task: usize, cell: usize) {
        todo!()
    }

    pub fn scalar_flux_sum(mcco: &MonteCarlo) -> f64 {
        todo!()
    }
}
