use std::fmt::Debug;

use num::zero;

use crate::{
    constants::CustomFloat,
    energy_spectrum::EnergySpectrum,
    mc::{
        mc_domain::MCDomain,
        mc_fast_timer::{self, Section},
    },
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
#[derive(Debug, Default)]
pub struct Fluence<T: CustomFloat> {
    pub domain: Vec<FluenceDomain<T>>,
}

impl<T: CustomFloat> Fluence<T> {
    pub fn compute(&mut self, domain_idx: usize, scalar_flux_domain: &ScalarFluxDomain<T>) {
        let n_cells = scalar_flux_domain.task[0].cell.len();
        while self.domain.len() <= domain_idx {
            let new_domain: FluenceDomain<T> = FluenceDomain {
                cell: Vec::with_capacity(n_cells),
            };
            self.domain.push(new_domain);
        }
        (0..n_cells).for_each(|cell_idx| {
            let n_groups = scalar_flux_domain.task[0].cell[cell_idx].len();
            (0..n_groups).for_each(|group_idx| {
                self.domain[domain_idx].add_cell(
                    cell_idx,
                    scalar_flux_domain.task[0].cell[cell_idx][group_idx],
                );
            });
        });
    }
}

/// Structure used to regulate the number of event in the simulation.
#[derive(Debug, Default, Clone)]
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
#[derive(Debug, Default, Clone)]
pub struct CellTallyTask<T: CustomFloat> {
    pub cell: Vec<T>,
}

impl<T: CustomFloat> CellTallyTask<T> {
    /// Constructor
    pub fn new(domain: &MCDomain<T>) -> Self {
        Self {
            cell: vec![zero(); domain.cell_state.len()],
        }
    }

    /// Reset fields to their default value i.e. 0.
    pub fn reset(&mut self) {
        self.cell = vec![zero(); self.cell.len()]; // no effect on allocated capacity
    }

    /// Add another [CellTallyTask]'s value to its own. Replace by an overload?
    pub fn add(&mut self, cell_tally_task: &CellTallyTask<T>) {
        //assert_eq!(self.cell.len(), cell_tally_task.cell.len());
        (0..self.cell.len()).for_each(|ii| self.cell[ii] += cell_tally_task.cell[ii]);
    }
}

/// ?
#[derive(Debug, Clone)]
pub struct ScalarFluxTask<T: CustomFloat> {
    pub cell: Vec<ScalarFluxCell<T>>,
}

impl<T: CustomFloat> ScalarFluxTask<T> {
    /// Constructor
    pub fn new(domain: &MCDomain<T>, num_groups: usize) -> Self {
        // originally uses BulkStorage object for contiguous memory
        let cell = vec![vec![zero::<T>(); num_groups]; domain.cell_state.len()];
        Self { cell }
    }

    /// Reset fields to their default value i.e. 0.
    pub fn reset(&mut self) {
        self.cell.iter_mut().for_each(|sf_cell| {
            sf_cell.fill(zero());
        });
    }

    /// Add another [ScalarFluxTask]'s value to its own. Replace by an overload?
    pub fn add(&mut self, scalar_flux_task: &ScalarFluxTask<T>) {
        let n_groups = self.cell[0].len();
        (0..self.cell.len()).for_each(|cell_idx| {
            (0..n_groups).for_each(|group_idx| {
                self.cell[cell_idx][group_idx] += scalar_flux_task.cell[cell_idx][group_idx];
            })
        });
    }
}

/// ?
#[derive(Debug)]
pub struct CellTallyDomain<T: CustomFloat> {
    pub task: Vec<CellTallyTask<T>>,
}

impl<T: CustomFloat> CellTallyDomain<T> {
    /// Constructor
    pub fn new(domain: &MCDomain<T>, cell_tally_replications: usize) -> Self {
        let task = vec![CellTallyTask::new(domain); cell_tally_replications];
        Self { task }
    }
}

/// ?
#[derive(Debug)]
pub struct ScalarFluxDomain<T: CustomFloat> {
    pub task: Vec<ScalarFluxTask<T>>,
}

impl<T: CustomFloat> ScalarFluxDomain<T> {
    // Constructor
    pub fn new(domain: &MCDomain<T>, num_groups: usize, flux_replications: usize) -> Self {
        let task = vec![ScalarFluxTask::new(domain, num_groups); flux_replications];
        Self { task }
    }
}

/// ?
#[derive(Debug, Default)]
pub struct FluenceDomain<T: CustomFloat> {
    pub cell: Vec<T>,
}

impl<T: CustomFloat> FluenceDomain<T> {
    pub fn add_cell(&mut self, index: usize, val: T) {
        self.cell[index] += val;
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
pub struct Tallies<T: CustomFloat> {
    pub balance_cumulative: Balance,
    pub balance_task: Vec<Balance>,
    pub scalar_flux_domain: Vec<ScalarFluxDomain<T>>,
    pub cell_tally_domain: Vec<CellTallyDomain<T>>,
    pub fluence: Fluence<T>,
    pub spectrum: EnergySpectrum,
    pub num_balance_replications: u32,
    pub num_flux_replications: u32,
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
            balance_cumulative: Balance::default(),
            balance_task: Vec::new(),
            scalar_flux_domain: Vec::new(),
            cell_tally_domain: Vec::new(),
            fluence: Fluence { domain: Vec::new() },
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

        // Initialize the balance tallies
        if self.balance_task.is_empty() {
            if self.balance_task.capacity() == 0 {
                self.balance_task
                    .reserve(self.num_balance_replications as usize);
            }

            (0..self.num_balance_replications).for_each(|_| {
                self.balance_task.push(Balance::default());
            });
        }

        // Initialize the cell tallies
        if self.cell_tally_domain.is_empty() {
            if self.cell_tally_domain.capacity() == 0 {
                self.cell_tally_domain.reserve(domain.len());
            }

            (0..domain.len()).for_each(|domain_idx| {
                self.cell_tally_domain.push(CellTallyDomain::new(
                    &domain[domain_idx],
                    self.num_cell_tally_replications as usize,
                ));
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
                    self.num_flux_replications as usize,
                ));
            });
        }
    }

    /// Sums the task-level data. This is used when replications
    /// is active.
    pub fn sum_tasks(&mut self) {
        (1..self.num_balance_replications).for_each(|rep_idx| {
            let bal = self.balance_task[rep_idx as usize].clone(); // is there a cheaper way?
            self.balance_task[0].add(&bal);
            self.balance_task[rep_idx as usize].reset();
        });
    }

    /// Prints summarized data recorded by the tallies.
    pub fn print_summary(&self, mcco: &MonteCarlo<T>) {
        if mcco.time_info.cycle == 0 {
            // print header
            print!("cycle     |      start       source           rr        split       absorb      scatter      fission ");
            println!("     produce    collision       escape       census      num_seg    scalar_flux      cycleInit  cycleTracking  cycleFinalize");
        }
        let cy_init = mc_fast_timer::get_last_cycle(mcco, Section::CycleInit);
        let cy_track = mc_fast_timer::get_last_cycle(mcco, Section::CycleTracking);
        let cy_fin = mc_fast_timer::get_last_cycle(mcco, Section::CycleFinalize);
        let sf_sum = self.scalar_flux_sum();
        let bal = &self.balance_task[0];
        println!("{:>9} | {:>10} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12}    {:.6e} {:>14e} {:>14e} {:>14e}",
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

    /// Atomic add?
    pub fn tally_scalar_flux(
        &mut self,
        value: T,
        domain: usize,
        task: usize,
        cell: usize,
        group: usize,
    ) {
        self.scalar_flux_domain[domain].task[task].cell[cell][group] += value;
    }

    /// Atomic add?
    pub fn tally_cell_value(&mut self, value: T, domain: usize, task: usize, cell: usize) {
        self.cell_tally_domain[domain].task[task].cell[cell] += value;
    }

    /// Sums above all ?
    pub fn scalar_flux_sum(&self) -> T {
        let mut sum: T = zero();

        let n_domain = self.scalar_flux_domain.len();
        // for all domains
        (0..n_domain).for_each(|domain_idx| {
            // for each (replicated) tally
            (0..self.num_flux_replications).for_each(|rep_idx| {
                let n_cells = self.scalar_flux_domain[domain_idx].task[rep_idx as usize]
                    .cell
                    .len();
                // for each cell
                (0..n_cells).for_each(|cell_idx| {
                    let n_groups = self.scalar_flux_domain[domain_idx].task[rep_idx as usize].cell
                        [cell_idx]
                        .len();
                    // for each energy group
                    (0..n_groups).for_each(|group_idx| {
                        sum += self.scalar_flux_domain[domain_idx].task[rep_idx as usize].cell
                            [cell_idx][group_idx];
                    })
                })
            })
        });

        sum
    }
}
