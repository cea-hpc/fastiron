#ifndef DIRECTION_COSINE_INCLUDE
#define DIRECTION_COSINE_INCLUDE

#include <stdint.h>
#include <math.h>

class DirectionCosine
{
public:
   double alpha;
   double beta;
   double gamma;

   DirectionCosine();

   DirectionCosine(double alpha, double beta, double gamma);

   DirectionCosine &operator=(const DirectionCosine &dc) 
   {
       alpha = dc.alpha;
       beta  = dc.beta;
       gamma = dc.gamma;
       return *this;
    }

   void Sample_Isotropic(uint64_t *seed);

   // rotate a direction cosine given the sine/cosine of theta and phi
   inline void Rotate3DVector( double sine_Theta,
                               double cosine_Theta,
                               double sine_Phi,
                               double cosine_Phi );

};


inline DirectionCosine::DirectionCosine()
   : alpha(0.0), beta(0.0), gamma(0.0)
{
}


inline DirectionCosine::DirectionCosine(double a_alpha, double a_beta, double a_gamma)
   : alpha(a_alpha),
     beta(a_beta),
     gamma(a_gamma)
{
}


inline void DirectionCosine::Rotate3DVector(double sin_Theta, double cos_Theta, double sin_Phi, double cos_Phi)
{
    // Calculate additional variables in the rotation matrix.
    double cos_theta = this->gamma;
    double sin_theta = sqrt((1.0 - (cos_theta*cos_theta)));

    double cos_phi;
    double sin_phi;
    if (sin_theta < 1e-6) // Order of sqrt(PhysicalConstants::tiny_double)
    {
        cos_phi = 1.0; // assume phi  = 0.0;
        sin_phi = 0.0;
    }
    else
    {
        cos_phi = this->alpha/sin_theta;
        sin_phi = this->beta/sin_theta;
    }

    // Calculate the rotated direction cosine
    this->alpha =  cos_theta*cos_phi*(sin_Theta*cos_Phi) - sin_phi*(sin_Theta*sin_Phi) + sin_theta*cos_phi*cos_Theta;
    this->beta  =  cos_theta*sin_phi*(sin_Theta*cos_Phi) + cos_phi*(sin_Theta*sin_Phi) + sin_theta*sin_phi*cos_Theta;
    this->gamma = -sin_theta        *(sin_Theta*cos_Phi) +                               cos_theta        *cos_Theta;
}

#endif
