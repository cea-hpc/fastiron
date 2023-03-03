#ifndef COMPUTATION
#define COMPUTATION

#include "MC_Vector.hh"

double MCT_Cell_Volume_3D_G_vector_tetDet(const MC_Vector &v0_,
                                          const MC_Vector &v1_,
                                          const MC_Vector &v2_,
                                          const MC_Vector &v3);

#define AB_CROSS_AC(ax,ay,bx,by,cx,cy) ( (bx-ax)*(cy-ay) - (by-ay)*(cx-ax) )

const double tolerance = 1.0e-9;

#define IF_POINT_BELOW_CONTINUE(intersection_pt, facet_coords0, facet_coords1, facet_coords2, axis)                                    \
             facet_coords0.axis > intersection_pt.axis + tolerance && \
             facet_coords1.axis > intersection_pt.axis + tolerance && \
             facet_coords2.axis > intersection_pt.axis + tolerance

#define IF_POINT_ABOVE_CONTINUE(intersection_pt, facet_coords0, facet_coords1, facet_coords2, axis)                                    \
             facet_coords0.axis < intersection_pt.axis - tolerance && \
             facet_coords1.axis < intersection_pt.axis - tolerance && \
             facet_coords2.axis < intersection_pt.axis - tolerance

#define BELONGS(intersection_pt, facet_coords0, facet_coords1, facet_coords2, axis) IF_POINT_ABOVE_CONTINUE(intersection_pt, facet_coords0, facet_coords1, facet_coords2, axis) || IF_POINT_BELOW_CONTINUE(intersection_pt, facet_coords0, facet_coords1, facet_coords2, axis)

#endif