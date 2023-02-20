use std::rc::Rc;

use clap::Parser;

use fastiron::init_mc::init_mc;
use fastiron::io_utils::Cli;
use fastiron::montecarlo::MonteCarlo;
use fastiron::parameters::get_parameters;
use num::Float;

fn main() {
    let cli = Cli::parse();
    println!("Printing CLI args:\n{cli:#?}");
    
    let params = get_parameters(cli).unwrap();
    println!("Printing Parameters:\n{params:#?}");

    let load_balance: bool = params.simulation_params.load_balance;

    let n_steps = params.simulation_params.n_steps;

    let mcco: Rc<MonteCarlo<f64>> = Rc::new(init_mc(params));

    // MCFastTimer::start

    for _ in 0..n_steps {
        cycle_init(load_balance);
        cycle_tracking(&mcco);
        cycle_finalize();

        //mcco.fast_timer
    }

    // MCFastTimer::stop

    // coral_benchmark_correctness
}

pub fn game_over() {

}

pub fn cycle_init(load_balance: bool) {
    todo!()
}

pub fn cycle_tracking<T: Float>(mcco: &MonteCarlo<T>) {
    todo!()
}

pub fn cycle_finalize() {
    todo!()
}