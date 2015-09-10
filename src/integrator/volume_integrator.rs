use core::{
    Camera,
    Scene,
    Integrator,
};

pub struct VolumeIntegrator;

impl Integrator for VolumeIntegrator {
    #[allow(unused_variables)]
    fn preprocess<T : Camera>(&self, scene: &Scene, camera: &T) {
    }
}
