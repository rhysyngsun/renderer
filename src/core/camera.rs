use core::CameraSample;

use linalg::ray::Ray;
use linalg::ray_differential::RayDifferential;

/// Common camera methods
pub trait Camera {
    fn generate_ray(&self, sample: &CameraSample, ray: &mut Ray) -> f64;
    fn generate_ray_differential(&self, sample: &CameraSample, ray: &mut RayDifferential) -> f64;
}
