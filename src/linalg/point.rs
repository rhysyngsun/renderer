use std::mem;
use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut};
use linalg::{Vector3, ApproxEq};

/// A point of 3 values
#[derive(Debug, Copy, Clone)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

scalar_new_impl!(Point3, x, y, z);
scalar_add_impl!(Point3, x, y, z);
pnt_add_vec_impl!(Point3, Vector3, x, y, z);
pnt_sub_pnt_impl!(Point3, Vector3, x, y, z);
pnt_sub_vec_impl!(Point3, Vector3, x, y, z);
scalar_mul_impl!(Point3, x, y, z);
pnt_mul_vec_impl!(Point3, Vector3, x, y, z);
scalar_div_impl!(Point3, x, y, z);
scalar_neg_impl!(Point3, x, y, z);
scalar_to_array_impl!(Point3, 3);
scalar_index_impl!(Point3);
pnt_distance_impl!(Point3);


#[cfg(test)]
mod test {
    use super::*;
    use linalg::Vector3;

    #[test]
    fn op_add_point() {
        let v = Point3 {
            x: 10.0,
            y: 8.0,
            z: 6.0,
        };
        let v2 = v + v;

        assert_eq!(v2.x, 20.0);
        assert_eq!(v2.y, 16.0);
        assert_eq!(v2.z, 12.0);
    }

    #[test]
    fn op_sub_point() {
        let v = Point3 {
            x: 10.0,
            y: 8.0,
            z: 6.0,
        };
        let v2 = v - v;

        assert_eq!(v2.x, 0.0);
        assert_eq!(v2.y, 0.0);
        assert_eq!(v2.z, 0.0);
    }

    #[test]
    fn op_sub_vector3() {
        let v = Point3 {
            x: 10.0,
            y: 8.0,
            z: 6.0,
        };
        let v2 = Vector3 {
            x: 4.0,
            y: 2.0,
            z: 2.0,
        };
        let v3 = v - v2;

        assert_eq!(v3.x, 6.0);
        assert_eq!(v3.y, 6.0);
        assert_eq!(v3.z, 4.0);
    }

    #[test]
    fn op_mul_scalar() {
        let v = Point3 {
            x: 10.0,
            y: 8.0,
            z: 6.0,
        };
        let v2 = v * 2.0;

        assert_eq!(v2.x, 20.0);
        assert_eq!(v2.y, 16.0);
        assert_eq!(v2.z, 12.0);
    }

    #[test]
    fn op_neg() {
        let v = Point3 {
            x: 10.0,
            y: -8.0,
            z: 6.0,
        };
        let v2 = -v;

        assert_eq!(v2.x, -10.0);
        assert_eq!(v2.y, 8.0);
        assert_eq!(v2.z, -6.0);
    }

    #[test]
    fn op_div_nonzero_scalar() {
        let v = Point3 {
            x: 10.0,
            y: 8.0,
            z: 6.0,
        };
        let v2 = v / 2.0;

        assert_eq!(v2.x, 5.0);
        assert_eq!(v2.y, 4.0);
        assert_eq!(v2.z, 3.0);
    }

    #[test]
    #[should_panic]
    fn op_div_zero_scalar() {
        let v = Point3 {
            x: 10.0,
            y: 8.0,
            z: 6.0,
        };
        v / 0.0;
    }

    #[test]
    fn op_index() {
        let v = Point3 {
            x: 10.0,
            y: 8.0,
            z: 6.0,
        };

        assert_eq!(v[0], 10.0);
        assert_eq!(v[1], 8.0);
        assert_eq!(v[2], 6.0);
    }

    #[test]
    fn op_mut_index() {
        let mut v = Point3 {
            x: 10.0,
            y: 8.0,
            z: 6.0,
        };

        v[0] = 2.0;
        v[1] = 1.0;
        v[2] = 0.0;

        assert_eq!(v[0], 2.0);
        assert_eq!(v[1], 1.0);
        assert_eq!(v[2], 0.0);
    }

}
