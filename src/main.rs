use clap::Parser;

use fastiron::io_utils::Cli;
use fastiron::parameters::get_parameters;

fn main() {
    let cli = Cli::parse();
    println!("Printing CLI args:\n{:#?}", cli);
    let params = get_parameters(cli);
    println!("Printing Parameters:\n{:#?}", params.unwrap());
}
