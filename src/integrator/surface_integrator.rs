
use scene::{
    Camera,
    Scene,
};

use integrator::Integrator;

pub struct SurfaceIntegrator;

impl Integrator for SurfaceIntegrator {
    fn preprocess(&self, scene: &Scene, camera: &Camera) {
    }
}
