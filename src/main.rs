use std::cell::RefCell;
use std::rc::Rc;

use clap::Parser;

use fastiron::coral_benchmark_correctness;
use fastiron::init_mc::init_mc;
use fastiron::io_utils::Cli;
use fastiron::mc::mc_fast_timer::{self, Section};
use fastiron::mc::mc_source_now;
use fastiron::montecarlo::MonteCarlo;
use fastiron::parameters::get_parameters;
use fastiron::population_control;
use num::Float;

fn main() {
    let cli = Cli::parse();
    println!("Printing CLI args:\n{cli:#?}");

    let params = get_parameters(cli).unwrap();
    println!("Printing Parameters:\n{params:#?}");

    let load_balance: bool = params.simulation_params.load_balance;

    let n_steps = params.simulation_params.n_steps;

    let mcco: Rc<RefCell<MonteCarlo<f64>>> = Rc::new(RefCell::new(init_mc(&params)));

    mc_fast_timer::start(&mut mcco.borrow_mut(), Section::Main as usize);

    for _ in 0..n_steps {
        cycle_init(&mut mcco.borrow_mut(), load_balance); // put the borrow inside the functions?
        cycle_tracking(&mut mcco.borrow_mut()); // put the borrow inside the functions?
        cycle_finalize(mcco.clone());

        mcco.borrow().fast_timer.last_cycle_report();
    }

    mc_fast_timer::stop(&mut mcco.borrow_mut(), Section::Main as usize);

    coral_benchmark_correctness::coral_benchmark_correctness(&mut mcco.borrow_mut(), &params);
}

pub fn game_over() {}

pub fn cycle_init<T: Float>(mcco: &mut MonteCarlo<T>, load_balance: bool) {
    mc_fast_timer::start(mcco, Section::CycleInit as usize);

    mcco.clear_cross_section_cache();

    // mcco.tallies.cycle_initialize(mcco); // literally an empty function

    mcco.particle_vault_container
        .swap_processing_processed_vaults();
    mcco.particle_vault_container.collapse_processed();
    mcco.particle_vault_container.collapse_processing();

    mcco.tallies.balance_task[0].start =
        mcco.particle_vault_container.processing_vaults.len() as u64;

    mcco.particle_buffer.initialize();

    mc_source_now::source_now(mcco);

    population_control::population_control(mcco, load_balance);
    population_control::roulette_low_weight_particles(mcco);

    mc_fast_timer::stop(mcco, Section::CycleInit as usize);
}

pub fn cycle_tracking<T: Float>(mcco: &mut MonteCarlo<T>) {
    mc_fast_timer::start(mcco, Section::CycleTracking as usize);
    let mut done = false;
    //
    loop {
        if !done {
            done = true;
        }
        if done {
            break;
        }
    }
    //
    mc_fast_timer::stop(mcco, Section::CycleTracking as usize);
}

pub fn cycle_finalize<T: Float>(mcco: Rc<RefCell<MonteCarlo<T>>>) {
    mc_fast_timer::start(&mut mcco.borrow_mut(), Section::CycleFinalize as usize);

    mcco.borrow_mut().tallies.balance_task[0].end = mcco
        .borrow()
        .particle_vault_container
        .processed_vaults
        .len() as u64;

    mcco.borrow_mut().tallies.cycle_finalize(&mcco.borrow());
    mcco.borrow_mut().time_info.cycle += 1;
    //mcco.particle_buffer.free_memory();

    mc_fast_timer::stop(&mut mcco.borrow_mut(), Section::CycleFinalize as usize);
}
