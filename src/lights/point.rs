
use linalg::Point3f64;

use core::Light;

pub struct PointLight {
    pub position: Point3f64,
}

impl Light for PointLight {
    fn n_samples(&self) -> u8 {
        64
    }
}
