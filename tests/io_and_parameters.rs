use fastiron::{
    io_utils::{parse_input_file, Cli, InputError},
    parameters::{
        check_parameters_integrity, supply_defaults, ParameterError, Parameters,
        SimulationParameters,
    },
};

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}

#[test]
fn verify_cli_parsing() {
    use clap::Parser;
    let cmd_line = "./fastiron -i somefile.inp -c -l -e out1 -S out2.dat -X 10 -Y 10 -Z 10.0 -s 123456 -n 1000 -b 10";
    let cli = Cli::parse_from(cmd_line.split(' '));
    let simulation_params = SimulationParameters::from_cli(&cli);
    assert_eq!(simulation_params.input_file, "somefile.inp");
    assert!(simulation_params.cycle_timers);
    assert!(simulation_params.load_balance);
    assert!(!simulation_params.debug_threads);
    assert_eq!(simulation_params.energy_spectrum, "out1");
    assert_eq!(simulation_params.cross_sections_out, "out2.dat");
    assert_eq!(simulation_params.lx, 10.0);
    assert_eq!(simulation_params.ly, 10.0);
    assert_eq!(simulation_params.lz, 10.0);
    assert_eq!(simulation_params.seed, 123456);
    assert_eq!(simulation_params.n_particles, 1000);
    assert_eq!(simulation_params.n_batches, 10);
}

#[test]
fn verify_input_file_compatibility() {
    let mut params = Parameters::default();
    // we just verify that the file could be parsed, not if it was parsed correctly
    parse_input_file("input_files/debug/homogeneous.inp".to_string(), &mut params).unwrap();
}

#[test]
fn missing_input_file() {
    let mut params = Parameters::default();
    assert_eq!(
        parse_input_file("input_files/do_not_exist.inp".to_string(), &mut params),
        Err(InputError::BadInputFile)
    );
}

#[test]
fn missing_cross_section() {
    let mut params = Parameters::default();
    parse_input_file(
        "input_files/debug/missing_cross_section.inp".to_string(),
        &mut params,
    )
    .unwrap();
    // check if the missing reference was noticed by the function
    if let Err(v) = check_parameters_integrity(&params) {
        assert!(v.contains(&ParameterError::MissingCrossSection(
            "sourceMaterial:flat".to_string()
        )));
    } else {
        unreachable!()
    }
}

#[test]
fn missing_material() {
    let mut params = Parameters::default();
    parse_input_file(
        "input_files/debug/missing_material.inp".to_string(),
        &mut params,
    )
    .unwrap();
    // check if the missing reference was noticed by the function
    if let Err(v) = check_parameters_integrity(&params) {
        assert!(v.contains(&ParameterError::MissingMaterial(
            "sourceMaterial".to_string()
        )));
    } else {
        unreachable!()
    }
}

#[test]
fn no_geometry_supplied() {
    let mut params = Parameters::default();
    // parse a file with no geometry block
    parse_input_file(
        "input_files/debug/sim_block_only.inp".to_string(),
        &mut params,
    )
    .unwrap();
    // check if a NoGeometry error was noticed by the function
    if let Err(v) = check_parameters_integrity(&params) {
        assert!(v.contains(&ParameterError::NoGeometry));
    } else {
        unreachable!()
    }
    // call supply_defaults and check that a default geometry was indeed supplied
    supply_defaults(&mut params);
    check_parameters_integrity(&params).unwrap();
}
