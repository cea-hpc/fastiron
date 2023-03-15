use clap::Parser;

use fastiron::constants::CustomFloat;
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
    println!("Printing Parameters:\n{params:#?}");

    let load_balance: bool = params.simulation_params.load_balance;

    let n_steps = params.simulation_params.n_steps;

    let mut mcco_obj: MonteCarlo<f64> = init_mc(params);
    let mcco = &mut mcco_obj;

    mc_fast_timer::start(mcco, Section::Main);

    for s in 0..n_steps {
        println!();
        println!();
        println!();
        println!("step: {s}");
        //println!("------cycle_init");
        cycle_init(mcco, load_balance);
        //println!("------cycle_tracking");
        cycle_tracking(mcco);
        //println!("------cycle_finalize");
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
    mcco.fast_timer.cumulative_report();

    mcco.tallies.spectrum.print_spectrum(mcco);
}

pub fn cycle_init<T: CustomFloat>(mcco: &mut MonteCarlo<T>, load_balance: bool) {
    mc_fast_timer::start(mcco, Section::CycleInit);

    mcco.clear_cross_section_cache();

    // mcco.tallies.cycle_initialize(mcco); // literally an empty function

    if mcco.params.simulation_params.debug_threads {
        println!(
            "# processing particles: {}",
            mcco.particle_vault_container.particles_processing_size()
        );
        println!(
            "# processed particles: {}",
            mcco.particle_vault_container.particles_processed_size()
        );
    }

    mcco.particle_vault_container
        .swap_processing_processed_vaults();

    if mcco.params.simulation_params.debug_threads {
        println!("swapped vault");
        println!(
            "# processing particles: {}",
            mcco.particle_vault_container.particles_processing_size()
        );
        println!(
            "# processed particles: {}",
            mcco.particle_vault_container.particles_processed_size()
        );
    }

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

    if mcco.params.simulation_params.debug_threads {
        println!(
            "{} processing particles",
            mcco.particle_vault_container.particles_processing_size()
        );
        println!(
            "{} processed particles",
            mcco.particle_vault_container.particles_processed_size()
        );
    }
}

pub fn cycle_tracking<T: CustomFloat>(mcco: &mut MonteCarlo<T>) {
    mc_fast_timer::start(mcco, Section::CycleTracking);
    let mut done = false;
    loop {
        //let mut particle_count: u64 = 0;

        while !done {
            let mut fill_vault: usize = 0;

            for processing_vault_idx in 0..mcco.particle_vault_container.processing_vaults.len() {
                // Computing block

                if mcco.params.simulation_params.debug_threads {
                    println!("processing vault #{processing_vault_idx}");
                    println!("processing vault capacity: {}", mcco.particle_vault_container.processing_vaults[processing_vault_idx].particles.len());
                    println!("processing vault size:     {}", mcco.particle_vault_container.processing_vaults[processing_vault_idx].size());
                }
                mc_fast_timer::start(mcco, Section::CycleTrackingKernel);

                let processed_vault_idx: usize = mcco
                    .particle_vault_container
                    .get_first_empty_processed_vault();

                //println!("current processed vault #{processed_vault_idx}");

                // number of VALID particles
                let num_particles =
                    mcco.particle_vault_container.processing_vaults[processing_vault_idx].size();

                //println!("{:#?}", mcco.particle_vault_container.processing_vaults[processing_vault_idx].particles);

                if num_particles != 0 {
                    // iterate directly on particles??
                    let mut particle_idx: usize = 0;
                    let mut processed_particles: usize = 0;
                    while particle_idx < mcco.particle_vault_container.vault_size {
                        //println!("processing particle #{particle_idx}");
                        cycle_tracking_guts(
                            mcco,
                            particle_idx,
                            &mut processed_particles,
                            processing_vault_idx,
                            processed_vault_idx,
                        );
                        // incremented in cycle_tracking guts, only if the particle was not invalid
                        particle_idx += 1;
                    }
                    //println!("vault #{processing_vault_idx} finished processing");
                }

                //particle_count += num_particles as u64;

                mc_fast_timer::stop(mcco, Section::CycleTrackingKernel);

                // Inter-domain communication block
                mc_fast_timer::start(mcco, Section::CycleTrackingMPI);

                let send_q = &mut mcco.particle_vault_container.send_queue;

                for idx in 0..send_q.size() {
                    let send_q_t = send_q.data[idx].clone();
                    let mcb_particle = mcco.particle_vault_container.processing_vaults
                        [processing_vault_idx]
                        .get_base_particle(idx);

                    mcco.particle_buffer
                        .buffer_particle(mcb_particle.unwrap(), send_q_t.neighbor);
                }

                //mcco.particle_vault_container.processing_vaults[processing_vault_idx].clear();
                send_q.clear();

                mcco.particle_vault_container.clean_extra_vaults();
                mcco.read_buffers(&mut fill_vault);

                mc_fast_timer::stop(mcco, Section::CycleTrackingMPI);
            }

            mc_fast_timer::start(mcco, Section::CycleTrackingMPI);

            mcco.particle_vault_container.collapse_processing();
            mcco.particle_vault_container.collapse_processed();
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

pub fn cycle_finalize<T: CustomFloat>(mcco: &mut MonteCarlo<T>) {
    mc_fast_timer::start(mcco, Section::CycleFinalize);

    mcco.tallies.balance_task[0].end = mcco.particle_vault_container.processed_vaults.len() as u64;

    mcco.cycle_finalize();
    mcco.time_info.cycle += 1;

    mc_fast_timer::stop(mcco, Section::CycleFinalize);
}
