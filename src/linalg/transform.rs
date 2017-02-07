
use linalg::{Cross,Vector3};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat4(pub [f64; 16]);

/// Compute the inverse of a matrix
#[allow(unused_variables)]
pub fn inverse(m: &Mat4) -> Mat4 {
   unimplemented!()
}

/// A 4x4 matrix, typically used to represent transformations in 3D space.
impl Mat4 {
    pub fn zero() -> Mat4 {
        Mat4([
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0
        ])
    }

    /// Identify matrix
    ///
    /// [ 1, 0, 0, 0,
    ///   0, 1, 0, 0,
    ///   0, 0, 1, 0,
    ///   0, 0, 0, 1 ]
    ///
    /// Examples:
    ///
    /// ```
    /// use renderer::linalg::transform::Mat4;
    ///
    /// let ident = Mat4::identity();
    /// ```
    pub fn identity() -> Mat4 {
        Mat4([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ])
    }
}


#[derive(Debug, PartialEq)]
pub struct Transform {
    m: Mat4,
    m_inv: Mat4,
}


impl Transform {
    /// Constructs a new transform from a given transformation matrix
    pub fn new(m: Mat4) -> Transform {
        let m_inv = inverse(&m);

        Transform {
            m: m,
            m_inv: m_inv,
        }
    }


    /// Identity transform
    pub fn identity() -> Transform {
        Transform::new(Mat4::identity())
    }

    /// Convenience method for constructing a look_at transformation.
    ///
    /// Typically used for cameras.
    ///
    /// Examples:
    ///
    /// ``` norun
    /// use renderer::linalg::ops::{Zero,One};
    /// use renderer::linalg::Vector3;
    /// use renderer::linalg::transform::Transform;
    ///
    /// let pos = Vector3::zero();
    /// let look = Vector3::one();
    /// let up = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
    ///
    /// let t = Transform::look_at(pos, look, up);
    /// ```
    pub fn look_at(pos: Vector3, look: Vector3, up: Vector3) -> Transform {
        let dir = (look - pos).normalize();
        let left = (up.normalize().cross(&dir)).normalize();
        let new_up = dir.cross(&left);

        let m = Mat4([
            left.x, new_up.x, dir.x, pos.x,
            left.y, new_up.y, dir.y, pos.y,
            left.z, new_up.z, dir.z, pos.z,
            0.0,    0.0,     0.0,   1.0
        ]);

        Transform::new(m)
    }
}
