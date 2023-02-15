use std::collections::HashMap;

use crate::io_utils::{self, InputError};

pub type Block = HashMap<String, String>;

#[derive(Debug, PartialEq)]
pub enum Shape {
    Undefined,
    Brick,
    Sphere,
}

#[derive(Debug)]
pub struct GeometryParameters {
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
    #[allow(clippy::too_many_arguments)]
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

    pub fn from_block(block: Block) -> Result<Self, InputError> {
        let mut geometry_params = Self::default();

        for (key, val) in block {
            match key.as_ref() {
                "material" => geometry_params.material_name = val,
                "shape" => {
                    geometry_params.shape = match val.as_ref() {
                        "brick" => Shape::Brick,
                        "sphere" => Shape::Sphere,
                        _ => return Err(InputError::BadGeometryBlock),
                    }
                }
                "radius" => {
                    geometry_params.radius = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadGeometryBlock),
                    }
                }
                "xCenter" => {
                    geometry_params.x_center = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadGeometryBlock),
                    }
                }
                "yCenter" => {
                    geometry_params.y_center = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadGeometryBlock),
                    }
                }
                "zCenter" => {
                    geometry_params.z_center = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadGeometryBlock),
                    }
                }
                "xMin" => {
                    geometry_params.x_min = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadGeometryBlock),
                    }
                }
                "yMin" => {
                    geometry_params.y_min = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadGeometryBlock),
                    }
                }
                "zMin" => {
                    geometry_params.z_min = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadGeometryBlock),
                    }
                }
                "xMax" => {
                    geometry_params.x_max = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadGeometryBlock),
                    }
                }
                "yMax" => {
                    geometry_params.y_max = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadGeometryBlock),
                    }
                }
                "zMax" => {
                    geometry_params.z_max = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadGeometryBlock),
                    }
                }
                _ => return Err(InputError::BadGeometryBlock),
            }
        }

        Ok(geometry_params)
    }
}

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
pub struct MaterialParameters {
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
    #[allow(clippy::too_many_arguments)]
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

    /// Creates a MaterialParameter object from a block of an input file.
    pub fn from_block(block: Block) -> Result<Self, InputError> {
        let mut material_params = Self::default();

        for (key, val) in block {
            match key.as_ref() {
                "name" => material_params.name = val,
                "mass" => {
                    material_params.mass = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadMaterialBlock),
                    }
                }
                "totalCrossSection" => {
                    material_params.total_cross_section = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadMaterialBlock),
                    }
                }
                "nIsotopes" => {
                    material_params.n_isotopes = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadMaterialBlock),
                    }
                }
                "nReactions" => {
                    material_params.n_reactions = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadMaterialBlock),
                    }
                }
                "sourceRate" => {
                    material_params.source_rate = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadMaterialBlock),
                    }
                }
                "scatteringCrossSection" => {
                    material_params.scattering_cross_section = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadMaterialBlock),
                    }
                }
                "absorptionCrossSection" => {
                    material_params.absorption_cross_section = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadMaterialBlock),
                    }
                }
                "fissionCrossSection" => {
                    material_params.fission_cross_section = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadMaterialBlock),
                    }
                }
                "scatteringCrossSectionRatio" => {
                    material_params.scattering_cross_section_ratio = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadMaterialBlock),
                    }
                }
                "absorptionCrossSectionRatio" => {
                    material_params.absorbtion_cross_section_ratio = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadMaterialBlock),
                    }
                }
                "fissionCrossSectionRatio" => {
                    material_params.fission_cross_section_ratio = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadMaterialBlock),
                    }
                }
                _ => return Err(InputError::BadMaterialBlock),
            }
        }

        Ok(material_params)
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
pub struct CrossSectionParameters {
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

    pub fn from_block(block: Block) -> Result<Self, InputError> {
        let mut cross_section = Self::default();

        for (key, val) in block {
            match key.as_ref() {
                "name" => cross_section.name = val,
                "A" => {
                    cross_section.aa = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadCrossSectionBlock),
                    }
                }
                "B" => {
                    cross_section.bb = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadCrossSectionBlock),
                    }
                }
                "C" => {
                    cross_section.cc = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadCrossSectionBlock),
                    }
                }
                "D" => {
                    cross_section.dd = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadCrossSectionBlock),
                    }
                }
                "E" => {
                    cross_section.ee = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadCrossSectionBlock),
                    }
                }
                "nuBar" => {
                    cross_section.nu_bar = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadCrossSectionBlock),
                    }
                }
                _ => return Err(InputError::BadCrossSectionBlock),
            }
        }
        Ok(cross_section)
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

/// Structure encompassing all simulation parameters. In the program's
/// execution flow, it is first initialized using the CLI arguments,
/// then optionally updated with a specified input file.
#[derive(Debug)]
pub struct SimulationParameters {
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
    coral_benchmark: bool,
}

impl SimulationParameters {
    /// Constructor.
    #[allow(clippy::too_many_arguments)]
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
        coral_benchmark: bool,
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
            coral_benchmark,
        }
    }

    /// Initialize a SimulationParameter object using a Cli object created
    /// from the arguments supplied via command line.
    ///
    /// ```rust
    /// use clap::Parser;
    /// use fastiron::io_utils::Cli;
    /// use fastiron::parameters::SimulationParameters;
    ///
    ///
    /// let cli = Cli::parse_from("./fastiron -i somefile -c -l".split(' '));
    /// let simulation_params = SimulationParameters::from_cli(&cli);
    /// // compare the structures...
    /// println!("{:#?}", cli);
    /// println!("{:#?}", simulation_params);
    /// 
    /// ```
    ///
    pub fn from_cli(cli: &io_utils::Cli) -> Self {
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
        simulation_params.cycle_timers = cli.cycle_timers;
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
            coral_benchmark: false,
        }
    }
}

/// Strucure encompassing all the problem's parameters. It is
/// created, completed and returned by the [get_parameters] method.
///
#[derive(Debug, Default)]
pub struct Parameters {
    simulation_params: SimulationParameters,
    geometry_params: Vec<GeometryParameters>,
    material_params: HashMap<String, MaterialParameters>,
    cross_section_params: HashMap<String, CrossSectionParameters>,
}

impl Parameters {
    pub fn update_simulation_parameters(&mut self, sim_block: Block) -> Result<(), InputError> {
        for (key, val) in sim_block {
            match key.as_ref() {
                "inputFile" => self.simulation_params.input_file = val,
                "energySpectrum" => self.simulation_params.energy_spectrum = val,
                "crossSectionsOut" => self.simulation_params.cross_sections_out = val,
                "boundaryCondition" => self.simulation_params.boundary_condition = val,
                "loadBalance" => {
                    self.simulation_params.load_balance = match val.as_ref() {
                        "0" => false,
                        "1" => true,
                        _ => return Err(InputError::BadSimulationBlock),
                    }
                }
                "cycleTimers" => {
                    self.simulation_params.cycle_timers = match val.as_ref() {
                        "0" => false,
                        "1" => true,
                        _ => return Err(InputError::BadSimulationBlock),
                    }
                }
                "debugThreads" => {
                    self.simulation_params.debug_threads = match val.as_ref() {
                        "0" => false,
                        "1" => true,
                        _ => return Err(InputError::BadSimulationBlock),
                    }
                }
                "nParticles" => {
                    self.simulation_params.n_particles = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "batchSize" => {
                    self.simulation_params.batch_size = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "nBatches" => {
                    self.simulation_params.n_batches = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "nSteps" => {
                    self.simulation_params.n_steps = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "nx" => {
                    self.simulation_params.nx = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "ny" => {
                    self.simulation_params.ny = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "nz" => {
                    self.simulation_params.nz = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "seed" => {
                    self.simulation_params.seed = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "xDom" | "yDom" | "zDom" => (),
                "dt" => {
                    self.simulation_params.dt = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "fMax" => {
                    self.simulation_params.f_max = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "lx" => {
                    self.simulation_params.lx = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "ly" => {
                    self.simulation_params.ly = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "lz" => {
                    self.simulation_params.lz = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "eMin" => {
                    self.simulation_params.e_min = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "eMax" => {
                    self.simulation_params.e_max = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "nGroups" => {
                    self.simulation_params.n_groups = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "lowWeightCutoff" => {
                    self.simulation_params.low_weight_cutoff = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "balanceTallyReplications" => {
                    self.simulation_params.balance_tally_replications = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "fluxTallyReplications" => {
                    self.simulation_params.flux_tally_replications = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "cellTallyReplications" => {
                    self.simulation_params.cell_tally_replications = match val.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(InputError::BadSimulationBlock),
                    }
                }
                "mpiThreadMultiple" => (),
                "coralBenchmark" => {
                    self.simulation_params.coral_benchmark = match val.as_ref() {
                        "0" => false,
                        "1" => true,
                        _ => return Err(InputError::BadSimulationBlock),
                    }
                }
                _ => return Err(InputError::BadSimulationBlock),
            }
        }
        Ok(())
    }
    pub fn add_geometry_parameter(&mut self, some_geometry: GeometryParameters) {
        self.geometry_params.push(some_geometry);
    }
    pub fn add_material_parameter(&mut self, some_material: MaterialParameters) {
        self.material_params
            .insert(some_material.name.to_owned(), some_material);
    }
    pub fn add_cross_section_parameter(&mut self, cross_section: CrossSectionParameters) {
        self.cross_section_params
            .insert(cross_section.name.to_owned(), cross_section);
    }
}



/// Use the cli arguments to initialize parameters of the simulation, complete the
/// structure and return it.The function will fail if it cannot read or find the
/// specified input_file (if specified).
pub fn get_parameters(cli: io_utils::Cli) -> Result<Parameters, io_utils::InputError> {
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

    if let Some(filename) = cli.input_file { io_utils::parse_input_file(filename, &mut params)? };
    if let Some(filename) = cli.energy_spectrum { params.simulation_params.energy_spectrum = filename };
    if let Some(filename) = cli.cross_sections_out { params.simulation_params.cross_sections_out = filename };

    supply_defaults(&mut params);
    check_parameters_integrity(&params);

    Ok(params)
}

/// Supply default parameters for the simulation if needed. The default problem
/// is provided if no geometries were specified.
fn supply_defaults(params: &mut Parameters) {
    // no need for default problem
    if !params.geometry_params.is_empty() {
        return;
    }

    // add a flat cross section
    let flat_cross_section = CrossSectionParameters { name: "flat".to_string(), ..Default::default()};
    params
        .cross_section_params
        .insert(flat_cross_section.name.to_owned(), flat_cross_section);

    // add source material data
    let mut source_material = MaterialParameters {name: "source_material".to_string(), ..Default::default()};
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
    let mut source_geometry: GeometryParameters = GeometryParameters { material_name: "source_material".to_string(), ..Default::default()};
    source_geometry.shape = Shape::Brick;
    source_geometry.x_max = params.simulation_params.lx;
    source_geometry.y_max = params.simulation_params.ly;
    source_geometry.z_max = params.simulation_params.lz;
    params.geometry_params.push(source_geometry);
}

/// Verify that the Parameters object passed as argument allows
/// for simulation (not necessarily as intended though), i.e.:
/// 1. There is at least one geometry
/// 2. All geometries shape are defined, i.e. set as brick or sphere
/// 3. All material referenced in geometries exist in the material list
/// 4. All cross sections referenced in materials exist in the cross section list
fn check_parameters_integrity(params: &Parameters) {
    // 1.
    assert!(!params.geometry_params.is_empty());
    // 2. and 3.
    params.geometry_params.iter().for_each(|g| {
        assert!(g.shape != Shape::Undefined);
        assert!(params.material_params.contains_key(&g.material_name))
    });
    // 4.
    params.material_params.iter().for_each(|(_, val)| {
        assert!(params
            .cross_section_params
            .contains_key(&val.absorption_cross_section));
        assert!(params
            .cross_section_params
            .contains_key(&val.scattering_cross_section));
        assert!(params
            .cross_section_params
            .contains_key(&val.fission_cross_section));
    });
}
