use clap::Parser;

use rusteel::io_utils::Cli;
use rusteel::parameters::get_parameters;

fn main() {
    let cli = Cli::parse();
    println!("Printing CLI args:\n{:#?}", cli);
    let params = get_parameters(cli);
    println!("Printing Parameters:\n{:#?}", params);
}
