use std::mem;

use std::num::{
    One,
    Zero,
};

use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Index,
    IndexMut,
};

use linalg::{
    BaseNum,
    Cross,
    ApproxEq,
    Absolute,
    Sqrt,
};

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

/// A vector of 3 values
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector3<N> {
    pub x: N,
    pub y: N,
    pub z: N
}

impl<N : BaseNum> Cross for Vector3<N> {
    type Output = Self;

    #[inline]
    fn cross(&self, v:&Vector3<N>) -> Vector3<N> {
        Vector3 {
            x: (self.y * v.z) - (self.z * v.y),
            y: (self.z * v.x) - (self.x * v.z),
            z: (self.x * v.y) - (self.y * v.x)
        }
    }
}

impl<N : BaseNum> Vector3<N> {
     pub fn coordinate_system(&self) -> (Vector3<N>, Vector3<N>) {
         let v2 = if N::abs(&self.x) > N::abs(&self.y) {
             let inv_len = N::one() / N::sqrt(&(self.x * self.x + self.z * self.z));
             Vector3 {
                 x: -self.z * inv_len,
                 y: N::zero(),
                 z: self.x * inv_len
             }
         } else {
             let inv_len = N::one() / N::sqrt(&(self.y * self.y + self.z * self.z));
             Vector3 {
                 x: N::zero(),
                 y: self.z * inv_len,
                 z: -self.y * inv_len
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
    fn op_add_vec3(v1: Vector3<f64>, v2: Vector3<f64>) -> bool {
        let v3 = v1 + v2;
        let v4 = Vector3::new(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z);
        //println!("Expected {:?} + {:?} == {:?}, got {:?}", v1, v2, v4, v3);
        ::approx_eq(&v3, &v4, &f64::approx_eps())
    }

    #[quickcheck]
    fn op_sub_vec3(v1: Vector3<f64>, v2: Vector3<f64>) -> bool {
        let v3 = v1 - v2;
        let v4 = Vector3::new(v1.x - v2.x, v1.y - v2.y, v1.z - v2.z);
        //println!("Expected {:?} - {:?} == {:?}, got {:?}", v1, v2, v4, v3);
        ::approx_eq(&v3, &v4, &f64::approx_eps())
    }

    #[quickcheck]
    fn op_div_scalar(v1: Vector3<f64>, s: f64) -> TestResult {
        if ::approx_eq(&s, &0.0f64, &f64::approx_eps()) {
            return TestResult::discard()
        }
        let v3 = v1 / s;
        let v4 = Vector3::new(v1.x / s, v1.y / s, v1.z / s);
        //println!("Expected {:?} / {:?} == {:?}, got {:?}", v1, s, v4, v3);
        TestResult::from_bool(::approx_eq(&v3, &v4, &f64::approx_eps()))
    }

    #[quickcheck]
    fn op_mul_scalar(v1: Vector3<f64>, s: f64) -> bool {
        let v3 = v1 * s;
        let v4 = Vector3::new(v1.x * s, v1.y * s, v1.z * s);
        //println!("Expected {:?} * {:?} == {:?}, got {:?}", v1, s, v4, v3);
        ::approx_eq(&v3, &v4, &f64::approx_eps())
    }

    #[quickcheck]
    fn to_array(v: Vector3<f64>) -> bool {
        let ar = v.to_array();
        let ax = [v.x, v.y, v.z];
        PartialEq::eq(ar, &ax)
    }

    #[quickcheck]
    fn op_index(v: Vector3<f64>) -> bool {
        v[0] == v.x && v[1] == v.y && v[2] == v.z
    }

    #[quickcheck]
    fn op_index_mut(v: Vector3<f64>) -> bool {
        let v2 = &mut v.clone();

        v2[0] = 0.0;
        v2[1] = 0.0;
        v2[2] = 0.0;

        ::approx_eq(&v2.x, &0.0, &f64::approx_eps()) &&
            ::approx_eq(&v2.y, &0.0, &f64::approx_eps()) &&
            ::approx_eq(&v2.z, &0.0, &f64::approx_eps())
    }
}
