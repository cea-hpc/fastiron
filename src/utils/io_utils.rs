//! Code used to manage I/O
//!
//! This module contains code used for the CLI and parsing input files.

use std::{fs::File, io::Read};

use crate::constants::CustomFloat;
use clap::Parser;

use crate::parameters::{
    Block, CrossSectionParameters, GeometryParameters, MaterialParameters, Parameters,
};

/// Enum used to categorize error related to the input of the program.
#[derive(Debug, PartialEq, Clone)]
pub enum InputError {
    BadInputFile,
    BadSimulationBlock,
    BadGeometryBlock,
    BadMaterialBlock,
    BadCrossSectionBlock,
    BadBlockType,
}

/// Fastiron, a Rust port of the Quicksilver proxy-app
#[derive(Debug, Parser)]
#[command(author, version, about, arg_required_else_help(true))]
pub struct Cli {
    /// name of input file
    #[arg(short = 'i', long = "input-file", num_args(1))]
    pub input_file: Option<String>,

    /// name of energy spectrum output file
    #[arg(short = 'e', long = "energy-spectrum", num_args(1))]
    pub energy_spectrum: Option<String>,

    /// name of cross-section output file
    #[arg(short = 'S', long = "cross-sections", num_args(1))]
    pub cross_sections_out: Option<String>,

    /// time step in seconds
    #[arg(short = 'D', long = "dt", num_args(1), allow_negative_numbers(false))]
    pub dt: Option<f32>,

    /// enable load balancing if present
    #[arg(short = 'l', long = "load-balance", num_args(0))]
    pub load_balance: bool,

    /// enable cycle timers if present
    #[arg(short = 'c', long = "cycle-timers", num_args(0))]
    pub cycle_timers: bool,

    /// enable thread debugging if present
    #[arg(short = 't', long = "debug-threads", num_args(0))]
    pub debug_threads: bool,

    /// x-size of simulation in cm
    #[arg(short = 'X', long = "lx", num_args(1), allow_negative_numbers(false))]
    pub lx: Option<f32>,

    /// y-size of simulation in cm
    #[arg(short = 'Y', long = "ly", num_args(1), allow_negative_numbers(false))]
    pub ly: Option<f32>,

    /// z-size of simulation in cm
    #[arg(short = 'Z', long = "lz", num_args(1), allow_negative_numbers(false))]
    pub lz: Option<f32>,

    /// total number of particules
    #[arg(
        short = 'n',
        long = "n-particles",
        num_args(1),
        allow_negative_numbers(false)
    )]
    pub n_particles: Option<u64>,

    /// number of threads that should be used to run the simulation
    #[arg(
        short = 'b',
        long = "n-thread",
        num_args(1),
        allow_negative_numbers(false)
    )]
    pub n_threads: Option<u64>,

    /// number of steps simulated
    #[arg(
        short = 'N',
        long = "n-steps",
        num_args(1),
        allow_negative_numbers(false)
    )]
    pub n_steps: Option<usize>,

    /// number of mesh elements along x
    #[arg(short = 'x', long = "nx", num_args(1), allow_negative_numbers(false))]
    pub nx: Option<usize>,

    /// number of mesh elements along y
    #[arg(short = 'y', long = "ny", num_args(1), allow_negative_numbers(false))]
    pub ny: Option<usize>,

    /// number of mesh elements along z
    #[arg(short = 'z', long = "nz", num_args(1), allow_negative_numbers(false))]
    pub nz: Option<usize>,

    /// random number seed
    #[arg(short = 's', long = "seed", num_args(1), allow_negative_numbers(false))]
    pub seed: Option<u64>, //maybe allow negative values ? need to test QS behavior

    /// number of balance tally replications
    #[arg(
        short = 'B',
        long = "b-tally",
        num_args(1),
        allow_negative_numbers(false)
    )]
    pub balance_tally_replications: Option<u32>,

    /// number of scalar flux tally replications
    #[arg(
        short = 'F',
        long = "f-tally",
        num_args(1),
        allow_negative_numbers(false)
    )]
    pub flux_tally_replications: Option<u32>,

    /// number of scalar cell tally replications
    #[arg(
        short = 'C',
        long = "c-tally",
        num_args(1),
        allow_negative_numbers(false)
    )]
    pub cell_tally_replications: Option<u32>,
}

/// Updates the Parameters structure passed as argument using the
/// provided input file. The file is first separated into blocks
/// with the rsplit call. The blocks are then used to complete the
/// parameter structure passed as argument.
pub fn parse_input_file<T: CustomFloat>(
    filename: String,
    params: &mut Parameters<T>,
) -> Result<(), Vec<InputError>> {
    let mut content = String::new();

    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Err(vec![InputError::BadInputFile]),
    };

    file.read_to_string(&mut content).unwrap();

    let res: Vec<Result<(), InputError>> = content
        .rsplit("\n\n")
        .map(|raw_block| {
            if let Some(val) = raw_block.find('\n') {
                let some_struct: Block = serde_yaml::from_str(&raw_block[val + 1..]).unwrap();
                //println!("{:#?}", some_struct); // uncomment if a parsing issue occur.

                match &raw_block[0..val] {
                    "Simulation:" => params.update_simulation_parameters(some_struct),
                    "Geometry:" => match GeometryParameters::from_block(some_struct) {
                        Ok(some_geometry) => {
                            params.add_geometry_parameter(some_geometry);
                            return Ok(());
                        }
                        Err(e) => Err(e),
                    },
                    "Material:" => match MaterialParameters::from_block(some_struct) {
                        Ok(some_material) => {
                            params.add_material_parameter(some_material);
                            return Ok(());
                        }
                        Err(e) => Err(e),
                    },
                    "CrossSection:" => match CrossSectionParameters::from_block(some_struct) {
                        Ok(some_cross_section) => {
                            params.add_cross_section_parameter(some_cross_section);
                            return Ok(());
                        }
                        Err(e) => Err(e),
                    },
                    _ => Err(InputError::BadBlockType),
                }?;
            }
            Ok(())
        })
        .collect();

    let errors: Vec<InputError> = res.iter().filter_map(|r| r.clone().err()).collect();

    if errors.is_empty() {
        return Ok(());
    }
    Err(errors)
}
