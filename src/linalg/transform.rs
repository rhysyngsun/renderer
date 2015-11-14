

#[derive(Copy, Clone)]
pub struct Mat4(pub [f64; 16]);


pub fn inverse(m: &Mat4) -> Mat4 {
    m.clone()
}

pub struct Transform {
    m: Mat4,
    m_inv: Mat4,
}


impl Transform {

    pub fn new(mat: [f64; 16]) -> Transform {
        let m = Mat4(mat);
        let m_inv = inverse(&m);

        Transform {
            m: m,
            m_inv: m_inv,
        }
    }


    pub fn identity() -> Transform {
        Transform::new([1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                        1.0])
    }
}
