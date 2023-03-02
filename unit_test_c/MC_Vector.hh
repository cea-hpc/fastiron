#ifndef MC_VECTOR_INCLUDE
#define MC_VECTOR_INCLUDE

#include <math.h>

class MC_Vector
{
 public:
   double x;
   double y;
   double z;

   MC_Vector() : x(0), y(0), z(0) {}

   MC_Vector(double a, double b, double c) : x(a), y(b), z(c) {}

   MC_Vector& operator=( const MC_Vector&tmp )
   {
      if ( this == &tmp ) { return *this; }

      x = tmp.x;
      y = tmp.y;
      z = tmp.z;

      return *this;
   }

   bool operator==( const MC_Vector& tmp )
   {
      return tmp.x == x && tmp.y == y && tmp.z == z;
   }

   MC_Vector& operator+=( const MC_Vector &tmp )
   {
      x += tmp.x;
      y += tmp.y;
      z += tmp.z;
      return *this;
   }

   MC_Vector& operator-=( const MC_Vector &tmp )
   {
      x -= tmp.x;
      y -= tmp.y;
      z -= tmp.z;
      return *this;
   }

   MC_Vector& operator*=(const double scalar)
   {
      x *= scalar;
      y *= scalar;
      z *= scalar;
      return *this;
   }

   MC_Vector& operator/=(const double scalar)
   {
      x /= scalar;
      y /= scalar;
      z /= scalar;
      return *this;
   }

   const MC_Vector operator+( const MC_Vector &tmp ) const
   {
      return MC_Vector(x + tmp.x, y + tmp.y, z + tmp.z);
   }

   const MC_Vector operator-( const MC_Vector &tmp ) const
   {
      return MC_Vector(x - tmp.x, y - tmp.y, z - tmp.z);
   }

   const MC_Vector operator*(const double scalar) const
   {
      return MC_Vector(scalar*x, scalar*y, scalar*z);
   }

   inline double Length() const { return sqrt(x*x + y*y + z*z); }

   // Distance from this vector to another point.
   inline double Distance(const MC_Vector& vv) const
   { return sqrt((x - vv.x)*(x - vv.x) + (y - vv.y)*(y - vv.y)+ (z - vv.z)*(z - vv.z)); }

   inline double Dot(const MC_Vector &tmp) const
   {
      return this->x*tmp.x + this->y*tmp.y + this->z*tmp.z;
   }

   inline MC_Vector Cross(const MC_Vector &v) const
   {
      return MC_Vector(y * v.z - z * v.y,
                       z * v.x - x * v.z,
                       x * v.y - y * v.x);
   }

};


#endif