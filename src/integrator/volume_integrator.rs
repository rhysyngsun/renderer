
use scene::{
    Camera,
    Scene,
};

use integrator::Integrator;

pub struct VolumeIntegrator;

impl Integrator for VolumeIntegrator {
    fn preprocess(&self, scene: &Scene, camera: &Camera) {
    }
}
