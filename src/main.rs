use std::fmt::{Debug, Display};
use std::ops::AddAssign;

use clap::Parser;
use num::{Float, FromPrimitive};

use fastiron::coral_benchmark_correctness;
use fastiron::cycle_tracking::cycle_tracking_guts;
use fastiron::init_mc::init_mc;
use fastiron::io_utils::Cli;
use fastiron::mc::mc_fast_timer::{self, Section};
use fastiron::mc::mc_utils;
use fastiron::montecarlo::MonteCarlo;
use fastiron::parameters::get_parameters;
use fastiron::population_control;

fn main() {
    let cli = Cli::parse();
    //println!("Printing CLI args:\n{cli:#?}");

    let params = get_parameters(cli).unwrap();
    //println!("Printing Parameters:\n{params:#?}");

    let load_balance: bool = params.simulation_params.load_balance;

    let n_steps = params.simulation_params.n_steps;

    let mut mcco_obj: MonteCarlo<f64> = init_mc(params);
    let mcco = &mut mcco_obj;

    mc_fast_timer::start(mcco, Section::Main);

    for _ in 0..n_steps {
        cycle_init(mcco, load_balance);
        cycle_tracking(mcco);
        cycle_finalize(mcco);

        mcco.fast_timer.last_cycle_report();
    }

    mc_fast_timer::stop(mcco, Section::Main);

    game_over(mcco);

    coral_benchmark_correctness::coral_benchmark_correctness(mcco);
}

pub fn game_over<T: Float + Display + FromPrimitive + Default>(mcco: &mut MonteCarlo<T>) {
    mcco.fast_timer.cumulative_report();
    mcco.tallies.spectrum.print_spectrum(&mcco);
}

pub fn cycle_init<T: Float + FromPrimitive + Display + Default>(
    mcco: &mut MonteCarlo<T>,
    load_balance: bool,
) {
    mc_fast_timer::start(mcco, Section::CycleInit);

    mcco.clear_cross_section_cache();

    // mcco.tallies.cycle_initialize(mcco); // literally an empty function

    mcco.particle_vault_container
        .swap_processing_processed_vaults();
    mcco.particle_vault_container.collapse_processed();
    mcco.particle_vault_container.collapse_processing();

    let tmp = mcco.particle_vault_container.processing_vaults.len() as u64;
    mcco.tallies.balance_task[0].start = tmp;

    mcco.particle_buffer.initialize(mcco.domain.len());

    mc_utils::source_now(mcco);

    population_control::population_control(mcco, load_balance);
    let lwc = mcco.params.simulation_params.low_weight_cutoff;
    let spw = mcco.source_particle_weight;
    population_control::roulette_low_weight_particles(
        lwc,
        spw,
        &mut mcco.particle_vault_container,
        &mut mcco.tallies.balance_task[0],
    );

    mc_fast_timer::stop(mcco, Section::CycleInit);
}

pub fn cycle_tracking<T: Float + FromPrimitive + AddAssign + Display + Debug + Default>(
    mcco: &mut MonteCarlo<T>,
) {
    mc_fast_timer::start(mcco, Section::CycleTracking);
    let mut done = false;
    // execution policy
    let my_particle_vault = &mut mcco.particle_vault_container;
    //
    //
    //
    loop {
        //let mut particle_count: u64 = 0;

        while !done {
            let mut fill_vault: usize = 0;

            for processing_vault_idx in 0..my_particle_vault.processing_vaults.len() {
                // Computing block
                mc_fast_timer::start(mcco, Section::CycleTrackingKernel);

                let processed_vault_idx: usize =
                    my_particle_vault.get_first_empty_processed_vault().unwrap();

                let processing_vault =
                    &mut my_particle_vault.processing_vaults[processing_vault_idx];
                let processed_vault = &mut my_particle_vault.processed_vaults[processed_vault_idx];

                let num_particles = processing_vault.size();
                // match ExecPolicy cpu
                if num_particles != 0 {
                    for particle_index in 0..num_particles {
                        cycle_tracking_guts(
                            mcco,
                            particle_index,
                            processing_vault,
                            processed_vault,
                        );
                    }
                }

                //particle_count += num_particles as u64;

                mc_fast_timer::stop(mcco, Section::CycleTrackingKernel);

                // Inter-domain communication block
                mc_fast_timer::start(mcco, Section::CycleTrackingMPI);

                let send_q = &mut my_particle_vault.send_queue;

                for idx in 0..send_q.size() {
                    let send_q_t = send_q.data[idx].clone();
                    let mcb_particle = processing_vault.get_base_particle(idx);

                    mcco.particle_buffer
                        .buffer_particle(mcb_particle.unwrap(), send_q_t.neighbor);
                }

                processing_vault.clear();
                send_q.clear();

                my_particle_vault.clean_extra_vaults();
                mcco.particle_buffer.read_buffers(&mut fill_vault, mcco);

                mc_fast_timer::stop(mcco, Section::CycleTrackingMPI);
            }

            mc_fast_timer::start(mcco, Section::CycleTrackingMPI);

            my_particle_vault.collapse_processing();
            my_particle_vault.collapse_processed();
            done = mcco.particle_buffer.test_done_new(mcco);

            mc_fast_timer::stop(mcco, Section::CycleTrackingMPI);
        }

        done = mcco.particle_buffer.test_done_new(mcco);

        if done {
            break;
        }
    }
    //
    mc_fast_timer::stop(mcco, Section::CycleTracking);
}

pub fn cycle_finalize<T: Float + Display + FromPrimitive + Default>(mcco: &mut MonteCarlo<T>) {
    mc_fast_timer::start(mcco, Section::CycleFinalize);

    mcco.tallies.balance_task[0].end = mcco.particle_vault_container.processed_vaults.len() as u64;

    mcco.tallies.cycle_finalize(mcco);
    mcco.time_info.cycle += 1;

    mc_fast_timer::stop(mcco, Section::CycleFinalize);
}
