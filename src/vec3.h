#ifndef _CLASS_VEC3_
#define _CLASS_VEC3_

template <typename T>
class vec3
{
protected:
    T x;
    T y;
    T z;

public:
    vec3::vec3(T _x, T _y, T _z)
    {
        x = _x;
        y = _y;
        z = _z;
    }

    vec3 operator * (T target) {
        return vec3(this->x * target, this->y * target, this->z * target);
    }

    vec3 operator * (vec3 target) {
        return vec3(this->x * target->x, this->y * target->y, this->z * target->z);
    }

    vec3 operator + (vec3 target) {
        return vec3(this->x + target->x, this->y + target->y, this->z + target->z);
    }

    vec3 operator - (vec3 target) {
        return vec3(this->x - target->x, this->y - target->y, this->z - target->z);
    }

    T dot(vec3 target) {
        return ((this->x * target->x) +
                (this->y * target->y) +
                (this->z * target->z));
    }

    vec3 cross(vec3 target) {
        return vec3(this->y * target->z - this->z * target->y,
                    this->z * target->x - this->x * target->z,
                    this->x * target->y - this->y * target->x
                    );
    }

    double magnitude() {
        return std::sqrt(this->dot(this));
    }

    T sqrMagnitude() {
        return this->dot(this);
    }

    vec3 normalized() {
        return (1 / this->sqrMagnitude) * this;
    }

    bool eqValue(vec3 target) {
        return (this->x == target->x) &&
               (this->y == target->y) && 
               (this->z == target->z);
    }

    ~vec3() {

    }
};

using vec3f = vec3<float>;

#endif