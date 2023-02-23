use std::cell::RefCell;
use std::rc::Rc;

use clap::Parser;

use fastiron::coral_benchmark_correctness;
use fastiron::cycle_tracking::cycle_tracking_guts;
use fastiron::init_mc::init_mc;
use fastiron::io_utils::Cli;
use fastiron::mc::mc_fast_timer::{self, Section};
use fastiron::mc::mc_utils;
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

    mc_fast_timer::start(mcco.clone(), Section::Main as usize);

    for _ in 0..n_steps {
        cycle_init(mcco.clone(), load_balance);
        cycle_tracking(mcco.clone());
        cycle_finalize(mcco.clone());

        mcco.borrow().fast_timer.last_cycle_report();
    }

    mc_fast_timer::stop(mcco.clone(), Section::Main as usize);

    game_over(mcco.clone());

    coral_benchmark_correctness::coral_benchmark_correctness(mcco, &params);
}

pub fn game_over<T: Float>(mcco: Rc<RefCell<MonteCarlo<T>>>) {
    mcco.borrow().fast_timer.cumulative_report();
    mcco.borrow()
        .tallies
        .spectrum
        .print_spectrum(&mcco.borrow());
}

pub fn cycle_init<T: Float>(mcco: Rc<RefCell<MonteCarlo<T>>>, load_balance: bool) {
    mc_fast_timer::start(mcco.clone(), Section::CycleInit as usize);

    mcco.borrow_mut().clear_cross_section_cache();

    // mcco.tallies.cycle_initialize(mcco); // literally an empty function

    mcco.borrow_mut()
        .particle_vault_container
        .swap_processing_processed_vaults();
    mcco.borrow_mut()
        .particle_vault_container
        .collapse_processed();
    mcco.borrow_mut()
        .particle_vault_container
        .collapse_processing();

    mcco.borrow_mut().tallies.balance_task[0].start = mcco
        .borrow()
        .particle_vault_container
        .processing_vaults
        .len() as u64;

    mcco.borrow_mut().particle_buffer.initialize();

    mc_utils::source_now(mcco.clone());

    population_control::population_control(mcco.clone(), load_balance);
    population_control::roulette_low_weight_particles(mcco.clone());

    mc_fast_timer::stop(mcco, Section::CycleInit as usize);
}

pub fn cycle_tracking<T: Float>(mcco: Rc<RefCell<MonteCarlo<T>>>) {
    mc_fast_timer::start(mcco.clone(), Section::CycleTracking as usize);
    let mut done = false;
    // execution policy
    let my_particle_vault = &mut mcco.borrow_mut().particle_vault_container;
    //
    //
    //
    loop {
        //let mut particle_count: u64 = 0;

        while !done {
            let mut fill_vault: usize = 0;

            for processing_vault_idx in 0..my_particle_vault.processing_vaults.len() {
                // Computing block
                mc_fast_timer::start(mcco.clone(), Section::CycleTrackingKernel as usize);

                let processed_vault_idx: usize =
                    my_particle_vault.get_first_empty_processed_vault();

                let processing_vault =
                    &mut my_particle_vault.processing_vaults[processing_vault_idx];
                let processed_vault = &mut my_particle_vault.processed_vaults[processed_vault_idx];

                let num_particles = processing_vault.size();
                // match ExecPolicy cpu
                if num_particles != 0 {
                    for particle_index in 0..num_particles {
                        cycle_tracking_guts(
                            mcco.clone(),
                            particle_index,
                            processing_vault,
                            processed_vault,
                        );
                    }
                }

                //particle_count += num_particles as u64;

                mc_fast_timer::stop(mcco.clone(), Section::CycleTrackingKernel as usize);

                // Inter-domain communication block
                mc_fast_timer::start(mcco.clone(), Section::CycleTrackingMPI as usize);

                let send_q = &mut my_particle_vault.send_queue;
                //mcco.borrow().particle_buffer....
                for idx in 0..send_q.size() {
                    let send_q_t = send_q.get_tuple(idx);
                    let mcb_particle = processing_vault.get_base_particle(idx);

                    mcco.borrow_mut()
                        .particle_buffer
                        .buffer_particle(mcb_particle.unwrap(), send_q_t.neighbor as usize);
                }

                processing_vault.clear();
                send_q.clear();

                my_particle_vault.clean_extra_vaults();
                mcco.borrow_mut()
                    .particle_buffer
                    .read_buffers(&mut fill_vault);

                mc_fast_timer::stop(mcco.clone(), Section::CycleTrackingMPI as usize);
            }

            mc_fast_timer::start(mcco.clone(), Section::CycleTrackingMPI as usize);

            my_particle_vault.collapse_processing();
            my_particle_vault.collapse_processed();
            done = mcco.borrow().particle_buffer.test_done_new();

            mc_fast_timer::stop(mcco.clone(), Section::CycleTrackingMPI as usize);
        }

        done = mcco.borrow().particle_buffer.test_done_new();

        if done {
            break;
        }
    }
    //
    mc_fast_timer::stop(mcco.clone(), Section::CycleTracking as usize);
}

pub fn cycle_finalize<T: Float>(mcco: Rc<RefCell<MonteCarlo<T>>>) {
    mc_fast_timer::start(mcco.clone(), Section::CycleFinalize as usize);

    mcco.borrow_mut().tallies.balance_task[0].end = mcco
        .borrow()
        .particle_vault_container
        .processed_vaults
        .len() as u64;

    mcco.borrow_mut().tallies.cycle_finalize(&mcco.borrow());
    mcco.borrow_mut().time_info.cycle += 1;
    //mcco.particle_buffer.free_memory();

    mc_fast_timer::stop(mcco, Section::CycleFinalize as usize);
}
