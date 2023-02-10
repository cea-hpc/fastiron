use std::ptr::null;

use clap::Parser;
use parameters::Parameters;

mod io_utils;
mod parameters;

fn main() {
    let cli = io_utils::Cli::parse();
    println!("{:#?}", cli);
}
