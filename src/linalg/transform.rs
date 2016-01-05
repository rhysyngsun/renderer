
use linalg::Vector3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat4(pub [f64; 16]);


pub fn inverse(m: &Mat4) -> Mat4 {
    m.clone()
}

impl Mat4 {
    pub fn zero() -> Mat4 {
        Mat4([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])
    }
    pub fn identity() -> Mat4 {
        Mat4([1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0])
    }
}

#[derive(Debug, PartialEq)]
pub struct Transform {
    m: Mat4,
    m_inv: Mat4,
}


impl Transform {
    pub fn new(m: Mat4) -> Transform {
        let m_inv = inverse(&m);

        Transform {
            m: m,
            m_inv: m_inv,
        }
    }


    pub fn identity() -> Transform {
        Transform::new(Mat4::identity())
    }

    pub fn look_at(position: Vector3, look: Vector3, up: Vector3) -> Transform {
        let m = Mat4::zero();

        Transform::new(m)
    }
}
