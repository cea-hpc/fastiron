// Tests used to compare the results of certain computation heavy
// functions with their result in the original code.
// A nice proper way to do it would be to make external calls
// to C++ functions and compare the results in the tests.
// For now, printing will be just fine.

use fastiron::{
    collision_event::update_trajectory,
    direction_cosine::DirectionCosine,
    mc::{
        mc_particle::MCParticle,
        mc_rng_state::{pseudo_des, rng_sample, spawn_rn_seed},
        mc_vector::MCVector,
    }, physical_constants::SMALL_FLOAT,
};
use num::Float;

#[test]
pub fn rng_spawned_number() {
    let mut seed: u64 = 90374384094798327;
    let res = spawn_rn_seed::<f64>(&mut seed);
    println!();
    println!("###########################");
    println!("#   spawned number test   #");
    println!("###########################");
    println!("spawned number: {res}");
}

#[test]
pub fn pseudo_hash() {
    let mut a: u32 = 123214124;
    let mut b: u32 = 968374242;
    pseudo_des(&mut a, &mut b);
    println!();
    println!("###########################");
    println!("#     pseudo hash test    #");
    println!("###########################");
    println!("a: {a}");
    println!("b: {b}");
}

#[test]
pub fn sample_isotropic() {
    let mut dd: DirectionCosine<f64> = DirectionCosine {
        alpha: 0.2140,
        beta: 0.8621,
        gamma: 0.7821,
    };
    let mut seed: u64 = 90374384094798327;
    dd.sample_isotropic(&mut seed);
    println!();
    println!("###########################");
    println!("#  sample isotropic test  #");
    println!("###########################");
    println!("dd: {dd:#?}");
}

#[test]
pub fn rotate_vector() {
    let mut dd: DirectionCosine<f64> = DirectionCosine {
        alpha: 0.2140,
        beta: 0.8621,
        gamma: 0.7821,
    };
    dd.rotate_3d_vector(1.0.sin(), 1.0.cos(), 2.0.sin(), 2.0.cos());
    println!();
    println!("###########################");
    println!("#  rotate 3d vector test  #");
    println!("###########################");
    println!("dd: {dd:#?}");
}

#[test]
pub fn trajectory() {
    let mut pp: MCParticle<f64> = MCParticle::default();
    // sets parameters
    let vv: MCVector<f64> = MCVector {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let d_cos: DirectionCosine<f64> = DirectionCosine {
        alpha: 1.0 / 3.0.sqrt(),
        beta: 1.0 / 3.0.sqrt(),
        gamma: 1.0 / 3.0.sqrt(),
    };
    let e: f64 = 1.0;
    pp.velocity = vv;
    pp.direction_cosine = d_cos;
    pp.kinetic_energy = e;
    let mut seed: u64 = 90374384094798327;
    let energy = rng_sample(&mut seed);
    let angle = rng_sample(&mut seed);

    // update & print result
    update_trajectory(energy, angle, &mut pp);

    println!();
    println!("###########################");
    println!("#  update trajectory test #");
    println!("###########################");
    println!("Energy: {energy}");
    println!("Angle: {angle}");
    println!("Particle: {pp:#?}");
}

#[test]
pub fn move_particle() {
    // copy pasting the core of the function to avoid the init of whole structures
    let move_factor: f64 = 0.5 * SMALL_FLOAT;
    let mut coord: MCVector<f64> = MCVector {x: 1.923, y: -2.45, z: 5.013 };
    let move_to: MCVector<f64> = MCVector { x: 4.0, y: 0.241, z: 7.9020 };
    
    coord += (move_to - coord) * move_factor;

    println!();
    println!("###########################");
    println!("#    move particle test   #");
    println!("###########################");
    println!("moved coord: {coord:#?}");
}

#[test]
pub fn compute_volume() {
    let v0: MCVector<f64> = MCVector {x: 1.923, y: -2.45, z: 5.013 };
    let v1: MCVector<f64> = MCVector {x: 3.041, y: 1.368, z: 9.143 };
    let v2: MCVector<f64> = MCVector {x: 6.235, y: 0.325, z: 2.502 };
    let v3: MCVector<f64> = MCVector {x: 1.634, y: -1.34, z: 3.873 };

    let tmp0 = v0 - v3;
    let tmp1 = v1 - v3;
    let tmp2 = v2 - v3;

    let volume = tmp0.dot(&tmp1.cross(&tmp2));

    println!();
    println!("###########################");
    println!("#   compute volume test   #");
    println!("###########################");
    println!("volume: {volume}");
}

#[test]
pub fn macros() {

}