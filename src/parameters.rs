use std::collections::HashMap;

use crate::io_utils::{parse_input_file, Cli};

#[derive(Debug)]
enum Shape {
    Undefined,
    Brick,
    Sphere,
}

#[derive(Debug)]
struct GeometryParameters {
    material_name: String,
    shape: Shape,
    radius: f64,
    x_center: f64,
    y_center: f64,
    z_center: f64,
    x_min: f64,
    y_min: f64,
    z_min: f64,
    x_max: f64,
    y_max: f64,
    z_max: f64,
}

impl GeometryParameters {
    pub fn new(
        material_name: String,
        shape: Shape,
        radius: f64,
        x_center: f64,
        y_center: f64,
        z_center: f64,
        x_min: f64,
        y_min: f64,
        z_min: f64,
        x_max: f64,
        y_max: f64,
        z_max: f64,
    ) -> Self {
        Self {
            material_name,
            shape,
            radius,
            x_center,
            y_center,
            z_center,
            x_min,
            y_min,
            z_min,
            x_max,
            y_max,
            z_max,
        }
    }
}

// we define default init value using traits
impl Default for GeometryParameters {
    fn default() -> Self {
        Self {
            material_name: Default::default(),
            shape: Shape::Undefined,
            radius: 0.0,
            x_center: 0.0,
            y_center: 0.0,
            z_center: 0.0,
            x_min: 0.0,
            y_min: 0.0,
            z_min: 0.0,
            x_max: 0.0,
            y_max: 0.0,
            z_max: 0.0,
        }
    }
}

#[derive(Debug)]
struct MaterialParameters {
    name: String,
    mass: f64,
    total_cross_section: f64,
    n_isotopes: u32,
    n_reactions: u32,
    source_rate: f64,
    scattering_cross_section: String,
    absorption_cross_section: String,
    fission_cross_section: String,
    scattering_cross_section_ratio: f64,
    absorbtion_cross_section_ratio: f64,
    fission_cross_section_ratio: f64,
}

impl MaterialParameters {
    pub fn new(
        name: String,
        mass: f64,
        total_cross_section: f64,
        n_isotopes: u32,
        n_reactions: u32,
        source_rate: f64,
        scattering_cross_section: String,
        absorption_cross_section: String,
        fission_cross_section: String,
        scattering_cross_section_ratio: f64,
        absorbtion_cross_section_ratio: f64,
        fission_cross_section_ratio: f64,
    ) -> Self {
        Self {
            name,
            mass,
            total_cross_section,
            n_isotopes,
            n_reactions,
            source_rate,
            scattering_cross_section,
            absorption_cross_section,
            fission_cross_section,
            scattering_cross_section_ratio,
            absorbtion_cross_section_ratio,
            fission_cross_section_ratio,
        }
    }
}

impl Default for MaterialParameters {
    fn default() -> Self {
        Self {
            name: Default::default(),
            mass: 1000.0,
            total_cross_section: 1.0,
            n_isotopes: 10,
            n_reactions: 9,
            source_rate: 0.0,
            scattering_cross_section: Default::default(),
            absorption_cross_section: Default::default(),
            fission_cross_section: Default::default(),
            scattering_cross_section_ratio: 1.0,
            absorbtion_cross_section_ratio: 1.0,
            fission_cross_section_ratio: 1.0,
        }
    }
}

#[derive(Debug)]
struct CrossSectionParameters {
    name: String,
    aa: f64,
    bb: f64,
    cc: f64,
    dd: f64,
    ee: f64,
    nu_bar: f64,
}

impl CrossSectionParameters {
    pub fn new(name: String, aa: f64, bb: f64, cc: f64, dd: f64, ee: f64, nu_bar: f64) -> Self {
        Self {
            name,
            aa,
            bb,
            cc,
            dd,
            ee,
            nu_bar,
        }
    }
}

impl Default for CrossSectionParameters {
    fn default() -> Self {
        Self {
            name: Default::default(),
            aa: 0.0,
            bb: 0.0,
            cc: 0.0,
            dd: 0.0,
            ee: 1.0,
            nu_bar: 2.4,
        }
    }
}

#[derive(Debug)]
struct SimulationParameters {
    input_file: String,
    energy_spectrum: String,
    cross_sections_out: String,
    boundary_condition: String,
    load_balance: bool,
    cycle_timers: bool,
    debug_threads: bool,
    n_particles: u64,
    batch_size: u64,
    n_batches: u64,
    n_steps: u32,
    nx: u32,
    ny: u32,
    nz: u32,
    seed: u32,
    //x_dom: u32,
    //y_dom: u32,
    //z_dom: u32,
    dt: f64,
    f_max: f64,
    lx: f64,
    ly: f64,
    lz: f64,
    e_min: f64,
    e_max: f64,
    n_groups: u32,
    low_weight_cutoff: f64,
    balance_tally_replications: u32,
    flux_tally_replications: u32,
    cell_tally_replications: u32,
}

impl SimulationParameters {
    pub fn new(
        input_file: String,
        energy_spectrum: String,
        cross_sections_out: String,
        boundary_condition: String,
        load_balance: bool,
        cycle_timers: bool,
        debug_threads: bool,
        n_particles: u64,
        batch_size: u64,
        n_batches: u64,
        n_steps: u32,
        nx: u32,
        ny: u32,
        nz: u32,
        seed: u32,
        //x_dom: u32,
        //y_dom: u32,
        //z_dom: u32,
        dt: f64,
        f_max: f64,
        lx: f64,
        ly: f64,
        lz: f64,
        e_min: f64,
        e_max: f64,
        n_groups: u32,
        low_weight_cutoff: f64,
        balance_tally_replications: u32,
        flux_tally_replications: u32,
        cell_tally_replications: u32,
    ) -> Self {
        Self {
            input_file,
            energy_spectrum,
            cross_sections_out,
            boundary_condition,
            load_balance,
            cycle_timers,
            debug_threads,
            n_particles,
            batch_size,
            n_batches,
            n_steps,
            nx,
            ny,
            nz,
            seed,
            //x_dom,
            //y_dom,
            //z_dom,
            dt,
            f_max,
            lx,
            ly,
            lz,
            e_min,
            e_max,
            n_groups,
            low_weight_cutoff,
            balance_tally_replications,
            flux_tally_replications,
            cell_tally_replications,
        }
    }

    pub fn from_cli(cli: &Cli) -> Self {
        let mut simulation_params = Self::default();

        // use the cli to override defaults
        macro_rules! fetch_from_cli {
            ($f: ident) => {
                match &cli.$f {
                    Some(val) => simulation_params.$f = val.to_owned(),
                    None => (),
                }
            };
        }
        // same order as the struct declaration
        fetch_from_cli!(input_file);
        fetch_from_cli!(energy_spectrum);
        fetch_from_cli!(cross_sections_out);
        fetch_from_cli!(dt);
        fetch_from_cli!(f_max);
        simulation_params.load_balance = cli.load_balance;
        simulation_params.cycle_timers = cli.load_balance;
        // debug thread level?
        fetch_from_cli!(lx);
        fetch_from_cli!(ly);
        fetch_from_cli!(lz);
        fetch_from_cli!(n_particles);
        fetch_from_cli!(batch_size);
        fetch_from_cli!(n_batches);
        fetch_from_cli!(n_steps);
        fetch_from_cli!(nx);
        fetch_from_cli!(ny);
        fetch_from_cli!(nz);
        fetch_from_cli!(seed);
        //fetch_from_cli!(x_dom);
        //fetch_from_cli!(y_dom);
        //fetch_from_cli!(z_dom);
        fetch_from_cli!(balance_tally_replications);
        fetch_from_cli!(flux_tally_replications);
        fetch_from_cli!(cell_tally_replications);

        simulation_params
    }
}

impl Default for SimulationParameters {
    fn default() -> Self {
        Self {
            input_file: Default::default(),
            energy_spectrum: "".to_string(),
            cross_sections_out: "".to_string(),
            boundary_condition: "reflect".to_string(),
            load_balance: false,
            cycle_timers: false,
            debug_threads: false,
            n_particles: 1000000,
            batch_size: 0,
            n_batches: 10,
            n_steps: 10,
            nx: 10,
            ny: 10,
            nz: 10,
            seed: 1029384756,
            //x_dom: 0,
            //y_dom: 0,
            //z_dom: 0,
            dt: 1e-8,
            f_max: 0.1,
            lx: 100.0,
            ly: 100.0,
            lz: 100.0,
            e_min: 1e-9,
            e_max: 20.0,
            n_groups: 230,
            low_weight_cutoff: 0.001,
            balance_tally_replications: 1,
            flux_tally_replications: 1,
            cell_tally_replications: 1,
        }
    }
}

#[derive(Debug)]
pub struct Parameters {
    simulation_params: SimulationParameters,
    geometry_params: Vec<GeometryParameters>,
    material_params: HashMap<String, MaterialParameters>,
    cross_section_params: HashMap<String, CrossSectionParameters>,
}

impl Parameters {}

pub fn get_parameters(cli: Cli) -> Parameters {
    // structs init
    let simulation_params: SimulationParameters = SimulationParameters::from_cli(&cli);
    let geometry_params: Vec<GeometryParameters> = Vec::new();
    let material_params: HashMap<String, MaterialParameters> = HashMap::new();
    let cross_section_params: HashMap<String, CrossSectionParameters> = HashMap::new();

    let mut params = Parameters {
        simulation_params,
        geometry_params,
        material_params,
        cross_section_params,
    };

    match cli.input_file {
        Some(filename) => parse_input_file(filename, &mut params),
        None => (),
    };
    match cli.energy_spectrum {
        Some(filename) => params.simulation_params.energy_spectrum = filename,
        None => (),
    };
    match cli.cross_sections_out {
        Some(filename) => params.simulation_params.cross_sections_out = filename,
        None => (),
    };

    supply_defaults(&mut params);

    params
}

fn supply_defaults(params: &mut Parameters) {
    // no need for default problem
    if !params.geometry_params.is_empty() {
        return;
    }

    // add a flat cross section
    let mut flat_cross_section = CrossSectionParameters::default();
    flat_cross_section.name = "flat".to_string();
    params
        .cross_section_params
        .insert(flat_cross_section.name.to_owned(), flat_cross_section);

    // add source material data
    let mut source_material = MaterialParameters::default();
    source_material.name = "source_material".to_string();
    source_material.mass = 1000.0;
    source_material.source_rate = 1e10;
    source_material.scattering_cross_section = "flat".to_string();
    source_material.absorption_cross_section = "flat".to_string();
    source_material.fission_cross_section = "flat".to_string();
    source_material.fission_cross_section_ratio = 0.1;
    params
        .material_params
        .insert(source_material.name.to_owned(), source_material);

    // add source geometry. source material occupies all the space
    let mut source_geometry: GeometryParameters = GeometryParameters::default();
    source_geometry.material_name = "source_material".to_string();
    source_geometry.shape = Shape::Brick;
    source_geometry.x_max = params.simulation_params.lx;
    source_geometry.y_max = params.simulation_params.ly;
    source_geometry.z_max = params.simulation_params.lz;
    params.geometry_params.push(source_geometry);
}
