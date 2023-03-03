#include "Computation.hh"
#include "DirectionCosine.hh"
#include "MC_Particle.hh"
#include "MC_RNG_State.hh"
#include "MC_Vector.hh"
#include "PhysicalConstants.hh"
#include "UpdateTrajectory.hh"
#include <math.h>
#include <stdint.h>
#include <stdio.h>

int main(int argc, char** argv) {
    printf("###########################\n");
    printf("#   spawned number test   #\n");
    printf("###########################\n");
    uint64_t init_seed_test = 90374384094798327;
    uint64_t* seed_test = &init_seed_test;
    uint64_t res = rngSpawn_Random_Number_Seed(seed_test);
    printf("spawned number: %lu\n", res);
    
    printf("\n");
    printf("###########################\n");
    printf("#     pseudo hash test    #\n");
    printf("###########################\n");
    uint32_t a = 123214124;
    uint32_t b = 968374242;
    pseudo_des(a, b);
    printf("a: %u\n", a);
    printf("b: %u\n", b);

    printf("\n");
    printf("###########################\n");
    printf("#  sample isotropic test  #\n");
    printf("###########################\n");
    DirectionCosine dd = DirectionCosine(0.2140, 0.8621, 0.7821);
    uint64_t init_seed = 90374384094798327;
    uint64_t* seed = &init_seed;
    dd.Sample_Isotropic(seed);
    printf("alpha: %17.16f\n",  dd.alpha);
    printf("beta: %17.16f\n" ,  dd.beta );
    printf("gamma: %17.16f\n",  dd.gamma);

    printf("\n");
    printf("###########################\n");
    printf("#  rotate 3d vector test  #\n");
    printf("###########################\n");
    DirectionCosine dd_r = DirectionCosine(0.2140, 0.8621, 0.7821);
    dd_r.Rotate3DVector(sin(1.0), cos(1.0), sin(2.0), cos(2.0));
    printf("alpha: %17.16f\n",  dd_r.alpha);
    printf("beta: %17.16f\n" ,  dd_r.beta );
    printf("gamma: %17.16f\n",  dd_r.gamma);

    printf("\n");
    printf("###########################\n");
    printf("#  update trajectory test #\n");
    printf("###########################\n");
    MC_Particle pp = MC_Particle();
    MC_Vector vv = MC_Vector(1.0, 1.0, 1.0);
    DirectionCosine d_cos = DirectionCosine(1.0/sqrt(3.0), 1.0/sqrt(3.0), 1.0/sqrt(3.0));
    pp.direction_cosine = d_cos;
    pp.velocity = vv;
    uint64_t init_seed_t = 90374384094798327;
    uint64_t* seed_t = &init_seed_t;
    double energy = rngSample(seed_t);
    double angle = rngSample(seed_t);
    updateTrajectory(energy, angle, pp);
    printf("energy: %17.16f\n", energy);
    printf("angle: %17.16f\n", angle);
    pp.PrintParticle();

    printf("\n");
    printf("###########################\n");
    printf("#    move particle test   #\n");
    printf("###########################\n");
    MC_Vector coordinate = MC_Vector(1.923, -2.45, 5.013);
    MC_Vector move_to = MC_Vector(4.0, 0.241, 7.9020);
    double move_factor = 0.5 * PhysicalConstants::_smallDouble;
    coordinate.x += move_factor * ( move_to.x - coordinate.x );
    coordinate.y += move_factor * ( move_to.y - coordinate.y );
    coordinate.z += move_factor * ( move_to.z - coordinate.z );
    printf("coordinate.x: %17.16f\n", coordinate.x);
    printf("coordinate.y: %17.16f\n", coordinate.y);
    printf("coordinate.z: %17.16f\n", coordinate.z);

    printf("\n");
    printf("###########################\n");
    printf("#   compute volume test   #\n");
    printf("###########################\n");
    MC_Vector v0 = MC_Vector(1.923, -2.45, 5.013);
    MC_Vector v1 = MC_Vector(3.041, 1.368, 9.143);
    MC_Vector v2 = MC_Vector(6.235, 0.325, 2.502);
    MC_Vector v3 = MC_Vector(1.634, -1.34, 3.873);

    double volume = MCT_Cell_Volume_3D_G_vector_tetDet(v0, v1, v2, v3);
    printf("volume: %17.16f\n", volume);
}