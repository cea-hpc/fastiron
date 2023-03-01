#include "UpdateTrajectory.hh"
#include "MC_Particle.hh"
#include "DirectionCosine.hh"
#include "PhysicalConstants.hh"
#include "MC_RNG_State.hh"
#include <stdio.h>

#define MAX_PRODUCTION_SIZE 4

//----------------------------------------------------------------------------------------------------------------------
//  Routine MC_Collision_Event determines the isotope, reaction and secondary (projectile)
//  particle characteristics for a collision event.
//
//  Return true if the particle will continue.
//----------------------------------------------------------------------------------------------------------------------

void updateTrajectory( double energy, double angle, MC_Particle& particle )
{
    particle.kinetic_energy = energy;

    double cosTheta = angle;
    double sinTheta = sqrt((1.0 - (cosTheta*cosTheta)));
    double randomNumber = rngSample(&particle.random_number_seed);
    double phi = 2 * 3.14159265 * randomNumber;
    double sinPhi = sin(phi);
    double cosPhi = cos(phi);

    particle.direction_cosine.Rotate3DVector(sinTheta, cosTheta, sinPhi, cosPhi);
    double speed = (PhysicalConstants::_speedOfLight *
            sqrt((1.0 - ((PhysicalConstants::_neutronRestMassEnergy *
            PhysicalConstants::_neutronRestMassEnergy) /
            ((energy + PhysicalConstants::_neutronRestMassEnergy) *
            (energy + PhysicalConstants::_neutronRestMassEnergy))))));

    particle.velocity.x = speed * particle.direction_cosine.alpha;
    particle.velocity.y = speed * particle.direction_cosine.beta;
    particle.velocity.z = speed * particle.direction_cosine.gamma;
    randomNumber = rngSample(&particle.random_number_seed);
    particle.num_mean_free_paths = -1.0*log(randomNumber);
}
