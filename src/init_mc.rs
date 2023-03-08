use std::collections::HashMap;

use crate::{
    comm_object::CommObject,
    decomposition_object::DecompositionObject,
    global_fcc_grid::{GlobalFccGrid, Tuple3},
    material_database::{Isotope, Material},
    mc::{mc_domain::MCDomain, mc_rng_state::rng_sample, mc_vector::MCVector},
    mesh_partition::MeshPartition,
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
    if my_rank == 0 {
        println!("Starting consistency check");
    }

    domain.iter().enumerate().for_each(|(domain_idx, dd)| {
        dd.mesh
            .cell_connectivity
            .iter()
            .enumerate()
            .for_each(|(cell_idx, cc)| {
                cc.facet.iter().enumerate().for_each(|(facet_idx, ff)| {
                    let current = ff.subfacet.current;
                    println!(
                        "current.cell == cell_idx: {}",
                        current.cell.unwrap() == cell_idx
                    );

                    let adjacent = ff.subfacet.adjacent;
                    let domain_idx_adj = adjacent.domain.unwrap();
                    let cell_idx_adj = adjacent.cell.unwrap();
                    let facet_idx_adj = adjacent.facet.unwrap();
                    let backside = &domain[domain_idx_adj].mesh.cell_connectivity[cell_idx_adj]
                        .facet[facet_idx_adj]
                        .subfacet;

                    println!(
                        "backside.adjacent.domain == domain_idx: {}",
                        backside.adjacent.domain.unwrap() == domain_idx
                    );
                    println!(
                        "backside.adjacent.cell == cell_idx: {}",
                        backside.adjacent.cell.unwrap() == cell_idx
                    );
                    println!(
                        "backside.adjacent.facet == facet_idx: {}",
                        backside.adjacent.facet.unwrap() == facet_idx
                    );
                });
            });
    });

    if my_rank == 0 {
        println!("Finished consistency check");
    }
}

fn initialize_centers_rand<T: Float + FromPrimitive>(
    n_centers: usize,
    grid: &GlobalFccGrid<T>,
    seed: &mut u64,
) -> Vec<MCVector<T>> {
    // original function uses drand48 which sample a double in [0;1)
    // our rng_sample function does that
    // TODO: handle limit case when the sampled float eq 1
    let nx: T = FromPrimitive::from_usize(grid.nx).unwrap();
    let ny: T = FromPrimitive::from_usize(grid.ny).unwrap();
    let nz: T = FromPrimitive::from_usize(grid.nz).unwrap();
    let mut centers: Vec<MCVector<T>> = Vec::new();
    (0..n_centers).into_iter().for_each(|_| {
        let f1: T = rng_sample(seed);
        let f2: T = rng_sample(seed);
        let f3: T = rng_sample(seed);
        let tt: Tuple3 = (
            (f1 * nx).floor().to_usize().unwrap(),
            (f2 * ny).floor().to_usize().unwrap(),
            (f3 * nz).floor().to_usize().unwrap(),
        );
        let cell_idx = grid.cell_tuple_to_idx(&tt);
        centers.push(grid.cell_center(cell_idx));
    });

    centers
}

/*
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
*/

fn init_mesh<T: Float + FromPrimitive>(mcco: &mut MonteCarlo<T>, params: &Parameters) {
    let nx: usize = params.simulation_params.nx;
    let ny: usize = params.simulation_params.ny;
    let nz: usize = params.simulation_params.nz;

    let lx: T = FromPrimitive::from_f64(params.simulation_params.lx).unwrap();
    let ly: T = FromPrimitive::from_f64(params.simulation_params.ly).unwrap();
    let lz: T = FromPrimitive::from_f64(params.simulation_params.lz).unwrap();

    // fixed value for now, this is mpi related so it should be deleted
    // these values may be somewhat equivalent to no MPI usage?
    //let x_dom: usize = 0;
    //let y_dom: usize = 0;
    //let z_dom: usize = 0;
    let n_ranks: usize = 1;
    let n_domains_per_rank = 4; // why 4 in original code?
    let my_rank = 0;

    let ddc = DecompositionObject::new(my_rank, n_ranks, n_domains_per_rank);
    let my_domain_gids = ddc.assigned_gids.clone();
    let global_grid: GlobalFccGrid<T> = GlobalFccGrid::new(nx, ny, nz, lx, ly, lz);

    let n_centers: usize = n_domains_per_rank * n_ranks;
    // we fixed *_dom = 0, so for now we always initialize centers randomly
    let mut s = params.simulation_params.seed + 1; // use a seed dependant on sim seed
    let domain_centers = initialize_centers_rand(n_centers, &global_grid, &mut s);

    let mut partition: Vec<MeshPartition> = Vec::with_capacity(my_domain_gids.len());
    (0..my_domain_gids.len()).into_iter().for_each(|ii| {
        // my rank should be constant
        partition.push(MeshPartition::new(my_domain_gids[ii], ii, my_rank));
    });

    let mut comm: CommObject = CommObject::new(&partition);
    partition
        .iter_mut()
        .for_each(|mesh_p| mesh_p.build_mesh_partition(&global_grid, &domain_centers, &mut comm));

    mcco.domain.reserve(my_domain_gids.len());
    partition.iter().for_each(|mesh_p| {
        mcco.domain
            .push(MCDomain::new(mesh_p, &global_grid, &ddc, params))
    });

    if n_ranks == 1 {
        consistency_check(my_rank, &mcco.domain);
    }
}

fn init_tallies<T: Float + FromPrimitive>(mcco: &mut MonteCarlo<T>, params: &Parameters) {
    todo!()
}

pub fn check_cross_sections<T: Float + FromPrimitive>(mcco: &MonteCarlo<T>, params: &Parameters) {
    todo!()
}
