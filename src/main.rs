use clap::Parser;
use fastiron::constants::CustomFloat;
use fastiron::init_mc::{init_mc, init_particle_containers};
use fastiron::montecarlo::MonteCarlo;
use fastiron::parameters::Parameters;
use fastiron::particles::mc_base_particle::Species;
use fastiron::particles::particle_container::ParticleContainer;
use fastiron::simulation::cycle_tracking::cycle_tracking_guts;
use fastiron::simulation::population_control;
use fastiron::utils::coral_benchmark_correctness::coral_benchmark_correctness;
use fastiron::utils::io_utils::Cli;
use fastiron::utils::mc_fast_timer::{self, Section};

fn main() {
    let cli = Cli::parse();

    let params = Parameters::get_parameters(cli).unwrap();
    println!("Printing Parameters:\n{params:#?}");

    let n_steps = params.simulation_params.n_steps;

    let mut mcco_obj: MonteCarlo<f64> = init_mc(params);
    let mcco = &mut mcco_obj;

    // todo: write a cleaner, more flexible way to use the vector for individual containers
    let mut containers = init_particle_containers(&mcco.params, &mcco.processor_info);
    let container = &mut containers[0];

    mc_fast_timer::start(mcco, Section::Main);

    for _ in 0..n_steps {
        cycle_init(mcco, container);
        cycle_tracking(mcco, container);
        cycle_finalize(mcco, container);
        if mcco.params.simulation_params.cycle_timers {
            mcco.fast_timer.last_cycle_report();
        }
    }

    mc_fast_timer::stop(mcco, Section::Main);

    game_over(mcco);

    coral_benchmark_correctness(&mcco.params, &mcco.tallies);
}

pub fn game_over<T: CustomFloat>(mcco: &mut MonteCarlo<T>) {
    mcco.fast_timer.update_main_stats();

    mcco.fast_timer
        .cumulative_report(mcco.tallies.balance_cumulative.num_segments);

    mcco.tallies.spectrum.print_spectrum(mcco);
}

pub fn cycle_init<T: CustomFloat>(mcco: &mut MonteCarlo<T>, container: &mut ParticleContainer<T>) {
    mc_fast_timer::start(mcco, Section::CycleInit);

    mcco.clear_cross_section_cache();

    //mcco.particle_vault_container
    //    .swap_processing_processed_vaults();
    container.swap_processing_processed();

    //mcco.particle_vault_container.collapse_processed();
    //mcco.particle_vault_container.collapse_processing();

    let tmp = container.processing_particles.len() as u64;
    mcco.tallies.balance_task[0].start = tmp;

    population_control::source_now(mcco, container);

    population_control::population_control(mcco, container);

    population_control::roulette_low_weight_particles(
        mcco.params.simulation_params.low_weight_cutoff,
        mcco.source_particle_weight,
        container,
        &mut mcco.particle_vault_container,
        &mut mcco.tallies.balance_task[0],
    );

    mc_fast_timer::stop(mcco, Section::CycleInit);
}

pub fn cycle_tracking<T: CustomFloat>(
    mcco: &mut MonteCarlo<T>,
    container: &mut ParticleContainer<T>,
) {
    mc_fast_timer::start(mcco, Section::CycleTracking);
    let mut done = false;
    loop {
        while !done {
            // track particles
            container
                .processing_particles
                .iter_mut()
                .enumerate()
                .for_each(|(particle_idx, base_particle)| {
                    cycle_tracking_guts(
                        mcco,
                        base_particle,
                        particle_idx,
                        &mut container.extra_particles,
                        &mut container.send_queue,
                    )
                });
            // delete invalid ones
            container
                .processing_particles
                .retain(|pp| pp.species != Species::Unknown);
            /*
            for processing_vault_idx in 0..mcco.particle_vault_container.processing_vaults.len() {
                // Computing block
                mc_fast_timer::start(mcco, Section::CycleTrackingKernel);

                // number of VALID particles in current vault
                let num_particles =
                    mcco.particle_vault_container.processing_vaults[processing_vault_idx].size();

                // for all particles
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

                // this replace the "send" part (tx)
                // in a shared memory context, we add the particles to extra vaults
                mcco.particle_vault_container.read_send_queue();

                // this would be the "receive" part (rx)
                // in a shared memory context, we transfer the particles
                // from extra vaults to processing vaults
                mcco.particle_vault_container.clean_extra_vaults();

                mc_fast_timer::stop(mcco, Section::CycleTrackingComm);
            }
            */
            mc_fast_timer::start(mcco, Section::CycleTrackingComm);

            //mcco.particle_vault_container.collapse_processing();
            //mcco.particle_vault_container.collapse_processed();
            // clean extra here
            container.process_sq();
            container.clean_extra_vaults();

            done = container.test_done_new();

            mc_fast_timer::stop(mcco, Section::CycleTrackingComm);
        }
        done = container.test_done_new();

        if done {
            break;
        }
    }
    //
    mc_fast_timer::stop(mcco, Section::CycleTracking);
}

pub fn cycle_finalize<T: CustomFloat>(
    mcco: &mut MonteCarlo<T>,
    container: &mut ParticleContainer<T>,
) {
    mc_fast_timer::start(mcco, Section::CycleFinalize);

    mcco.tallies.balance_task[0].end = container.processed_particles.len() as u64;
    //mcco.particle_vault_container.particles_processed_size() as u64;

    mcco.cycle_finalize(container);
    mcco.time_info.cycle += 1;

    mc_fast_timer::stop(mcco, Section::CycleFinalize);

    mcco.fast_timer.clear_last_cycle_timers();
}
