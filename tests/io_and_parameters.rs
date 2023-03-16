use fastiron::{
    io_utils::{parse_input_file, Cli, InputError},
    parameters::{
        check_parameters_integrity, supply_defaults, GeometryParameters, ParameterError,
        Parameters, Shape, SimulationParameters,
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
    let simulation_params = SimulationParameters::<f64>::from_cli(&cli);
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
    let mut params = Parameters::<f64>::default();
    // we just verify that the file could be parsed, not if it was parsed correctly
    parse_input_file("input_files/debug/homogeneous.inp".to_string(), &mut params).unwrap();
}

#[test]
fn missing_input_file() {
    let mut params = Parameters::<f64>::default();
    if let Err(v) = parse_input_file("input_files/do_not_exist.inp".to_string(), &mut params) {
        assert!(v.contains(&InputError::BadInputFile));
    } else {
        unreachable!()
    }
}

#[test]
fn missing_cross_section() {
    let mut params = Parameters::<f64>::default();
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
    let mut params = Parameters::<f64>::default();
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
    let mut params = Parameters::<f64>::default();
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

#[test]
fn verify_file_parsing() {
    let mut params = Parameters::default();
    parse_input_file("input_files/debug/parsing.inp".to_string(), &mut params).unwrap();
    // check for obvious issues
    check_parameters_integrity(&params).unwrap();
    // lots of assert!
    // sim block
    assert_eq!(params.simulation_params.dt, 1e-06);
    assert_eq!(params.simulation_params.f_max, 0.1);
    assert_eq!(params.simulation_params.input_file, "parsing.inp");
    assert_eq!(params.simulation_params.boundary_condition, "reflect");
    assert!(!params.simulation_params.load_balance);
    assert!(!params.simulation_params.cycle_timers);
    assert!(!params.simulation_params.debug_threads);
    assert_eq!(params.simulation_params.n_steps, 10);
    assert_eq!(params.simulation_params.seed, 1029384756);
    assert_eq!(params.simulation_params.e_max, 20.0);
    assert_eq!(params.simulation_params.e_min, 1e-09);
    assert_eq!(params.simulation_params.n_groups, 230);
    assert_eq!(params.simulation_params.low_weight_cutoff, 0.001);

    // geometry blocks
    fn match_a_block(g: &GeometryParameters<f64>) -> bool {
        let match_b1: bool = (g.material_name == "sourceMaterial")
            & matches!(g.shape, Shape::Brick)
            & (g.x_max == 1000.0)
            & (g.x_min == 0.0)
            & (g.y_max == 1000.0)
            & (g.y_min == 0.0)
            & (g.z_max == 1000.0)
            & (g.z_min == 0.0);

        let match_b2: bool = (g.material_name == "mat1")
            & matches!(g.shape, Shape::Brick)
            & (g.x_max == 2000.0)
            & (g.x_min == 1000.0)
            & (g.y_max == 1000.0)
            & (g.y_min == 0.0)
            & (g.z_max == 1000.0)
            & (g.z_min == 0.0);

        match_b1 | match_b2
    }
    for g in params.geometry_params {
        assert!(match_a_block(&g));
    }

    // material blocks
    for (key, val) in params.material_params {
        match key.as_ref() {
            "sourceMaterial" => {
                assert_eq!(val.name, key);
                assert_eq!(val.mass, 12.011);
                assert_eq!(val.n_isotopes, 10);
                assert_eq!(val.n_reactions, 9);
                assert_eq!(val.source_rate, 1e+10);
                assert_eq!(val.total_cross_section, 0.1);
                assert_eq!(val.absorption_cross_section, "flat");
                assert_eq!(val.fission_cross_section, "flat");
                assert_eq!(val.scattering_cross_section, "flat");
                assert_eq!(val.absorbtion_cross_section_ratio, 0.1086);
                assert_eq!(val.fission_cross_section_ratio, 0.0969);
                assert_eq!(val.scattering_cross_section_ratio, 0.7946);
            }
            "mat1" => {
                assert_eq!(val.name, key);
                assert_eq!(val.mass, 12.011);
                assert_eq!(val.n_isotopes, 10);
                assert_eq!(val.n_reactions, 9);
                assert_eq!(val.source_rate, 1e+10);
                assert_eq!(val.total_cross_section, 0.1);
                assert_eq!(val.absorption_cross_section, "absorb");
                assert_eq!(val.fission_cross_section, "fission");
                assert_eq!(val.scattering_cross_section, "scatter");
                assert_eq!(val.absorbtion_cross_section_ratio, 0.1086);
                assert_eq!(val.fission_cross_section_ratio, 0.0969);
                assert_eq!(val.scattering_cross_section_ratio, 0.7946);
            }
            _ => panic!(),
        }
    }

    // cross section blocks
    for (key, val) in params.cross_section_params {
        match key.as_ref() {
            "flat" => {
                assert_eq!(val.name, key);
                assert_eq!(val.aa, 0.0);
                assert_eq!(val.bb, 0.0);
                assert_eq!(val.cc, 0.0);
                assert_eq!(val.dd, 0.0);
                assert_eq!(val.ee, 1.0);
                assert_eq!(val.nu_bar, 1.0);
            }
            "absorb" => {
                assert_eq!(val.name, key);
                assert_eq!(val.aa, 0.0);
                assert_eq!(val.bb, 0.0);
                assert_eq!(val.cc, -0.0);
                assert_eq!(val.dd, -0.8446);
                assert_eq!(val.ee, -0.5243);
                assert_eq!(val.nu_bar, -2.22);
            }
            "fission" => {
                assert_eq!(val.name, key);
                assert_eq!(val.aa, 0.0);
                assert_eq!(val.bb, 0.0);
                assert_eq!(val.cc, 0.0);
                assert_eq!(val.dd, -0.342);
                assert_eq!(val.ee, 0.0);
                assert_eq!(val.nu_bar, 2.4);
            }
            "scatter" => {
                assert_eq!(val.name, key);
                assert_eq!(val.aa, 0.0);
                assert_eq!(val.bb, 0.0);
                assert_eq!(val.cc, 0.0);
                assert_eq!(val.dd, 0.0);
                assert_eq!(val.ee, 0.7);
            }
            _ => panic!(),
        }
    }
}
