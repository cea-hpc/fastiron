// Tests used to compare the results of certain computation heavy
// functions with their result in the original code.
// A nice proper way to do it would be to make external calls
// to C++ functions and compare the results in the tests.
// For now, printing will be just fine.

use fastiron::{
    direction_cosine::DirectionCosine,
    mc::mc_rng_state::{pseudo_des, spawn_rn_seed},
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
