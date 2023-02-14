use fastiron::{
    io_utils::{parse_input_file, Cli},
    parameters::Parameters,
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
    parse_input_file("input_files/hg7.inp".to_string(), &mut params).unwrap();
}
