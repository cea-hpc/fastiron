use fastiron::{
    io_utils::{parse_input_file, Cli, InputError},
    parameters::{check_parameters_integrity, supply_defaults, ParameterError, Parameters},
};

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
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
