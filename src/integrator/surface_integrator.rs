use core::{Camera, Scene, Integrator};

pub struct SurfaceIntegrator;

impl Integrator for SurfaceIntegrator {
    #[allow(unused_variables)]
    fn preprocess<T: Camera>(&self, scene: &Scene, camera: &T) {}
}
