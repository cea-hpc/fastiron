use clap::Parser;

use fastiron::constants::CustomFloat;
use fastiron::coral_benchmark_correctness;
use fastiron::init_mc::init_mc;
use fastiron::io_utils::Cli;
use fastiron::mc::mc_fast_timer::{self, Section};
use fastiron::mc::mc_utils;
use fastiron::montecarlo::MonteCarlo;
use fastiron::parameters::Parameters;
use fastiron::simulation::cycle_tracking::cycle_tracking_guts;
use fastiron::simulation::population_control;

fn main() {
    let cli = Cli::parse();

    let params = Parameters::get_parameters(cli).unwrap();
    println!("Printing Parameters:\n{params:#?}");

    let load_balance: bool = params.simulation_params.load_balance;

    let n_steps = params.simulation_params.n_steps;

    let mut mcco_obj: MonteCarlo<f64> = init_mc(params);
    let mcco = &mut mcco_obj;

    mc_fast_timer::start(mcco, Section::Main);

    for _ in 0..n_steps {
        cycle_init(mcco, load_balance);
        cycle_tracking(mcco);
        cycle_finalize(mcco);
        if mcco.params.simulation_params.cycle_timers {
            mcco.fast_timer.last_cycle_report();
        }
    }

    mc_fast_timer::stop(mcco, Section::Main);

    game_over(mcco);

    coral_benchmark_correctness::coral_benchmark_correctness(mcco);
}

pub fn game_over<T: CustomFloat>(mcco: &mut MonteCarlo<T>) {
    mcco.fast_timer.update_main_stats();

    mcco.fast_timer
        .cumulative_report(mcco.tallies.balance_cumulative.num_segments);

    mcco.tallies.spectrum.print_spectrum(mcco);
}

pub fn cycle_init<T: CustomFloat>(mcco: &mut MonteCarlo<T>, load_balance: bool) {
    mc_fast_timer::start(mcco, Section::CycleInit);

    mcco.clear_cross_section_cache();

    mcco.particle_vault_container
        .swap_processing_processed_vaults();

    mcco.particle_vault_container.collapse_processed();
    mcco.particle_vault_container.collapse_processing();

    let tmp = mcco.particle_vault_container.particles_processing_size() as u64;
    mcco.tallies.balance_task[0].start = tmp;

    mcco.particle_buffer.initialize(mcco.domain.len());

    mc_utils::source_now(mcco);

    population_control::population_control(mcco, load_balance);

    population_control::roulette_low_weight_particles(
        mcco.params.simulation_params.low_weight_cutoff,
        mcco.source_particle_weight,
        &mut mcco.particle_vault_container,
        &mut mcco.tallies.balance_task[0],
    );

    mc_fast_timer::stop(mcco, Section::CycleInit);
}

pub fn cycle_tracking<T: CustomFloat>(mcco: &mut MonteCarlo<T>) {
    mc_fast_timer::start(mcco, Section::CycleTracking);
    let mut done = false;
    loop {
        while !done {
            let mut fill_vault: usize = 0;

            for processing_vault_idx in 0..mcco.particle_vault_container.processing_vaults.len() {
                // Computing block
                mc_fast_timer::start(mcco, Section::CycleTrackingKernel);

                // number of VALID particles in current vault
                let num_particles =
                    mcco.particle_vault_container.processing_vaults[processing_vault_idx].size();

                if num_particles != 0 {
                    let mut particle_idx: usize = 0;
                    while particle_idx < mcco.particle_vault_container.vault_size {
                        cycle_tracking_guts(mcco, particle_idx, processing_vault_idx);
                        particle_idx += 1;
                    }
                }

                mc_fast_timer::stop(mcco, Section::CycleTrackingKernel);

                // Inter-domain communication block
                mc_fast_timer::start(mcco, Section::CycleTrackingComm);

                let send_q = &mut mcco.particle_vault_container.send_queue;

                for idx in 0..send_q.size() {
                    let send_q_t = send_q.data[idx].clone();
                    let mcb_particle = mcco.particle_vault_container.processing_vaults
                        [processing_vault_idx]
                        .get_base_particle(idx);

                    mcco.particle_buffer
                        .buffer_particle(mcb_particle.unwrap(), send_q_t.neighbor);
                }

                send_q.clear();

                mcco.particle_vault_container.clean_extra_vaults();
                mcco.read_buffers(&mut fill_vault);

                mc_fast_timer::stop(mcco, Section::CycleTrackingComm);
            }

            mc_fast_timer::start(mcco, Section::CycleTrackingComm);

            mcco.particle_vault_container.collapse_processing();
            mcco.particle_vault_container.collapse_processed();
            done = mcco.particle_buffer.test_done_new(mcco);

            mc_fast_timer::stop(mcco, Section::CycleTrackingComm);
        }

        done = mcco.particle_buffer.test_done_new(mcco);

        if done {
            break;
        }
    }
    //
    mc_fast_timer::stop(mcco, Section::CycleTracking);
}

pub fn cycle_finalize<T: CustomFloat>(mcco: &mut MonteCarlo<T>) {
    mc_fast_timer::start(mcco, Section::CycleFinalize);

    mcco.tallies.balance_task[0].end =
        mcco.particle_vault_container.particles_processed_size() as u64;

    mcco.cycle_finalize();
    mcco.time_info.cycle += 1;

    mc_fast_timer::stop(mcco, Section::CycleFinalize);

    mcco.fast_timer.clear_last_cycle_timers();
}
