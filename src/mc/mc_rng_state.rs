use num::Float;

/// Generates a new random number seed from the parent seed passed as
/// argument.
pub fn spawn_rn_seed(parent_seed: &u64) -> u64 {
    todo!()
}

/// Returns the pseudo-random number produced by a call to a random
/// number generator.
pub fn rng_sample<T: Float>(seed: &mut u64) -> T {
    todo!()
}
