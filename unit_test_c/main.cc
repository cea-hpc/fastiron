#include "DirectionCosine.hh"
#include "MC_Particle.hh"
#include "MC_RNG_State.hh"
#include "MC_Vector.hh"
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
    uint64_t init_seed_t = 90374384094798327;
    uint64_t* seed_t = &init_seed_t;
    double energy = rngSample(seed_t);
    double angle = rngSample(seed_t);
    updateTrajectory(energy, angle, pp);
    printf("energy: %17.16f\n", energy);
    printf("angle: %17.16f\n", angle);
    pp.PrintParticle();

}