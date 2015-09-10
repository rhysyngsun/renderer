use core::CameraSample;

use linalg::{
    Ray,
    RayDifferential,
};

pub trait Camera {
    fn generate_ray(&self, sample: &CameraSample, ray: &mut Ray<f64>) -> f64;
    fn generate_ray_differential(&self, sample: &CameraSample, ray: &mut RayDifferential) -> f64;
}
