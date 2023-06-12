//! Initialization code for the problem

use std::{collections::HashMap, fmt::Debug, fs::File, io::Write};

use crate::{
    constants::{CustomFloat, Tuple3},
    data::{
        material_database::{Isotope, Material},
        mc_vector::MCVector,
        nuclear_data::{NuclearData, Polynomial, ReactionType},
    },
    geometry::{
        global_fcc_grid::GlobalFccGrid, mc_domain::MCDomain, mesh_partition::MeshPartition,
    },
    montecarlo::{MonteCarloData, MonteCarloResults, MonteCarloUnit},
    parameters::Parameters,
    particles::particle_container::ParticleContainer,
    utils::{
        comm_object::CommObject,
        decomposition_object::DecompositionObject,
        mc_processor_info::{ExecPolicy, MCProcessorInfo},
        mc_rng_state::rng_sample,
    },
};
use atomic::Atomic;
use num::{one, zero, Float, FromPrimitive};

/// Creates a [MonteCarloData] object using the specified parameters.
pub fn init_mcdata<T: CustomFloat>(params: Parameters<T>) -> MonteCarloData<T> {
    let mut mcdata: MonteCarloData<T> = MonteCarloData::new(params);

    println!("  [MonteCarloData Initialization]: Start");
    init_nuclear_data(&mut mcdata);

    if !mcdata
        .params
        .simulation_params
        .cross_sections_out
        .is_empty()
    {
        check_cross_sections(&mcdata);
    }
    println!("  [MonteCarloData Initialization]: Done");

    mcdata
}

/// Creates the correct number of particle containers for simulation.
///
/// The correct number is determined according to simulation parameters & execution policy.
pub fn init_particle_containers<T: CustomFloat>(
    params: &Parameters<T>,
    proc_info: &MCProcessorInfo, // may be removed if we add it to parameters
) -> Vec<ParticleContainer<T>> {
    println!("  [ParticleContainer Initialization]: Start");
    // compute the capacities using number of threads, target number of particles & fission statistical offset
    let target_n_particles = params.simulation_params.n_particles as usize;

    let regular_capacity_per_container = target_n_particles / proc_info.n_units; // equivalent of batch size
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
    let n_container: usize = match proc_info.exec_policy {
        ExecPolicy::Sequential | ExecPolicy::Rayon => 1,
        ExecPolicy::Distributed => todo!(),
        ExecPolicy::Hybrid => todo!(),
    };
    println!("  [ParticleContainer Initialization]: Done");
    vec![container; n_container]
}

pub fn init_mcunits<T: CustomFloat>(mcdata: &MonteCarloData<T>) -> Vec<MonteCarloUnit<T>> {
    let mut units: Vec<MonteCarloUnit<T>> = (0..mcdata.params.simulation_params.n_units)
        .map(|_| MonteCarloUnit::default())
        .collect();

    // inits
    println!("  [MonteCarloUnit Initialization]: Start");
    init_mesh(&mut units, mcdata);
    init_tallies(&mut units, &mcdata.params);
    init_xs_cache(&mut units, mcdata.params.simulation_params.n_groups);
    println!("  [MonteCarloUnit Initialization]: Done");

    // checks
    println!("  [Consistency Check]: Start");
    // TODO: implement the check correctly according to new init
    consistency_check(&units);
    println!("  [Consistency Check]: Done");

    units
}

pub fn init_results<T: CustomFloat>(params: &Parameters<T>) -> MonteCarloResults<T> {
    MonteCarloResults::new(
        params.simulation_params.energy_spectrum.to_owned(),
        params.simulation_params.n_groups,
        params.simulation_params.coral_benchmark,
    )
}

//==================
// Private functions
//==================

fn init_nuclear_data<T: CustomFloat>(mcdata: &mut MonteCarloData<T>) {
    let params = &mcdata.params;
    let energy_low: T = params.simulation_params.e_min;
    let energy_high: T = params.simulation_params.e_max;
    mcdata.nuclear_data =
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

    mcdata.nuclear_data.isotopes.reserve(n_isotopes);
    mcdata.material_database.mat.reserve(n_materials);

    for mp in params.material_params.values() {
        let mut material: Material<T> = Material {
            name: mp.name.to_owned(),
            mass: mp.mass,
            iso: Vec::with_capacity(mp.n_isotopes),
        };

        (0..mp.n_isotopes).for_each(|_| {
            let isotope_gid = mcdata.nuclear_data.add_isotope(
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
        mcdata.material_database.add_material(material);
    }
}

/// Check the consistency of the domain list passed as argument.
///
/// This function goes through the given domain list and check for inconsistencies
/// by checking adjacencies coherence.
pub fn consistency_check<T: CustomFloat>(units: &[MonteCarloUnit<T>]) {
    units
        .iter()
        .map(|unit| &unit.domain)
        .enumerate()
        .for_each(|(domain_idx, dd)| {
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
                            let backside = &units[domain_idx_adj].domain.mesh.cell_connectivity
                                [cell_idx_adj]
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

fn init_mesh<T: CustomFloat>(mcunits: &mut [MonteCarloUnit<T>], mcdata: &MonteCarloData<T>) {
    // readability variables
    let params = &mcdata.params;
    let mat_db = &mcdata.material_database;

    let nx: usize = params.simulation_params.nx;
    let ny: usize = params.simulation_params.ny;
    let nz: usize = params.simulation_params.nz;

    let lx: T = params.simulation_params.lx;
    let ly: T = params.simulation_params.ly;
    let lz: T = params.simulation_params.lz;

    let n_units: usize = params.simulation_params.n_units as usize;

    let n_ranks: usize = 1;
    let n_domains_per_rank = 4; // why 4 in original code?
    let my_rank = 0;

    let ddc = DecompositionObject::new(my_rank, n_ranks, n_domains_per_rank);
    let global_grid: GlobalFccGrid<T> = GlobalFccGrid::new(nx, ny, nz, lx, ly, lz);

    let mut s = params.simulation_params.seed + 1; // use a seed dependant on sim seed
    let domain_centers = initialize_centers_rand(n_units, &global_grid, &mut s);

    let partition: Vec<MeshPartition> = (0..n_units)
        .map(|ii| {
            // my rank should be constant
            MeshPartition::new(ii, my_rank)
        })
        .collect();

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

    (0..n_units).for_each(|gid| {
        mcunits[gid].domain =
            MCDomain::new(&comm.partition[gid], &global_grid, &ddc, params, mat_db);
    });
}

fn init_tallies<T: CustomFloat>(mcunits: &mut [MonteCarloUnit<T>], params: &Parameters<T>) {
    mcunits.iter_mut().for_each(|mcunit| {
        mcunit.tallies.initialize_tallies(
            mcunit.domain.cell_state.len(),
            params.simulation_params.n_groups,
        )
    })
}

fn init_xs_cache<T: CustomFloat>(mcunits: &mut [MonteCarloUnit<T>], n_energy_groups: usize) {
    mcunits.iter_mut().for_each(|mcunit| {
        mcunit.xs_cache.num_groups = n_energy_groups;
        mcunit.xs_cache.cache = mcunit
            .domain
            .cell_state
            .iter()
            .flat_map(|_| (0..n_energy_groups).map(|_| Atomic::new(zero())))
            .collect();
    })
}

#[derive(Debug, Clone, Default)]
struct XSData<T: Float> {
    abs: T,
    fis: T,
    sca: T,
}

/// Prints cross-section data of the problem.
///
/// The data is simply presented in a table sorted by energy levels. Here is a snippet
/// of the first ten lines of a possible output:
///
/// ```shell
/// group |           energy |  sourceMaterial_absorb |  sourceMaterial_fission |  sourceMaterial_scatter
///     0 |   0.000000001054 |        0.0108589141086 |         0.0096890310969 |         0.0794520547945
///     1 |   0.000000001168 |        0.0108589141086 |         0.0096890310969 |         0.0794520547945
///     2 |   0.000000001294 |        0.0108589141086 |         0.0096890310969 |         0.0794520547945
///     3 |   0.000000001434 |        0.0108589141086 |         0.0096890310969 |         0.0794520547945
///     4 |   0.000000001589 |        0.0108589141086 |         0.0096890310969 |         0.0794520547945
///     5 |   0.000000001761 |        0.0108589141086 |         0.0096890310969 |         0.0794520547945
///     6 |   0.000000001952 |        0.0108589141086 |         0.0096890310969 |         0.0794520547945
///     7 |   0.000000002163 |        0.0108589141086 |         0.0096890310969 |         0.0794520547945
///     8 |   0.000000002397 |        0.0108589141086 |         0.0096890310969 |         0.0794520547945
///     9 |   0.000000002656 |        0.0108589141086 |         0.0096890310969 |         0.0794520547945
/// ```
///
/// The energy scale is logarithmic, hence the way it is printed.
pub fn check_cross_sections<T: CustomFloat>(mcdata: &MonteCarloData<T>) {
    let nucdb = &mcdata.nuclear_data;
    let matdb = &mcdata.material_database;

    let n_groups = mcdata.params.simulation_params.n_groups;

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
    let file_name = mcdata
        .params
        .simulation_params
        .cross_sections_out
        .to_owned()
        + ".dat";
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
            if xc_vec[ii].abs < T::tiny_float() {
                xc_vec[ii].abs = zero();
            }
            if xc_vec[ii].fis < T::tiny_float() {
                xc_vec[ii].fis = zero();
            }
            if xc_vec[ii].sca < T::tiny_float() {
                xc_vec[ii].sca = zero();
            }
            write!(
                file,
                "{:>20.13} |  {:>22.13} |  {:>22.13}",
                xc_vec[ii].abs, xc_vec[ii].fis, xc_vec[ii].sca
            )
            .unwrap();
        });
        writeln!(file).unwrap()
    });
}
