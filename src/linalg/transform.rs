

pub struct Matrix4x4 {
    pub m: [f64;16]
}

pub struct Transform {
    m: Matrix4x4,
    m_inv: Matrix4x4,
}
