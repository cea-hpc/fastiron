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
use fastiron::utils::mc_processor_info::ExecPolicy;

fn main() {
    let cli = Cli::parse();

    let params = Parameters::get_parameters(cli).unwrap();
    println!("Printing Parameters:\n{params:#?}");

    let n_steps = params.simulation_params.n_steps;

    let mut mcco_obj: MonteCarlo<f32> = init_mc(params);
    let mcco = &mut mcco_obj;

    let mut containers = init_particle_containers(&mcco.params, &mcco.processor_info);

    match mcco.processor_info.exec_policy {
        ExecPolicy::Sequential => {
            let container = &mut containers[0];

            mc_fast_timer::start(mcco, Section::Main);

            for _ in 0..n_steps {
                cycle_init(mcco, container);
                cycle_tracking(mcco, container);
                cycle_finalize(mcco, container);
            }

            mc_fast_timer::stop(mcco, Section::Main);
        }
        ExecPolicy::Parallel => {
            todo!()
        }
    }

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
    mcco.tallies.balance_cycle.start = tmp;

    population_control::source_now(mcco, container);

    population_control::population_control(mcco, container);

    population_control::roulette_low_weight_particles(
        mcco.params.simulation_params.low_weight_cutoff,
        mcco.source_particle_weight,
        container,
        &mut mcco.tallies.balance_cycle,
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
                .for_each(|base_particle| {
                    println!("{base_particle:#?}");
                    cycle_tracking_guts(
                        mcco,
                        base_particle,
                        &mut container.extra_particles,
                        &mut container.send_queue,
                    );
                    if base_particle.species != Species::Unknown {
                        container.processed_particles.push(base_particle.clone());
                    }
                });
            container.processing_particles.clear();
            println!("Kernel tracking done");

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

    // prepare data for summary; this would be a sync phase in parallel exec
    mcco.tallies.balance_cycle.end = container.processed_particles.len() as u64;

    mc_fast_timer::stop(mcco, Section::CycleFinalize);

    // print summary
    mcco.tallies.print_summary(mcco);

    // record / process data for the next cycle
    mcco.tallies
        .cycle_finalize(mcco.params.simulation_params.coral_benchmark);
    mcco.update_spectrum(container);
    mcco.time_info.cycle += 1;

    mcco.fast_timer.clear_last_cycle_timers();
}
