use num::{zero, Float};

use crate::{
    bulk_storage::BulkStorage, energy_spectrum::EnergySpectrum, mc::mc_domain::MCDomain,
    montecarlo::MonteCarlo,
};

/// Enum representing a tally event.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MCTallyEvent {
    Collision,
    FacetCrossingTransitExit,
    Census,
    FacetCrossingTrackingError,
    FacetCrossingEscape,
    FacetCrossingReflection,
    FacetCrossingCommunication,
}

impl Default for MCTallyEvent {
    fn default() -> Self {
        Self::Census
    }
}

/// May need to change it to a full-fledged structure later.
pub type Fluence<T> = Vec<FluenceDomain<T>>;

/// Structure used to regulate the number of event in the simulation.
#[derive(Debug, Default)]
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
        *self = Self::default(); // is the old value correctly dropped or just shadowed?
    }

    /// Add another [Balance]'s value to its own. Replace by an overload?
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

/// May need to change it to a full-fledged structure later.
type ScalarFluxCell<T> = Vec<T>;

/// ?
#[derive(Debug, Default)]
pub struct CellTallyTask<T: Float> {
    pub cell: Vec<T>,
}

impl<T: Float> CellTallyTask<T> {
    /// Constructor
    pub fn new(domain: &MCDomain<T>) -> Self {
        Self {
            cell: vec![zero(); domain.cell_state.len()],
        }
    }

    /// Reset fields to their default value i.e. 0.
    pub fn reset(&mut self) {
        self.cell.clear(); // no effect on allocated capacity
    }

    /// Add another [CellTallyTask]'s value to its own. Replace by an overload?
    pub fn add(&mut self, cell_tally_task: &CellTallyTask<T>) {
        //assert_eq!(self.cell.len(), cell_tally_task.cell.len());
        (0..self.cell.len())
            .into_iter()
            .for_each(|ii| self.cell[ii] = self.cell[ii] + cell_tally_task.cell[ii]);
    }
}

/// ?
#[derive(Debug)]
pub struct ScalarFluxTask<T: Float> {
    pub cell: Vec<ScalarFluxCell<T>>,
}

impl<T: Float> ScalarFluxTask<T> {
    /// Constructor
    pub fn new(domain: &MCDomain<T>, num_groups: usize) -> Self {
        let mut cell = Vec::with_capacity(domain.cell_state.len());

        // originally uses BulkStorage object for contiguous memory
        (0..domain.cell_state.len())
            .into_iter()
            .for_each(|_| cell.push(Vec::with_capacity(num_groups)));

        Self { cell }
    }

    /// Reset fields to their default value i.e. 0.
    pub fn reset(&mut self) {
        self.cell.clear();
    }

    /// Add another [ScalarFluxTask]'s value to its own. Replace by an overload?
    pub fn add(&mut self, scalar_flux_task: &ScalarFluxTask<T>) {
        let n_groups = self.cell[0].len();
        (0..self.cell.len()).into_iter().for_each(|cell_idx| {
            (0..n_groups).into_iter().for_each(|group_idx| {
                self.cell[cell_idx][group_idx] =
                    self.cell[cell_idx][group_idx] + scalar_flux_task.cell[cell_idx][group_idx];
            })
        });
    }
}

/// ?
#[derive(Debug)]
pub struct CellTallyDomain<T: Float> {
    pub task: Vec<CellTallyTask<T>>,
}

impl<T: Float> CellTallyDomain<T> {
    /// Constructor
    pub fn new(domain: &MCDomain<T>, cell_tally_replications: usize) -> Self {
        let mut task = Vec::with_capacity(cell_tally_replications);
        (0..cell_tally_replications)
            .into_iter()
            .for_each(|_| task.push(CellTallyTask::new(domain)));
        Self { task }
    }
}

/// ?
#[derive(Debug)]
pub struct ScalarFluxDomain<T: Float> {
    pub task: Vec<ScalarFluxTask<T>>,
}

impl<T: Float> ScalarFluxDomain<T> {
    // Constructor
    pub fn new(domain: &MCDomain<T>, num_groups: usize, flux_replications: usize) -> Self {
        let mut task = Vec::with_capacity(flux_replications);
        (0..flux_replications)
            .into_iter()
            .for_each(|_| task.push(ScalarFluxTask::new(domain, num_groups)));
        Self { task }
    }
}

/// ?
#[derive(Debug)]
pub struct FluenceDomain<T: Float> {
    pub cell: Vec<T>,
}

impl<T: Float> FluenceDomain<T> {
    pub fn add_cell(&mut self, index: usize, val: T) {
        self.cell[index] = self.cell[index] + val;
    }

    pub fn get_cell(&self, index: usize) -> T {
        self.cell[index]
    }

    pub fn size(&self) -> usize {
        self.cell.len()
    }
}

/// Structure used as tallies.
#[derive(Debug)]
pub struct Tallies<T: Float> {
    pub balance_cumulative: Balance,
    pub balance_task: Vec<Balance>,
    pub scalar_flux_domain: Vec<ScalarFluxDomain<T>>,
    pub cell_tally_domain: Vec<CellTallyDomain<T>>,
    pub fluence: Fluence<T>,
    pub spectrum: EnergySpectrum<T>,
    pub num_balance_replications: u32,
    pub num_flux_replications: u32,
    pub num_cell_tally_replications: u32,
}

impl<T: Float> Tallies<T> {
    /// Constructor.
    pub fn new(
        bal_rep: u32,
        flux_rep: u32,
        cell_rep: u32,
        spectrum_name: String,
        spectrum_size: u64,
    ) -> Self {
        todo!()
    }

    /// Prepare the tallies for use.
    pub fn initialize_tallies(
        &mut self,
        monte_carlo: &MonteCarlo<T>,
        balance_replications: u32,
        flux_replications: u32,
        cell_replications: u32,
    ) {
        todo!()
    }

    /*
    pub fn cycle_initialize(&mut self, mcco: &MonteCarlo<T>) {
        todo!()
    }
    */

    /// Sums the task-level data. This is used when replications
    /// is active.
    pub fn sum_tasks(&mut self) {
        todo!()
    }

    /// End-of-simulation routine that updates its own data and other structures'.
    pub fn cycle_finalize(&mut self, mcco: &MonteCarlo<T>) {
        todo!()
    }

    /// Prints summarized data recorded by the tallies.
    pub fn print_summary(&self, mcco: &MonteCarlo<T>) {
        todo!()
    }

    /// Atomic add?
    pub fn tally_scalar_flux(
        &mut self,
        value: T,
        domain: usize,
        task: usize,
        cell: usize,
        group: usize,
    ) {
        todo!()
    }

    /// Atomic add?
    pub fn tally_cell_value(&mut self, value: T, domain: usize, task: usize, cell: usize) {
        todo!()
    }

    /// Sums above all ?
    pub fn scalar_flux_sum(mcco: &MonteCarlo<T>) -> T {
        todo!()
    }
}
