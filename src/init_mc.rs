use crate::{montecarlo::MonteCarlo, parameters::Parameters, mc::{mc_domain::MCDomain, mc_vector::MCVector}, global_fcc_grid::GlobalFccGrid};
use num::Float;

/// Creates a [MonteCarlo] object using the specified parameters.
pub fn init_mc<T: Float>(params: &Parameters) -> MonteCarlo<T> {
    let mut mcco: MonteCarlo<T> = MonteCarlo::new(params);
    
    init_proc_info(&mut mcco);
    init_time_info(&mut mcco, params);
    init_nuclear_data(&mut mcco, params);
    init_mesh(&mut mcco, params);
    init_tallies(&mut mcco, params);

    //check_cross_sections(&mcco, params);

    mcco
}

fn init_proc_info<T: Float>(mcco: &mut MonteCarlo<T>) {
    todo!()
}

fn init_time_info<T: Float>(mcco: &mut MonteCarlo<T>, params: &Parameters) {
    todo!()
}

fn init_nuclear_data<T: Float>(mcco: &mut MonteCarlo<T>, params: &Parameters) {
    todo!()
}

fn consistency_check<T: Float>(my_rank: usize, domain: &[MCDomain<T>]) {
    todo!()
}

fn initialize_centers_rand<T: Float>(n_centers: usize, grid: &GlobalFccGrid<T>) -> Vec<MCVector<T>> {
    todo!()
}

fn initialize_centers_grid<T: Float>(lx: T, ly: T, lz: T, x_dom: usize, y_dom: usize, z_dom: usize) -> Vec<MCVector<T>> {
    todo!()
}

fn init_mesh<T: Float>(mcco: &mut MonteCarlo<T>, params: &Parameters) {
    todo!()
}

fn init_tallies<T: Float>(mcco: &mut MonteCarlo<T>, params: &Parameters) {
    todo!()
}

pub fn check_cross_sections<T: Float>(mcco: &MonteCarlo<T>, params: &Parameters) {
    todo!()
}