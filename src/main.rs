use clap::Parser;

use crate::parameters::get_parameters;

mod io_utils;
mod parameters;

fn main() {
    let cli = io_utils::Cli::parse();
    println!("Printing CLI args:\n{:#?}", cli);
    let params = get_parameters(cli);
    println!("Printing Parameters:\n{:#?}", params);
}
