use num::{FromPrimitive};

use crate::constants::CustomFloat;

/// Returns the pseudo-random number produced by a call to a random
/// number generator. The returned number is a decimal in segment [0;1]
pub fn rng_sample<T: CustomFloat>(seed: &mut u64) -> T {
    // Reset the state from previous value
    *seed = 2862933555777941757u64
        .overflowing_mul(*seed)
        .0
        .overflowing_add(3037000493u64)
        .0;

    // Bijection between integers [0; 2^64] and decimal [0; 1]
    //let f: f64 = 5.4210108624275222e-20*(*seed as f64);
    let f: f64 = 5.421010862427522e-20 * (*seed as f64); // took out one decimal
    FromPrimitive::from_f64(f).unwrap()
}

/// Generates a new random number seed from the parent seed passed as
/// argument.
pub fn spawn_rn_seed<T: CustomFloat>(parent_seed: &mut u64) -> u64 {
    let spawned_seed = hash_state(*parent_seed);
    rng_sample::<T>(parent_seed);
    spawned_seed
}

/// Function used to hash a 64 bit int to get an initial state.
fn hash_state(init: u64) -> u64 {
    let mut words = breakup_u64(init);

    pseudo_des(&mut words.0, &mut words.1);

    rebuild_u64(words.0, words.1)
}

fn breakup_u64(n: u64) -> (u32, u32) {
    let tmp: [u8; 8] = n.to_be_bytes();
    let tmp1 = [tmp[0], tmp[1], tmp[2], tmp[3]];
    let tmp2 = [tmp[4], tmp[5], tmp[6], tmp[7]];
    (u32::from_be_bytes(tmp1), u32::from_be_bytes(tmp2))
}

pub fn pseudo_des(lword: &mut u32, irword: &mut u32) {
    let n_iter: usize = 2;
    let c1: [u32; 4] = [0xbaa96887, 0x1e17d32c, 0x03bcdc3c, 0x0f33d1b2];
    let c2: [u32; 4] = [0x4b0f3b58, 0xe874f0c3, 0x6955c5a6, 0x55a7ca46];

    // need to test if the results are the same in the original code
    for idx in 0..n_iter {
        let iswap: u32 = *irword;
        let mut ia = iswap ^ c1[idx];

        let itmpl: u32 = ia & 0xffff;
        let itmph: u32 = ia >> 16;
        let ib: u32 = (itmpl.overflowing_mul(itmpl).0)
            .overflowing_add(!itmph.overflowing_mul(itmph).0)
            .0;

        ia = (ib >> 16) | ((ib & 0xffff) << 16);

        *irword = *lword
            ^ ((ia ^ c2[idx])
                .overflowing_add(itmpl.overflowing_mul(itmph).0)
                .0);
        *lword = iswap;
    }
}

fn rebuild_u64(front: u32, back: u32) -> u64 {
    let frt: [u8; 4] = front.to_be_bytes();
    let bck: [u8; 4] = back.to_be_bytes();
    u64::from_be_bytes([
        frt[0], frt[1], frt[2], frt[3], bck[0], bck[1], bck[2], bck[3],
    ])
}
