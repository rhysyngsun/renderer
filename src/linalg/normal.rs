
use std::mem;
use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut};

use linalg::ops::{One, Zero, Cross, ApproxEq};
use linalg::vector::Vector3;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
struct Normal3 {
    x: f64,
    y: f64,
    z: f64,
}

scalar_new_impl!(Normal3, x, y, z);
scalar_add_impl!(Normal3, x, y, z);
scalar_sub_impl!(Normal3, x, y, z);
scalar_div_impl!(Normal3, x, y, z);
scalar_mul_impl!(Normal3, x, y, z);
scalar_neg_impl!(Normal3, x, y, z);
dot_impl!(Normal3, x, y, z);
len_impl!(Normal3, x, y, z);
normalize_impl!(Normal3, x, y, z);
zero_impl!(Normal3, x, y, z);
one_impl!(Normal3, x, y, z);
arbitrary_impl!(Normal3, x, y, z);
approx_eq_impl!(Normal3, x, y, z);
scalar_to_array_impl!(Normal3, 3);
scalar_index_impl!(Normal3);

impl From<Vector3> for Normal3 {
    fn from(v: Vector3) -> Self {
        Normal3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}
