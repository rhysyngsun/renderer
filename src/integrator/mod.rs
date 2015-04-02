
mod surface_integrator;
mod volume_integrator;

use scene::Scene;
use scene::Camera;

pub trait Integrator {
    fn preprocess(&self, scene: &Scene, camera: &Camera);
}
