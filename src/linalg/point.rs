
use std::ops;
use linalg::Vector3;


#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64
}

impl ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl ops::Sub<Vector3<f64>> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector3<f64>) -> Point {
        Point{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl ops::Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Point {
        assert!(rhs != 0.0);
        let inv = 1.0 / rhs;
        Point{x: self.x * inv, y: self.y * inv, z: self.z * inv}
    }
}

impl ops::Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Point {
        Point{x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl ops::Mul<Vector3<f64>> for Point {
    type Output = Point;

    fn mul(self, rhs: Vector3<f64>) -> Point {
        Point{x: self.x * rhs.x, y: self.y * rhs.x, z: self.z * rhs.z}
    }
}

impl ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point{x: -self.x, y: -self.y, z: -self.z}
    }
}

impl ops::Index<usize> for Point {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index for Point"),
        }
    }
}

impl ops::IndexMut<usize> for Point {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index for Point"),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use linalg::Vector3;

    #[test]
    fn op_add_point() {
        let v = Point{x: 10.0, y: 8.0, z: 6.0};
        let v2 = v + v;

        assert_eq!(v2.x, 20.0);
        assert_eq!(v2.y, 16.0);
        assert_eq!(v2.z, 12.0);
    }

    #[test]
    fn op_sub_point() {
        let v = Point{x: 10.0, y: 8.0, z: 6.0};
        let v2 = v - v;

        assert_eq!(v2.x, 0.0);
        assert_eq!(v2.y, 0.0);
        assert_eq!(v2.z, 0.0);
    }

    #[test]
    fn op_sub_vector3() {
        let v = Point{x: 10.0, y: 8.0, z: 6.0};
        let v2 = Vector3{x: 4.0, y: 2.0, z: 2.0};
        let v3 = v - v2;

        assert_eq!(v3.x, 6.0);
        assert_eq!(v3.y, 6.0);
        assert_eq!(v3.z, 4.0);
    }

    #[test]
    fn op_mul_scalar() {
        let v = Point{x: 10.0, y: 8.0, z: 6.0};
        let v2 = v * 2.0;

        assert_eq!(v2.x, 20.0);
        assert_eq!(v2.y, 16.0);
        assert_eq!(v2.z, 12.0);
    }

    #[test]
    fn op_neg() {
        let v = Point{x: 10.0, y: -8.0, z: 6.0};
        let v2 = -v;

        assert_eq!(v2.x, -10.0);
        assert_eq!(v2.y, 8.0);
        assert_eq!(v2.z, -6.0);
    }

    #[test]
    fn op_div_nonzero_scalar() {
        let v = Point{x: 10.0, y: 8.0, z: 6.0};
        let v2 = v / 2.0;

        assert_eq!(v2.x, 5.0);
        assert_eq!(v2.y, 4.0);
        assert_eq!(v2.z, 3.0);
    }

    #[test]
    #[should_panic]
    fn op_div_zero_scalar() {
        let v = Point{x: 10.0, y: 8.0, z: 6.0};
        v / 0.0;
    }

    #[test]
    fn op_index() {
        let v = Point{x: 10.0, y: 8.0, z: 6.0};

        assert_eq!(v[0], 10.0);
        assert_eq!(v[1], 8.0);
        assert_eq!(v[2], 6.0);
    }

    #[test]
    fn op_mut_index() {
        let mut v = Point{x: 10.0, y: 8.0, z: 6.0};

        v[0] = 2.0;
        v[1] = 1.0;
        v[2] = 0.0;

        assert_eq!(v[0], 2.0);
        assert_eq!(v[1], 1.0);
        assert_eq!(v[2], 0.0);
    }

}
