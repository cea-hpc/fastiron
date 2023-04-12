use clap::Parser;
use fastiron::constants::CustomFloat;
use fastiron::init::{init_mc, init_particle_containers};
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
    }

    mc_fast_timer::stop(mcco, Section::Main);

    game_over(mcco);

    coral_benchmark_correctness(mcco.params.simulation_params.coral_benchmark, &mcco.tallies);
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

    container.swap_processing_processed();

    let tmp = container.processing_particles.len() as u64;
    mcco.tallies.balance_task[0].start = tmp;

    population_control::source_now(mcco, container);

    population_control::population_control(mcco, container);

    population_control::roulette_low_weight_particles(
        mcco.params.simulation_params.low_weight_cutoff,
        mcco.source_particle_weight,
        container,
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
            mc_fast_timer::start(mcco, Section::CycleTrackingKernel);

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
                    );
                    if base_particle.species != Species::Unknown {
                        container.processed_particles.push(base_particle.clone());
                    }
                });
            container.processing_particles.clear();

            mc_fast_timer::stop(mcco, Section::CycleTrackingKernel);
            mc_fast_timer::start(mcco, Section::CycleTrackingComm);

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
    mc_fast_timer::stop(mcco, Section::CycleTracking);
}

pub fn cycle_finalize<T: CustomFloat>(
    mcco: &mut MonteCarlo<T>,
    container: &mut ParticleContainer<T>,
) {
    mc_fast_timer::start(mcco, Section::CycleFinalize);

    // prepare data for summary
    mcco.tallies.balance_task[0].end = container.processed_particles.len() as u64;
    mcco.tallies.sum_tasks();
    // print summary
    //mc_fast_timer::stop(mcco, Section::CycleFinalize);
    mcco.tallies.print_summary(mcco);
    mc_fast_timer::start(mcco, Section::CycleFinalize);
    // record / process data for the next cycle
    mcco.tallies
        .cycle_finalize(mcco.params.simulation_params.coral_benchmark);
    mcco.update_spectrum(container);
    mcco.time_info.cycle += 1;

    mc_fast_timer::stop(mcco, Section::CycleFinalize);

    mcco.fast_timer.clear_last_cycle_timers();
}
