//! Parameters-related code
//!
//! The module holds all parameters related structure used during the
//! simulation. Input reading is located in the [`crate::utils`] module,
//! while parsing is done here.

use crate::{
    constants::CustomFloat,
    utils::io_utils::{parse_input_file, Cli, InputError},
};
use std::collections::HashMap;

/// Alias for a `<String, String>` [`HashMap`]. See here for detailed
/// structure of input blocks.
///
/// TODO: Add a breakdown of the input structure
pub type Block = HashMap<String, String>;

/// Enum used to categorize inconsistencies within parameters
#[derive(Debug, PartialEq)]
pub enum ParameterError {
    /// There are no specified geometries in the problem.
    NoGeometry,
    /// A [GeometryParameters] object has an undefined [Shape].
    UndefinedGeometry,
    /// There is a missing reference to a material; The string
    /// contains the name of the aforementioned material.
    MissingMaterial(String),
    /// There is a missing reference to a cross section; The string
    /// contains the name of the aforementionned cross section and
    /// the material refering to it.
    MissingCrossSection(String),
}

/// Enum used to run additional tests according to the input benchmark
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum BenchType {
    /// No additional tests are executed. This is the default mode.
    #[default]
    Standard,
    /// First configuration for the additionnal tests.
    Coral1,
    /// Second configuration for the additionnal tests.
    Coral2,
}

/// Enum used to describe a geometry's shape
#[derive(Debug, Default, PartialEq)]
pub enum Shape {
    /// Default value. Will result in errors if any geometries still
    /// hold this value at the end of initialization.
    #[default]
    Undefined,
    /// Brick-shaped geometry, i.e. a rectangular cuboid.
    Brick,
    /// Sphere-shaped geometry.
    Sphere,
}

/// Structure used to describe a geometry, i.e. a physical space of a
/// certain shape and certain material.
#[derive(Debug, Default)]
pub struct GeometryParameters<T: CustomFloat> {
    /// Name of the material the geometry is made of.
    pub material_name: String,
    /// Shape of the material. Note that this value defines which other fields are used:
    /// - A sphere-shaped geometry will only use radius and coordinates of the center.
    /// - A brick-shaped geometry will only use bounds on the axes.
    pub shape: Shape,
    /// Radius of a shere-shaped geometry.
    pub radius: T,
    /// x-coordinate of the center of a sphere-shaped geometry.
    pub x_center: T,
    /// y-coordinate of the center of a sphere-shaped geometry.
    pub y_center: T,
    /// z-coordinate of the center of a sphere-shaped geometry.
    pub z_center: T,
    /// Lower bound on the x-axis of a brick-shaped geometry.
    pub x_min: T,
    /// Lower bound on the y-axis of a brick-shaped geometry.
    pub y_min: T,
    /// Lower bound on the z-axis of a brick-shaped geometry.
    pub z_min: T,
    /// Upper bound on the x-axis of a brick-shaped geometry.
    pub x_max: T,
    /// Upper bound on the y-axis of a brick-shaped geometry.
    pub y_max: T,
    /// Upper bound on the z-axis of a brick-shaped geometry.
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

/// Struct used to describe a material, i.e. its name and relevant physical
/// properties.
#[derive(Debug)]
pub struct MaterialParameters<T: CustomFloat> {
    /// Name of the material.
    pub name: String,
    /// Mass of the material in grams.
    pub mass: T,
    /// Total value of the cross section.
    pub total_cross_section: T,
    /// Number of isotopes.
    pub n_isotopes: usize,
    /// Number of reactions.
    pub n_reactions: usize,
    /// Rate of particle sourcing.
    pub source_rate: T,
    /// Scattering reaction cross section name.
    pub scattering_cross_section: String,
    /// Absorption reaction cross section name.
    pub absorption_cross_section: String,
    /// Fission reaction cross section name.
    pub fission_cross_section: String,
    /// Scattering reaction cross section ratio i.e. its relative weight.
    pub scattering_cross_section_ratio: T,
    /// Absorption reaction cross section ratio i.e. its relative weight.
    pub absorbtion_cross_section_ratio: T,
    /// Fission reaction cross section ratio i.e. its relative weight.
    pub fission_cross_section_ratio: T,
}

impl<T: CustomFloat> MaterialParameters<T> {
    /// Creates a [MaterialParameters] object using the [Block] passed as
    /// argument. Any field not specified in the block will have its default
    /// value as defined in the [Default] implementation. May return an error
    /// if the block isn't a proper Material block, i.e.:
    /// - There is an unknown field.
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
            mass: T::from_f64(1000.0).unwrap(),
            total_cross_section: T::one(),
            n_isotopes: 10,
            n_reactions: 9,
            source_rate: Default::default(),
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
///
/// The probability density functions are represented using degree 4 polynomial
/// functions.
#[derive(Debug)]
pub struct CrossSectionParameters<T: CustomFloat> {
    /// Name of the cross section.
    pub name: String,
    /// Leading coefficient of the polynomial function.
    pub aa: T,
    /// Degree 3 coefficient of the polynomial function.
    pub bb: T,
    /// Degree 2 coefficient of the polynomial function.
    pub cc: T,
    /// Degree 1 coefficient of the polynomial function.
    pub dd: T,
    /// Degree 0 coefficient of the polynomial function.
    pub ee: T,
    /// Normalization value?
    pub nu_bar: T,
}

impl<T: CustomFloat> CrossSectionParameters<T> {
    /// Creates a [CrossSectionParameters] object using the [Block] passed as
    /// argument. Any field not specified in the block will have its default
    /// value as defined in the [Default] implementation. May return an error
    /// if the block isn't a proper CrossSection block, i.e.:
    /// - There is an unknown field.
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
            aa: Default::default(),
            bb: Default::default(),
            cc: Default::default(),
            dd: Default::default(),
            ee: T::one(),
            nu_bar: T::from_f32(2.4).unwrap(),
        }
    }
}

/// Structure holding all simulation parameters.
///
/// In the program's execution flow, it is first initialized using
/// the CLI arguments, then optionally updated with a specified input file.
#[derive(Debug)]
pub struct SimulationParameters<T: CustomFloat> {
    /// Path to the input file, it can be relative or absolute.
    pub input_file: String,
    /// Name of the output file the energy spectrum may be saved to.
    pub energy_spectrum: String,
    /// Name of the output file the cross sections may be saved to.
    pub cross_sections_out: String,
    /// Boundary conditions of the problem. Mesh is initialized according to this value.
    pub boundary_condition: String,
    /// Switch to enable or disable load balancing during execution.
    pub load_balance: bool,
    /// Switch used to print thread-debugging information. Currently unused.
    pub debug_threads: bool,
    /// Target number of particle for the simulation. Population will be controled
    /// according to this value.
    pub n_particles: u64,
    /// Number of threads that should be used to run the simulation.
    pub n_threads: u64,
    /// Number of steps simulated by the program.
    pub n_steps: usize,
    /// Number of cells along the x axis.
    pub nx: usize,
    /// Number of cells along the y axis.
    pub ny: usize,
    /// Number of cells along the z axis.
    pub nz: usize,
    /// Random number seed for the PRNG used by the simulation.
    pub seed: u64,
    /// Value of the time step in seconds.
    pub dt: T,
    /// Size of the simulation along the x axis.
    pub lx: T,
    /// Size of the simulation along the y axis.
    pub ly: T,
    /// Size of the simulation along the z axis.
    pub lz: T,
    /// Energy value of the lowest energy group.
    pub e_min: T,
    /// Energy value of the highest energy group.
    pub e_max: T,
    /// Number of energy groups to build a spectrum.
    pub n_groups: usize,
    /// Low statistical weight cutoff used for population control.
    pub low_weight_cutoff: T,
    /// Benchmark type of the input problem. See [BenchType] for more information.
    pub coral_benchmark: BenchType,
}

impl<T: CustomFloat> SimulationParameters<T> {
    /// Initialize a [SimulationParameters] object using a Cli object created
    /// from the arguments supplied via command line.
    ///
    /// ```rust
    /// use clap::Parser;
    /// use fastiron::utils::io_utils::Cli;
    /// use fastiron::parameters::SimulationParameters;
    ///
    ///
    /// let cli = Cli::parse_from("./fastiron -i somefile -c -l".split(' '));
    /// let simulation_params = SimulationParameters::<f64>::from_cli(&cli);
    /// // compare the structures...
    /// assert_eq!(cli.input_file.unwrap(), simulation_params.input_file);
    /// assert!(simulation_params.load_balance);
    /// ```
    pub fn from_cli(cli: &Cli) -> Self {
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
        simulation_params.load_balance = cli.load_balance;
        simulation_params.debug_threads = cli.debug_threads;
        fetch_from_cli!(lx);
        fetch_from_cli!(ly);
        fetch_from_cli!(lz);
        fetch_from_cli!(n_particles);
        fetch_from_cli!(n_threads);
        fetch_from_cli!(n_steps);
        fetch_from_cli!(nx);
        fetch_from_cli!(ny);
        fetch_from_cli!(nz);
        fetch_from_cli!(seed);

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
            debug_threads: false,
            n_particles: 1000000,
            n_threads: 1,
            n_steps: 10,
            nx: 10,
            ny: 10,
            nz: 10,
            seed: 1029384756,
            dt: T::from_f64(1e-8).unwrap(),
            lx: T::from_f64(100.0).unwrap(),
            ly: T::from_f64(100.0).unwrap(),
            lz: T::from_f64(100.0).unwrap(),
            e_min: T::from_f64(1e-9).unwrap(),
            e_max: T::from_f64(20.0).unwrap(),
            n_groups: 230,
            low_weight_cutoff: T::from_f64(0.001).unwrap(),
            coral_benchmark: BenchType::Standard,
        }
    }
}

/// Structure holding all the problem's parameters.
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
    /// Use the cli arguments to initialize parameters of the simulation, complete the
    /// structure and return it. The function will fail if:
    /// - it cannot read or find the specified input_file (if specified)
    /// - the resulting [Parameters] object is compromised
    pub fn get_parameters(cli: Cli) -> Result<Self, Vec<InputError>> {
        // structs init
        let mut params = Self {
            simulation_params: SimulationParameters::from_cli(&cli),
            geometry_params: Vec::new(),
            material_params: HashMap::new(),
            cross_section_params: HashMap::new(),
        };

        if let Some(filename) = cli.input_file {
            parse_input_file(filename, &mut params)?
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
                self.simulation_params.$f = match $v {
                    '0' => false,
                    '1' => true,
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
                "loadBalance" => {
                    let chars: Vec<char> = val.chars().collect();
                    fetch_bool!(load_balance, chars[0]);
                }
                "debugThreads" => {
                    let chars: Vec<char> = val.chars().collect();
                    fetch_bool!(debug_threads, chars[0]);
                }
                "coralBenchmark" => {
                    let chars: Vec<char> = val.chars().collect();
                    self.simulation_params.coral_benchmark = match chars[0] {
                        '0' => BenchType::Standard,
                        '1' => BenchType::Coral1,
                        '2' => BenchType::Coral2,
                        _ => return Err(InputError::BadSimulationBlock),
                    }
                }
                "nParticles" => fetch_data!(n_particles, val),
                "nSteps" => fetch_data!(n_steps, val),
                "nx" => fetch_data!(nx, val),
                "ny" => fetch_data!(ny, val),
                "nz" => fetch_data!(nz, val),
                "seed" => fetch_data!(seed, val),
                "dt" => fetch_data!(dt, val),
                "lx" => fetch_data!(lx, val),
                "ly" => fetch_data!(ly, val),
                "lz" => fetch_data!(lz, val),
                "eMin" => fetch_data!(e_min, val),
                "eMax" => fetch_data!(e_max, val),
                "nGroups" => fetch_data!(n_groups, val),
                "lowWeightCutoff" => fetch_data!(low_weight_cutoff, val),

                // Unused in fastiron;
                "cycleTimers" => (),
                "batchSize" | "nBatches" => (),
                "balanceTallyReplications" | "bTally" => (),
                "fluxTallyReplications" | "fTally" => (),
                "cellTallyReplications" | "cTally" => (),
                "xDom" | "yDom" | "zDom" | "fMax" => (),
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
    /// Add a new [CrossSectionParameters] object to the internal map.
    pub fn add_cross_section_parameter(&mut self, cross_section: CrossSectionParameters<T>) {
        self.cross_section_params
            .insert(cross_section.name.to_owned(), cross_section);
    }
}
