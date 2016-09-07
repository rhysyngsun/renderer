
use linalg::Point3;

use core::Light;

/// Point light
pub struct PointLight {
    pub position: Point3,
}

impl Light for PointLight {
    /// Returns the number of samples taken for this light
    fn n_samples(&self) -> u8 {
        64
    }
}
