use std::{collections::HashMap, fmt::Debug, fs::File, io::Write};

use crate::{
    constants::{sim::TINY_FLOAT, CustomFloat, Tuple3},
    data::{
        material_database::{Isotope, Material},
        mc_vector::MCVector,
        nuclear_data::{NuclearData, Polynomial, ReactionType},
    },
    geometry::{
        global_fcc_grid::GlobalFccGrid, mc_domain::MCDomain, mesh_partition::MeshPartition,
    },
    montecarlo::MonteCarlo,
    parameters::Parameters,
    particles::particle_container::ParticleContainer,
    utils::{
        comm_object::CommObject, decomposition_object::DecompositionObject,
        mc_processor_info::MCProcessorInfo, mc_rng_state::rng_sample,
    },
};
use num::{one, zero, Float, FromPrimitive};

/// Creates the correct number of particle containers for simulation.
///
/// The correct number is determined according to simulation parameters & execution policy.
pub fn init_particle_containers<T: CustomFloat>(
    params: &Parameters<T>,
    proc_info: &MCProcessorInfo, // may be removed if we add it to parameters
) -> Vec<ParticleContainer<T>> {
    // compute the capacities using number of threads, target number of particles & fission statistical offset
    let target_n_particles = params.simulation_params.n_particles as usize;

    let regular_capacity_per_container = target_n_particles / proc_info.num_threads; // equivalent of batch size
    let regular_capacity = regular_capacity_per_container + regular_capacity_per_container / 10; // approximate 10% margin

    let max_nu_bar: usize = params
        .material_params
        .values()
        .map(|mp| {
            params.cross_section_params[&mp.fission_cross_section]
                .nu_bar
                .ceil()
                .to_usize()
                .unwrap()
        })
        .max()
        .unwrap();

    let extra_capacity = regular_capacity_per_container * max_nu_bar.max(2);

    let container = ParticleContainer::new(regular_capacity, extra_capacity);
    vec![container; proc_info.num_threads]
}

/// Creates a [MonteCarlo] object using the specified parameters.
pub fn init_mc<T: CustomFloat>(params: Parameters<T>) -> MonteCarlo<T> {
    let mut mcco: MonteCarlo<T> = MonteCarlo::new(params);

    init_proc_info(&mut mcco);
    init_time_info(&mut mcco);
    init_nuclear_data(&mut mcco);
    init_mesh(&mut mcco);
    init_tallies(&mut mcco);

    check_cross_sections(&mcco);

    mcco
}

//==================
// Private functions
//==================

fn init_proc_info<T: CustomFloat>(_mcco: &mut MonteCarlo<T>) {}

fn init_time_info<T: CustomFloat>(mcco: &mut MonteCarlo<T>) {
    let params = &mcco.params;
    mcco.time_info.time_step = params.simulation_params.dt;
}

fn init_nuclear_data<T: CustomFloat>(mcco: &mut MonteCarlo<T>) {
    let params = &mcco.params;
    let energy_low: T = params.simulation_params.e_min;
    let energy_high: T = params.simulation_params.e_max;
    mcco.nuclear_data =
        NuclearData::new(params.simulation_params.n_groups, energy_low, energy_high);

    let mut cross_section: HashMap<String, Polynomial<T>> = Default::default();
    for xs_params in params.cross_section_params.values() {
        cross_section.insert(
            xs_params.name.to_owned(),
            Polynomial {
                aa: xs_params.aa,
                bb: xs_params.bb,
                cc: xs_params.cc,
                dd: xs_params.dd,
                ee: xs_params.ee,
            },
        );
    }

    let mut n_isotopes: usize = 0;
    let n_materials: usize = params.material_params.len();

    for mat_params in params.material_params.values() {
        n_isotopes += mat_params.n_isotopes;
    }

    mcco.nuclear_data.isotopes.reserve(n_isotopes);
    mcco.material_database.mat.reserve(n_materials);

    for mp in params.material_params.values() {
        let mut material: Material<T> = Material {
            name: mp.name.to_owned(),
            mass: mp.mass,
            iso: Vec::with_capacity(mp.n_isotopes),
        };

        (0..mp.n_isotopes).for_each(|_| {
            let isotope_gid = mcco.nuclear_data.add_isotope(
                &cross_section,
                mp,
                params.cross_section_params[&mp.fission_cross_section].nu_bar,
            );
            // All isotopes are equally prevalent => each weights 1/n_isotopes
            material.add_isotope(Isotope {
                gid: isotope_gid,
                atom_fraction: one::<T>() / FromPrimitive::from_usize(mp.n_isotopes).unwrap(),
            })
        });
        mcco.material_database.add_material(material);
    }
}

/// Check the consistency of the domain list passed as argument.
///
/// This function goes through the given domain list and check for inconsistencies
/// by checking adjacencies coherence.
pub fn consistency_check<T: CustomFloat>(my_rank: usize, domain: &[MCDomain<T>]) {
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
                    assert_eq!(current.cell.unwrap(), cell_idx);
                    let adjacent = ff.subfacet.adjacent;
                    // These can hold none as a correct value e.g. if the current cell is on the border of the problem
                    if adjacent.domain.is_some()
                        & adjacent.cell.is_some()
                        & adjacent.facet.is_some()
                    {
                        let domain_idx_adj = adjacent.domain.unwrap();
                        let cell_idx_adj = adjacent.cell.unwrap();
                        let facet_idx_adj = adjacent.facet.unwrap();
                        let backside = &domain[domain_idx_adj].mesh.cell_connectivity[cell_idx_adj]
                            .facet[facet_idx_adj]
                            .subfacet;

                        assert!(
                            (backside.adjacent.domain.unwrap() == domain_idx)
                                & (backside.adjacent.cell.unwrap() == cell_idx)
                                & (backside.adjacent.facet.unwrap() == facet_idx)
                        )
                    }
                });
            });
    });

    if my_rank == 0 {
        println!("Finished consistency check");
    }
}

fn initialize_centers_rand<T: CustomFloat>(
    n_centers: usize,
    grid: &GlobalFccGrid<T>,
    seed: &mut u64,
) -> Vec<MCVector<T>> {
    // original function uses drand48 which sample a double in [0; 1[
    // our rng_sample function does that
    let nx: T = FromPrimitive::from_usize(grid.nx).unwrap();
    let ny: T = FromPrimitive::from_usize(grid.ny).unwrap();
    let nz: T = FromPrimitive::from_usize(grid.nz).unwrap();
    let mut centers: Vec<MCVector<T>> = Vec::new();
    (0..n_centers).for_each(|_| {
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

fn init_mesh<T: CustomFloat>(mcco: &mut MonteCarlo<T>) {
    let params = &mcco.params;

    let nx: usize = params.simulation_params.nx;
    let ny: usize = params.simulation_params.ny;
    let nz: usize = params.simulation_params.nz;

    let lx: T = params.simulation_params.lx;
    let ly: T = params.simulation_params.ly;
    let lz: T = params.simulation_params.lz;

    // these values may be somewhat equivalent to no MPI usage
    let n_ranks: usize = 1;
    let n_domains_per_rank = 4; // why 4 in original code?
    let my_rank = 0;

    let ddc = DecompositionObject::new(my_rank, n_ranks, n_domains_per_rank);
    let my_domain_gids = ddc.assigned_gids.clone();
    let global_grid: GlobalFccGrid<T> = GlobalFccGrid::new(nx, ny, nz, lx, ly, lz);

    // initialize centers randomly
    let n_centers: usize = n_domains_per_rank * n_ranks;
    let mut s = params.simulation_params.seed + 1; // use a seed dependant on sim seed
    let domain_centers = initialize_centers_rand(n_centers, &global_grid, &mut s);

    let mut partition: Vec<MeshPartition> = Vec::with_capacity(my_domain_gids.len());
    (0..my_domain_gids.len()).for_each(|ii| {
        // my rank should be constant
        partition.push(MeshPartition::new(my_domain_gids[ii], ii, my_rank));
    });

    let mut comm: CommObject = CommObject::new(&partition);
    // indexing should be coherent since we cloned partition in comm's construction
    // this loop has to be done using indexes because of the way we init the mesh
    // the main init function is used on the current indexed partition but others
    // may be accessed to process neighboring cells, so no borrow allowed
    (0..comm.partition.len()).for_each(|mesh_p_idx| {
        let remote_cells =
            comm.partition[mesh_p_idx].build_mesh_partition(&global_grid, &domain_centers);

        // replace the send call originally in build_cell_idx_map
        for (remote_domain_gid, cell_gid) in &remote_cells {
            let cell_to_send = *comm.partition[mesh_p_idx]
                .cell_info_map
                .get(cell_gid)
                .unwrap();
            let target_partition = &mut comm.partition[comm.gid_to_idx[*remote_domain_gid]];
            assert!(cell_to_send.domain_index.is_some());
            assert!(cell_to_send.cell_index.is_some());
            target_partition
                .cell_info_map
                .insert(*cell_gid, cell_to_send);
        }
    });

    mcco.domain.reserve(my_domain_gids.len());
    comm.partition.iter().for_each(|mesh_p| {
        mcco.domain.push(MCDomain::new(
            mesh_p,
            &global_grid,
            &ddc,
            params,
            &mcco.material_database,
        ))
    });

    if n_ranks == 1 {
        consistency_check(my_rank, &mcco.domain);
    }
}

fn init_tallies<T: CustomFloat>(mcco: &mut MonteCarlo<T>) {
    let params = &mcco.params;
    mcco.tallies.initialize_tallies(
        &mcco.domain,
        params.simulation_params.n_groups,
        params.simulation_params.balance_tally_replications,
        params.simulation_params.flux_tally_replications,
        params.simulation_params.cell_tally_replications,
    )
}

#[derive(Debug, Clone, Default)]
struct XSData<T: Float> {
    abs: T,
    fis: T,
    sca: T,
}

/// Prints cross-section data of the problem.
///
/// TODO: add a model of the produced output
pub fn check_cross_sections<T: CustomFloat>(mcco: &MonteCarlo<T>) {
    let params = &mcco.params;
    if params.simulation_params.cross_sections_out.is_empty() {
        return;
    }

    let nucdb = &mcco.nuclear_data;
    let matdb = &mcco.material_database;

    let n_groups = params.simulation_params.n_groups;

    let mut energies: Vec<T> = Vec::with_capacity(n_groups);
    for ii in 0..n_groups {
        energies.push((nucdb.energies[ii] + nucdb.energies[ii + 1]) / (one::<T>() + one()));
    }

    // compute
    let mut xc_table: HashMap<String, Vec<XSData<T>>> = Default::default();
    // for each material
    matdb.mat.iter().for_each(|material| {
        let mat_name = material.name.to_owned();
        let mut xc_vec: Vec<XSData<T>> = vec![XSData::default(); n_groups];
        let n_isotopes: T = FromPrimitive::from_usize(material.iso.len()).unwrap();
        // for each isotope
        material.iso.iter().for_each(|isotope| {
            // for each reaction
            nucdb.isotopes[isotope.gid][0]
                .reactions
                .iter()
                .for_each(|reaction| {
                    // for each energy group
                    (0..n_groups).for_each(|group_idx| match reaction.reaction_type {
                        ReactionType::Scatter => {
                            xc_vec[group_idx].sca += reaction.cross_section[group_idx] / n_isotopes;
                        }
                        ReactionType::Absorption => {
                            xc_vec[group_idx].abs += reaction.cross_section[group_idx] / n_isotopes;
                        }
                        ReactionType::Fission => {
                            xc_vec[group_idx].fis += reaction.cross_section[group_idx] / n_isotopes;
                        }
                    });
                });
        });
        xc_table.insert(mat_name, xc_vec);
    });

    // build an output file
    let file_name = params.simulation_params.cross_sections_out.to_owned() + ".dat";
    let mut file = File::create(file_name).unwrap();
    // header
    write!(file, "group |           energy |  ").unwrap();
    xc_table.iter().for_each(|(mat_name, _)| {
        write!(
            file,
            "{mat_name}_absorb |  {mat_name}_fission |  {mat_name}_scatter"
        )
        .unwrap();
    });
    writeln!(file).unwrap();
    // data
    (0..n_groups).for_each(|ii| {
        write!(file, "{:>5} |  {:>15.12} |   ", ii, energies[ii]).unwrap();
        xc_table.values_mut().for_each(|xc_vec| {
            if xc_vec[ii].abs < FromPrimitive::from_f64(TINY_FLOAT).unwrap() {
                xc_vec[ii].abs = zero();
            }
            if xc_vec[ii].fis < FromPrimitive::from_f64(TINY_FLOAT).unwrap() {
                xc_vec[ii].fis = zero();
            }
            if xc_vec[ii].sca < FromPrimitive::from_f64(TINY_FLOAT).unwrap() {
                xc_vec[ii].sca = zero();
            }
            write!(
                file,
                "{} |  {:>22} |  {:>22}",
                xc_vec[ii].abs, xc_vec[ii].fis, xc_vec[ii].sca
            )
            .unwrap();
        });
        writeln!(file).unwrap()
    });
}