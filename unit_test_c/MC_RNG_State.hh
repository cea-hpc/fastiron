#ifndef MC_RNG_STATE_INCLUDE
#define MC_RNG_STATE_INCLUDE
#include <stdint.h>

//----------------------------------------------------------------------------------------------------------------------
//  A random number generator that implements a 64 bit linear congruential generator (lcg).
//
//  This implementation is based on the rng class from Nick Gentile.
//----------------------------------------------------------------------------------------------------------------------

// Generate a new random number seed
uint64_t rngSpawn_Random_Number_Seed(uint64_t *parent_seed);

void pseudo_des(uint32_t& lword, uint32_t& irword);

//----------------------------------------------------------------------------------------------------------------------
//  Sample returns the pseudo-random number produced by a call to a random
//  number generator.
//----------------------------------------------------------------------------------------------------------------------
inline double rngSample(uint64_t *seed)
{
   // Reset the state from the previous value.
   *seed = 2862933555777941757ULL*(*seed) + 3037000493ULL;

   // Map the int state in (0,2**64) to double (0,1)
   // by multiplying by
   // 1/(2**64 - 1) = 1/18446744073709551615.
   return 5.4210108624275222e-20*(*seed);
}

#endif