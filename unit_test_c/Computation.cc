#include "Computation.hh"

double MCT_Cell_Volume_3D_G_vector_tetDet(const MC_Vector &v0_,
                                          const MC_Vector &v1_,
                                          const MC_Vector &v2_,
                                          const MC_Vector &v3) {
    MC_Vector v0(v0_), v1(v1_), v2(v2_);

    v0.x -= v3.x;
    v0.y -= v3.y;
    v0.z -= v3.z;
    v1.x -= v3.x;
    v1.y -= v3.y;
    v1.z -= v3.z;
    v2.x -= v3.x;
    v2.y -= v3.y;
    v2.z -= v3.z;

    return v0.z * (v1.x * v2.y - v1.y * v2.x) +
         v0.y * (v1.z * v2.x - v1.x * v2.z) +
         v0.x * (v1.y * v2.z - v1.z * v2.y);
}
