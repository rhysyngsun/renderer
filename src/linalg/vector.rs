
use std::num::{Float,ToPrimitive,NumCast};
use std::ops::{Add,Sub,Mul,Div,Neg};

use linalg::{One,Zero,Cross,BaseNum};

#[cfg(feature="arbitrary")]
use quickcheck::{Arbitrary, Gen};


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector3<N : BaseNum> {
    pub x: N,
    pub y: N,
    pub z: N
}

impl<N : BaseNum> Cross for Vector3<N> {
    type Output = Self;

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

new_impl!(Vector3, x, y, z);
add_impl!(Vector3, x, y, z);
sub_impl!(Vector3, x, y, z);
div_impl!(Vector3, x, y, z);
mul_impl!(Vector3, x, y, z);
neg_impl!(Vector3, x, y, z);
dot_impl!(Vector3, x, y, z);
len_impl!(Vector3, x, y, z);
zero_impl!(Vector3, x, y, z);
one_impl!(Vector3, x, y, z);
arbitrary_impl!(Vector3, x, y, z);

// impl<N> ops::Index<usize> for Vector3<N> {
//     type Output = f64;
//
//     fn index(&self, i: &usize) -> &f64 {
//         match *i {
//             0 => &self.x,
//             1 => &self.y,
//             2 => &self.z,
//             _ => panic!("Invalid index for Vector3"),
//         }
//     }
// }
//
// impl<N> ops::IndexMut<usize> for Vector3<N> {
//     fn index_mut<'a>(&'a mut self, i: &usize) -> &'a mut f64 {
//         match *i {
//             0 => &mut self.x,
//             1 => &mut self.y,
//             2 => &mut self.z,
//             _ => panic!("Invalid index for Point"),
//         }
//     }
// }

#[cfg(test)]
mod test {
    use super::Vector3;

    #[quickcheck]
    fn add_vec(v1: Vector3<f64>, v2:Vector3<f64>) {
        let v3 = v1 + v2;
        let v4 = Vector3::new(v1.x + v2.x, v1.y + v2.y, v3.z + v3.z);
        v3 == v4
    }
}

