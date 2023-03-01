// Tests used to compare the results of certain computation heavy 
// functions with their result in the original code. 
// A nice proper way to do it would be to make external calls
// to C++ functions and compare the results in the tests.
// For now, printing will be just fine.

use fastiron::mc::mc_rng_state::{spawn_rn_seed, pseudo_des};

#[test]
pub fn rng_spawned_number() {
    let mut seed: u64 = 90374384094798327;
    let res = spawn_rn_seed::<f64>(&mut seed);
    println!("spawned number: {res}");
    //panic!()
}

#[test]
pub fn pseudo() {
    let mut a: u32 = 123214124;
    let mut b: u32 = 968374242;
    pseudo_des(&mut a, &mut b);
    println!("a: {a}");
    println!("b: {b}");
    //panic!()
}