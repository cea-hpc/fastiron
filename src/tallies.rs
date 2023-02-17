use crate::{energy_spectrum::EnergySpectrum, bulk_storage::BulkStorage};

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

/// May need to change it to a full-fledged structure later.
type ScalarFluxCell = Vec<f64>;

/// ?
#[derive(Debug)]
pub struct CellTallyTask {
    pub cell: Vec<f64>,
}

/// ?
#[derive(Debug)]
pub struct ScalarFluxTask {
    pub cell: Vec<ScalarFluxCell>,
    pub scalar_flux_cell_storage: BulkStorage<f64>,
}

/// ?
#[derive(Debug)]
pub struct CellTallyDomain {
    pub task: Vec<CellTallyTask>,
}

/// ?
#[derive(Debug)]
pub struct ScalarFluxDomain {
    pub task: Vec<ScalarFluxTask>,
}

/// ?
#[derive(Debug)]
pub struct FluenceDomain {
    pub cell: Vec<f64>,
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
