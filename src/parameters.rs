use crate::constants::CustomFloat;
use std::collections::HashMap;

use crate::io_utils::{self, InputError};

/// Alias between Block and [HashMap<String,String>]. This allows for
/// better readability.
pub type Block = HashMap<String, String>;

/// Enum used to categorize inconsistencies within parameters.
/// - NoGeometry: there are no specified geometries
/// - UndefinedGeometry: a [GeometryParameters] object has an undefined [Shape]
/// - MissingMaterial: there is a missing reference to a material; the string
/// contains the name of the aforementioned material
/// - MissingCrossSection: there is a missing reference to a cross section;
/// the string contains the name of the aforementionned cross section and
/// the material refering to it
#[derive(Debug, PartialEq)]
pub enum ParameterError {
    NoGeometry,
    UndefinedGeometry,
    MissingMaterial(String),
    MissingCrossSection(String),
}

/// Enum used to describe a geometry's shape.
#[derive(Debug, PartialEq)]
pub enum Shape {
    Undefined,
    Brick,
    Sphere,
}

impl<T: CustomFloat> Parameters<T> {
    /// Use the cli arguments to initialize parameters of the simulation, complete the
    /// structure and return it. The function will fail if
    /// - it cannot read or find the specified input_file (if specified)
    /// - the resulting [Parameters] object is compromised
    pub fn get_parameters(cli: io_utils::Cli) -> Result<Self, Vec<io_utils::InputError>> {
        // structs init
        let mut params = Self {
            simulation_params: SimulationParameters::from_cli(&cli),
            geometry_params: Vec::new(),
            material_params: HashMap::new(),
            cross_section_params: HashMap::new(),
        };

        if let Some(filename) = cli.input_file {
            io_utils::parse_input_file(filename, &mut params)?
        };
        if let Some(filename) = cli.energy_spectrum {
            params.simulation_params.energy_spectrum = filename
        };
        if let Some(filename) = cli.cross_sections_out {
            params.simulation_params.cross_sections_out = filename
        };

        params.supply_defaults();
        if let Err(e) = params.check_parameters_integrity() {
            println!("{e:?}");
            return Err(vec![InputError::BadInputFile]);
        };

        Ok(params)
    }

    /// Supply default parameters for the simulation if needed. The default problem
    /// is provided if no geometries were specified.
    pub fn supply_defaults(&mut self) {
        // no need for default problem
        if !self.geometry_params.is_empty() {
            return;
        }

        // add a flat cross section
        let flat_cross_section = CrossSectionParameters {
            name: "flat".to_string(),
            ..Default::default()
        };
        self.cross_section_params
            .insert(flat_cross_section.name.to_owned(), flat_cross_section);

        // add source material data
        let mut source_material = MaterialParameters {
            name: "source_material".to_string(),
            ..Default::default()
        };
        source_material.mass = T::from_f64(1000.0).unwrap();
        source_material.source_rate = T::from_f64(1e10).unwrap();
        source_material.scattering_cross_section = "flat".to_string();
        source_material.absorption_cross_section = "flat".to_string();
        source_material.fission_cross_section = "flat".to_string();
        source_material.fission_cross_section_ratio = T::from_f64(0.1).unwrap();
        self.material_params
            .insert(source_material.name.to_owned(), source_material);

        // add source geometry. source material occupies all the space
        let mut source_geometry = GeometryParameters::<T> {
            material_name: "source_material".to_string(),
            ..Default::default()
        };
        source_geometry.shape = Shape::Brick;
        source_geometry.x_max = self.simulation_params.lx;
        source_geometry.y_max = self.simulation_params.ly;
        source_geometry.z_max = self.simulation_params.lz;
        self.geometry_params.push(source_geometry);
    }

    /// Verify that the Parameters object passed as argument allows
    /// for simulation (not necessarily as intended though), i.e.:
    /// 1. There is at least one geometry
    /// 2. All geometries shape are defined, i.e. set as brick or sphere
    /// 3. All material referenced in geometries exist in the material list
    /// 4. All cross sections referenced in materials exist in the cross section list
    pub fn check_parameters_integrity(&self) -> Result<(), Vec<ParameterError>> {
        let mut errors: Vec<ParameterError> = Vec::new();
        // 1.
        if self.geometry_params.is_empty() {
            errors.push(ParameterError::NoGeometry);
        }
        // 2. and 3.
        self.geometry_params
            .iter()
            .for_each(|g: &GeometryParameters<T>| {
                if g.shape == Shape::Undefined {
                    errors.push(ParameterError::UndefinedGeometry);
                }
                if !self.material_params.contains_key(&g.material_name) {
                    errors.push(ParameterError::MissingMaterial(g.material_name.to_owned()));
                }
            });
        // 4.
        self.material_params.iter().for_each(|(_, val)| {
            if !self
                .cross_section_params
                .contains_key(&val.absorption_cross_section)
            {
                errors.push(ParameterError::MissingCrossSection(
                    val.name.to_owned() + ":" + val.absorption_cross_section.as_ref(),
                ));
            }
            if !self
                .cross_section_params
                .contains_key(&val.scattering_cross_section)
            {
                errors.push(ParameterError::MissingCrossSection(
                    val.name.to_owned() + ":" + val.scattering_cross_section.as_ref(),
                ));
            }
            if !self
                .cross_section_params
                .contains_key(&val.fission_cross_section)
            {
                errors.push(ParameterError::MissingCrossSection(
                    val.name.to_owned() + ":" + val.fission_cross_section.as_ref(),
                ));
            }
        });
        if errors.is_empty() {
            return Ok(());
        }
        Err(errors)
    }
}

/// Structure used to describe a geometry, i.e. a physical space of a
/// certain shape and certain material.
#[derive(Debug)]
pub struct GeometryParameters<T: CustomFloat> {
    pub material_name: String,
    pub shape: Shape,
    pub radius: T,
    pub x_center: T,
    pub y_center: T,
    pub z_center: T,
    pub x_min: T,
    pub y_min: T,
    pub z_min: T,
    pub x_max: T,
    pub y_max: T,
    pub z_max: T,
}

impl<T: CustomFloat> GeometryParameters<T> {
    /// Creates a [GeometryParameters] object using the [Block] passed as
    /// argument. Any field not specified in the block will have its default
    /// value as defined in the [Default] implementation. May return an error
    /// if the block isn't a proper Geometry block, i.e.:
    /// - There is an unknown field
    /// - A value associated to a valid field is invalid
    /// In that case, the [GeometryParameters] object is scrapped instead of being
    /// returned as incomplete or potentially erroneous.
    pub fn from_block(block: Block) -> Result<Self, InputError> {
        let mut geometry_params = Self::default();

        macro_rules! fetch_data {
            ($f: ident, $v: expr) => {
                geometry_params.$f = match $v.parse() {
                    Ok(v) => v,
                    Err(_) => return Err(InputError::BadGeometryBlock),
                }
            };
        }
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
                "radius" => fetch_data!(radius, val),
                "xCenter" => fetch_data!(x_center, val),
                "yCenter" => fetch_data!(y_center, val),
                "zCenter" => fetch_data!(z_center, val),
                "xMin" => fetch_data!(x_min, val),
                "yMin" => fetch_data!(y_min, val),
                "zMin" => fetch_data!(z_min, val),
                "xMax" => fetch_data!(x_max, val),
                "yMax" => fetch_data!(y_max, val),
                "zMax" => fetch_data!(z_max, val),
                _ => return Err(InputError::BadGeometryBlock),
            }
        }

        Ok(geometry_params)
    }
}

impl<T: CustomFloat> Default for GeometryParameters<T> {
    fn default() -> Self {
        Self {
            material_name: Default::default(),
            shape: Shape::Undefined,
            radius: T::zero(),
            x_center: T::zero(),
            y_center: T::zero(),
            z_center: T::zero(),
            x_min: T::zero(),
            y_min: T::zero(),
            z_min: T::zero(),
            x_max: T::zero(),
            y_max: T::zero(),
            z_max: T::zero(),
        }
    }
}

/// Struct used to describe a material, i.e. its name and relevant physical
/// properties.
#[derive(Debug)]
pub struct MaterialParameters<T: CustomFloat> {
    pub name: String,
    pub mass: T,
    pub total_cross_section: T,
    pub n_isotopes: usize,
    pub n_reactions: usize,
    pub source_rate: T,
    pub scattering_cross_section: String,
    pub absorption_cross_section: String,
    pub fission_cross_section: String,
    pub scattering_cross_section_ratio: T,
    pub absorbtion_cross_section_ratio: T,
    pub fission_cross_section_ratio: T,
}

impl<T: CustomFloat> MaterialParameters<T> {
    /// Creates a [MaterialParameters] object using the [Block] passed as
    /// argument. Any field not specified in the block will have its default
    /// value as defined in the [Default] implementation. May return an error
    /// if the block isn't a proper Material block, i.e.:
    /// - There is an unknown field
    /// - A value associated to a valid field is invalid
    /// In that case, the [MaterialParameters] object is scrapped instead of being
    /// returned as incomplete or potentially erroneous.
    pub fn from_block(block: Block) -> Result<Self, InputError> {
        let mut material_params = Self::default();

        macro_rules! fetch_data {
            ($f: ident, $v: expr) => {
                material_params.$f = match $v.parse() {
                    Ok(v) => v,
                    Err(_) => return Err(InputError::BadMaterialBlock),
                }
            };
        }
        for (key, val) in block {
            match key.as_ref() {
                "name" => material_params.name = val,
                "mass" => fetch_data!(mass, val),
                "totalCrossSection" => fetch_data!(total_cross_section, val),
                "nIsotopes" => fetch_data!(n_isotopes, val),
                "nReactions" => fetch_data!(n_reactions, val),
                "sourceRate" => fetch_data!(source_rate, val),
                "scatteringCrossSection" => fetch_data!(scattering_cross_section, val),
                "absorptionCrossSection" => fetch_data!(absorption_cross_section, val),
                "fissionCrossSection" => fetch_data!(fission_cross_section, val),
                "scatteringCrossSectionRatio" => fetch_data!(scattering_cross_section_ratio, val),
                "absorptionCrossSectionRatio" => fetch_data!(absorbtion_cross_section_ratio, val),
                "fissionCrossSectionRatio" => fetch_data!(fission_cross_section_ratio, val),
                _ => return Err(InputError::BadMaterialBlock),
            }
        }

        Ok(material_params)
    }
}

impl<T: CustomFloat> Default for MaterialParameters<T> {
    fn default() -> Self {
        Self {
            name: Default::default(),
            mass: T::from_f32(1000.0).unwrap(),
            total_cross_section: T::one(),
            n_isotopes: 10,
            n_reactions: 9,
            source_rate: T::zero(),
            scattering_cross_section: Default::default(),
            absorption_cross_section: Default::default(),
            fission_cross_section: Default::default(),
            scattering_cross_section_ratio: T::one(),
            absorbtion_cross_section_ratio: T::one(),
            fission_cross_section_ratio: T::one(),
        }
    }
}

/// Structure used to describe a cross section, i.e. a probability density
/// representation.
#[derive(Debug)]
pub struct CrossSectionParameters<T: CustomFloat> {
    pub name: String,
    pub aa: T,
    pub bb: T,
    pub cc: T,
    pub dd: T,
    pub ee: T,
    pub nu_bar: T,
}

impl<T: CustomFloat> CrossSectionParameters<T> {
    /// Creates a [CrossSectionParameters] object using the [Block] passed as
    /// argument. Any field not specified in the block will have its default
    /// value as defined in the [Default] implementation. May return an error
    /// if the block isn't a proper CrossSection block, i.e.:
    /// - There is an unknown field
    /// - A value associated to a valid field is invalid
    /// In that case, the [CrossSectionParameters] object is scrapped instead of
    /// being returned as incomplete or potentially erroneous.
    pub fn from_block(block: Block) -> Result<Self, InputError> {
        let mut cross_section = Self::default();

        macro_rules! fetch_data {
            ($f: ident, $v: expr) => {
                cross_section.$f = match $v.parse() {
                    Ok(v) => v,
                    Err(_) => return Err(InputError::BadCrossSectionBlock),
                }
            };
        }

        for (key, val) in block {
            match key.as_ref() {
                "name" => cross_section.name = val,
                "A" => fetch_data!(aa, val),
                "B" => fetch_data!(bb, val),
                "C" => fetch_data!(cc, val),
                "D" => fetch_data!(dd, val),
                "E" => fetch_data!(ee, val),
                "nuBar" => fetch_data!(nu_bar, val),
                _ => return Err(InputError::BadCrossSectionBlock),
            }
        }
        Ok(cross_section)
    }
}

impl<T: CustomFloat> Default for CrossSectionParameters<T> {
    fn default() -> Self {
        Self {
            name: Default::default(),
            aa: T::zero(),
            bb: T::zero(),
            cc: T::zero(),
            dd: T::zero(),
            ee: T::one(),
            nu_bar: T::from_f32(2.4).unwrap(),
        }
    }
}

/// Structure encompassing all simulation parameters. In the program's
/// execution flow, it is first initialized using the CLI arguments,
/// then optionally updated with a specified input file.
#[derive(Debug)]
pub struct SimulationParameters<T: CustomFloat> {
    pub input_file: String,
    pub energy_spectrum: String,
    pub cross_sections_out: String,
    pub boundary_condition: String,
    pub load_balance: bool,
    pub cycle_timers: bool,
    pub debug_threads: bool,
    pub n_particles: u64,
    pub batch_size: u64,
    pub n_batches: u64,
    pub n_steps: usize,
    pub nx: usize,
    pub ny: usize,
    pub nz: usize,
    pub seed: u64,
    //x_dom: u32,
    //y_dom: u32,
    //z_dom: u32,
    pub dt: T,
    pub f_max: T,
    pub lx: T,
    pub ly: T,
    pub lz: T,
    pub e_min: T,
    pub e_max: T,
    pub n_groups: usize,
    pub low_weight_cutoff: T,
    pub balance_tally_replications: u32,
    pub flux_tally_replications: u32,
    pub cell_tally_replications: u32,
    pub coral_benchmark: bool,
}

impl<T: CustomFloat> SimulationParameters<T> {
    /// Initialize a [SimulationParameters] object using a Cli object created
    /// from the arguments supplied via command line.
    ///
    /// ```rust
    /// use clap::Parser;
    /// use fastiron::io_utils::Cli;
    /// use fastiron::parameters::SimulationParameters;
    ///
    ///
    /// let cli = Cli::parse_from("./fastiron -i somefile -c -l".split(' '));
    /// let simulation_params = SimulationParameters::<f64>::from_cli(&cli);
    /// // compare the structures...
    /// println!("{:#?}", cli);
    /// println!("{:#?}", simulation_params);
    ///
    /// ```
    pub fn from_cli(cli: &io_utils::Cli) -> Self {
        let mut simulation_params = Self::default();

        // use the cli to override defaults
        macro_rules! fetch_from_cli {
            ($f: ident) => {
                match &cli.$f {
                    Some(val) => simulation_params.$f = val.to_owned().into(),
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
        simulation_params.debug_threads = cli.debug_threads;
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

impl<T: CustomFloat> Default for SimulationParameters<T> {
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
            dt: T::from_f64(1e-8).unwrap(),
            f_max: T::from_f64(0.1).unwrap(),
            lx: T::from_f64(100.0).unwrap(),
            ly: T::from_f64(100.0).unwrap(),
            lz: T::from_f64(100.0).unwrap(),
            e_min: T::from_f64(1e-9).unwrap(),
            e_max: T::from_f64(20.0).unwrap(),
            n_groups: 230,
            low_weight_cutoff: T::from_f64(0.001).unwrap(),
            balance_tally_replications: 1,
            flux_tally_replications: 1,
            cell_tally_replications: 1,
            coral_benchmark: false,
        }
    }
}

/// Strucure encompassing all the problem's parameters. It is
/// created, completed and returned by the [get_parameters] method.
#[derive(Debug, Default)]
pub struct Parameters<T: CustomFloat> {
    /// Object used to store simulation parameters
    pub simulation_params: SimulationParameters<T>,
    /// List of geometries. See [GeometryParameters] for more.
    pub geometry_params: Vec<GeometryParameters<T>>,
    /// Map of materials. See [MaterialParameters] for more.
    pub material_params: HashMap<String, MaterialParameters<T>>,
    /// Map of cross sections. See [CrossSectionParameters] for more.
    pub cross_section_params: HashMap<String, CrossSectionParameters<T>>,
}

impl<T: CustomFloat> Parameters<T> {
    /// Update the object's [SimulationParameters] field using the [Block] passed
    /// as argument. May return an error if the block isn't a proper Simulation
    /// block, i.e.:
    /// - There is an unknown field
    /// - A value associated to a valid field is invalid
    pub fn update_simulation_parameters(&mut self, sim_block: Block) -> Result<(), InputError> {
        macro_rules! fetch_data {
            ($f: ident, $v: expr) => {
                self.simulation_params.$f = match $v.parse() {
                    Ok(v) => v,
                    Err(_) => return Err(InputError::BadSimulationBlock),
                }
            };
        }
        macro_rules! fetch_bool {
            ($f: ident, $v: expr) => {
                self.simulation_params.load_balance = match $v.as_ref() {
                    "0" => false,
                    "1" => true,
                    _ => return Err(InputError::BadSimulationBlock),
                }
            };
        }

        for (key, val) in sim_block {
            match key.as_ref() {
                "inputFile" => self.simulation_params.input_file = val,
                "energySpectrum" => self.simulation_params.energy_spectrum = val,
                "crossSectionsOut" => self.simulation_params.cross_sections_out = val,
                "boundaryCondition" => self.simulation_params.boundary_condition = val,

                "loadBalance" => fetch_bool!(load_balance, val),
                "cycleTimers" => fetch_bool!(cycle_timers, val),
                "debugThreads" => fetch_bool!(debug_threads, val),
                "coralBenchmark" => fetch_bool!(coral_benchmark, val),

                "nParticles" => fetch_data!(n_particles, val),
                "batchSize" => fetch_data!(batch_size, val),
                "nBatches" => fetch_data!(n_batches, val),
                "nSteps" => fetch_data!(n_steps, val),
                "nx" => fetch_data!(nx, val),
                "ny" => fetch_data!(ny, val),
                "nz" => fetch_data!(nz, val),
                "seed" => fetch_data!(seed, val),
                "dt" => fetch_data!(dt, val),
                "fMax" => fetch_data!(f_max, val),
                "lx" => fetch_data!(lx, val),
                "ly" => fetch_data!(ly, val),
                "lz" => fetch_data!(lz, val),
                "eMin" => fetch_data!(e_min, val),
                "eMax" => fetch_data!(e_max, val),
                "nGroups" => fetch_data!(n_groups, val),
                "lowWeightCutoff" => fetch_data!(low_weight_cutoff, val),
                "balanceTallyReplications" => fetch_data!(balance_tally_replications, val),
                "fluxTallyReplications" => fetch_data!(flux_tally_replications, val),
                "cellTallyReplications" => fetch_data!(cell_tally_replications, val),

                "xDom" | "yDom" | "zDom" => (),
                "mpiThreadMultiple" => (),
                _ => return Err(InputError::BadSimulationBlock),
            }
        }
        Ok(())
    }
    /// Add a new [GeometryParameters] object to the internal list.
    pub fn add_geometry_parameter(&mut self, some_geometry: GeometryParameters<T>) {
        self.geometry_params.push(some_geometry);
    }
    /// Add a new [MaterialParameters] object to the internal map.
    pub fn add_material_parameter(&mut self, some_material: MaterialParameters<T>) {
        self.material_params
            .insert(some_material.name.to_owned(), some_material);
    }
    pub fn add_cross_section_parameter(&mut self, cross_section: CrossSectionParameters<T>) {
        self.cross_section_params
            .insert(cross_section.name.to_owned(), cross_section);
    }
}
