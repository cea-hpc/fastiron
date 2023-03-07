use std::collections::HashMap;

use crate::{
    global_fcc_grid::GlobalFccGrid,
    material_database::{Isotope, Material},
    mc::{mc_domain::MCDomain, mc_vector::MCVector},
    montecarlo::MonteCarlo,
    nuclear_data::{NuclearData, Polynomial},
    parameters::Parameters,
};
use num::{one, Float, FromPrimitive};

/// Creates a [MonteCarlo] object using the specified parameters.
pub fn init_mc<T: Float + FromPrimitive>(params: &Parameters) -> MonteCarlo<T> {
    let mut mcco: MonteCarlo<T> = MonteCarlo::new(params);

    init_proc_info(&mut mcco);
    init_time_info(&mut mcco, params);
    init_nuclear_data(&mut mcco, params);
    init_mesh(&mut mcco, params);
    init_tallies(&mut mcco, params);

    //check_cross_sections(&mcco, params);

    mcco
}

fn init_proc_info<T: Float + FromPrimitive>(_mcco: &mut MonteCarlo<T>) {}

fn init_time_info<T: Float + FromPrimitive>(mcco: &mut MonteCarlo<T>, params: &Parameters) {
    mcco.time_info.time_step = FromPrimitive::from_f64(params.simulation_params.dt).unwrap();
}

fn init_nuclear_data<T: Float + FromPrimitive>(mcco: &mut MonteCarlo<T>, params: &Parameters) {
    let energy_low: T = FromPrimitive::from_f64(params.simulation_params.e_min).unwrap();
    let energy_high: T = FromPrimitive::from_f64(params.simulation_params.e_max).unwrap();
    mcco.nuclear_data =
        NuclearData::new(params.simulation_params.n_groups, energy_low, energy_high);
    //mcco.material_database = MaterialDatabase::default(); // will already be done in the mcco constructor

    let mut cross_section: HashMap<String, Polynomial<T>> = Default::default();
    for xs_params in params.cross_section_params.values() {
        let aa = FromPrimitive::from_f64(xs_params.aa).unwrap();
        let bb = FromPrimitive::from_f64(xs_params.bb).unwrap();
        let cc = FromPrimitive::from_f64(xs_params.cc).unwrap();
        let dd = FromPrimitive::from_f64(xs_params.dd).unwrap();
        let ee = FromPrimitive::from_f64(xs_params.ee).unwrap();
        cross_section.insert(xs_params.name.to_owned(), Polynomial { aa, bb, cc, dd, ee });
    }

    let mut n_isotopes: usize = 0;
    let n_materials: usize = params.material_params.len();

    for mat_params in params.material_params.values() {
        n_isotopes += mat_params.n_isotopes;
    }

    // These should be of capacity 0 by default, using directly the count is correct
    mcco.nuclear_data.isotopes.reserve(n_isotopes);
    mcco.material_database.mat.reserve(n_materials);

    for mp in params.material_params.values() {
        let mut material: Material<T> = Material {
            name: mp.name.to_owned(),
            mass: FromPrimitive::from_f64(mp.mass).unwrap(),
            ..Default::default()
        };
        let nu_bar: T =
            FromPrimitive::from_f64(params.cross_section_params[&mp.fission_cross_section].nu_bar)
                .unwrap();
        material.iso.reserve(mp.n_isotopes);

        (0..mp.n_isotopes).into_iter().for_each(|_| {
            let isotope_gid = mcco.nuclear_data.add_isotope(
                mp.n_reactions,
                &cross_section[&mp.fission_cross_section],
                &cross_section[&mp.scattering_cross_section],
                &cross_section[&mp.absorption_cross_section],
                nu_bar,
                FromPrimitive::from_f64(mp.total_cross_section).unwrap(),
                FromPrimitive::from_f64(mp.fission_cross_section_ratio).unwrap(),
                FromPrimitive::from_f64(mp.scattering_cross_section_ratio).unwrap(),
                FromPrimitive::from_f64(mp.absorbtion_cross_section_ratio).unwrap(),
            );
            // All isotopes are equally prevalent => each weight 1/n_isotopes
            material.add_isotope(Isotope {
                gid: isotope_gid,
                atom_fraction: one::<T>() / FromPrimitive::from_usize(mp.n_isotopes).unwrap(),
            })
        });
        mcco.material_database.add_material(material);
    }
}

fn consistency_check<T: Float>(my_rank: usize, domain: &[MCDomain<T>]) {
    todo!()
}

fn initialize_centers_rand<T: Float>(
    n_centers: usize,
    grid: &GlobalFccGrid<T>,
) -> Vec<MCVector<T>> {
    todo!()
}

fn initialize_centers_grid<T: Float>(
    lx: T,
    ly: T,
    lz: T,
    x_dom: usize,
    y_dom: usize,
    z_dom: usize,
) -> Vec<MCVector<T>> {
    todo!()
}

fn init_mesh<T: Float + FromPrimitive>(mcco: &mut MonteCarlo<T>, params: &Parameters) {
    todo!()
}

fn init_tallies<T: Float + FromPrimitive>(mcco: &mut MonteCarlo<T>, params: &Parameters) {
    todo!()
}

pub fn check_cross_sections<T: Float + FromPrimitive>(mcco: &MonteCarlo<T>, params: &Parameters) {
    todo!()
}
