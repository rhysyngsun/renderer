use std::mem;

use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Index,
    IndexMut
};

use linalg::{
    Cross,
    ApproxEq
};
use linalg::ops::{
    One,
    Zero,
};

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

/// A 3-vector of f64 values
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Cross for Vector3 {
    type Output = Self;

    #[inline]
    fn cross(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: (self.y * v.z) - (self.z * v.y),
            y: (self.z * v.x) - (self.x * v.z),
            z: (self.x * v.y) - (self.y * v.x),
        }
    }
}

impl Vector3 {
    pub fn coordinate_system(&self) -> (Vector3, Vector3) {
        let v2 = if f64::abs(self.x) > f64::abs(self.y) {
            let inv_len = 1.0 / f64::sqrt(self.x * self.x + self.z * self.z);
            Vector3 {
                x: -self.z * inv_len,
                y: 0.0,
                z: self.x * inv_len,
            }
        } else {
            let inv_len = 1.0 / f64::sqrt(self.y * self.y + self.z * self.z);
            Vector3 {
                x: 0.0,
                y: self.z * inv_len,
                z: -self.y * inv_len,
            }
        };

        (v2, self.cross(&v2))
    }
}

scalar_new_impl!(Vector3, x, y, z);
scalar_add_impl!(Vector3, x, y, z);
scalar_sub_impl!(Vector3, x, y, z);
scalar_div_impl!(Vector3, x, y, z);
scalar_mul_impl!(Vector3, x, y, z);
scalar_neg_impl!(Vector3, x, y, z);
dot_impl!(Vector3, x, y, z);
len_impl!(Vector3, x, y, z);
normalize_impl!(Vector3, x, y, z);
zero_impl!(Vector3, x, y, z);
one_impl!(Vector3, x, y, z);
arbitrary_impl!(Vector3, x, y, z);
approx_eq_impl!(Vector3, x, y, z);
scalar_to_array_impl!(Vector3, 3);
scalar_index_impl!(Vector3);

#[cfg(test)]
mod test {
    #![macro_use]

    use super::Vector3;
    use quickcheck::TestResult;
    use linalg::ApproxEq;

    #[quickcheck]
    fn op_add_vec3(v1: Vector3, v2: Vector3) -> bool {
        let v3 = v1 + v2;
        let v4 = Vector3::new(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z);

        ::approx_eq(&v3, &v4, &f64::approx_eps())
    }

    #[quickcheck]
    fn op_sub_vec3(v1: Vector3, v2: Vector3) -> bool {
        let v3 = v1 - v2;
        let v4 = Vector3::new(v1.x - v2.x, v1.y - v2.y, v1.z - v2.z);

        ::approx_eq(&v3, &v4, &f64::approx_eps())
    }

    #[quickcheck]
    fn op_div_scalar(v1: Vector3, s: f64) -> TestResult {
        if ::approx_eq(&s, &0.0f64, &f64::approx_eps()) {
            return TestResult::discard();
        }
        let v3 = v1 / s;
        let v4 = Vector3::new(v1.x / s, v1.y / s, v1.z / s);

        TestResult::from_bool(::approx_eq(&v3, &v4, &f64::approx_eps()))
    }

    #[quickcheck]
    fn op_mul_scalar(v1: Vector3, s: f64) -> bool {
        let v3 = v1 * s;
        let v4 = Vector3::new(v1.x * s, v1.y * s, v1.z * s);

        ::approx_eq(&v3, &v4, &f64::approx_eps())
    }

    #[quickcheck]
    fn to_array(v: Vector3) -> bool {
        let ar = v.to_array();
        let ax = [v.x, v.y, v.z];
        PartialEq::eq(ar, &ax)
    }

    #[quickcheck]
    fn op_index(v: Vector3) -> bool {
        v[0] == v.x && v[1] == v.y && v[2] == v.z
    }

    #[quickcheck]
    fn op_index_mut(v: Vector3) -> bool {
        let v2 = &mut v.clone();

        v2[0] = 0.0;
        v2[1] = 0.0;
        v2[2] = 0.0;

        ::approx_eq(&v2.x, &0.0, &f64::approx_eps()) &&
        ::approx_eq(&v2.y, &0.0, &f64::approx_eps()) &&
        ::approx_eq(&v2.z, &0.0, &f64::approx_eps())
    }
}
