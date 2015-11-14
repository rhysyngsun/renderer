
use linalg::Point3;

use core::Light;

pub struct PointLight {
    pub position: Point3,
}

impl Light for PointLight {
    fn n_samples(&self) -> u8 {
        64
    }
}
