
use linalg::Vector3;

use std::num::{One, Zero};

pub struct PerspectiveCamera {
    pub pos: Vector3,

    dx_camera: Vector3,
    dy_camera: Vector3,
}


impl PerspectiveCamera {
    pub fn new(pos: Vector3) -> PerspectiveCamera {
        PerspectiveCamera {
            pos: pos,

            dx_camera: Vector3::zero(),
            dy_camera: Vector3::one(),
        }
    }
}
