

use scene::Scene;
use scene::Camera;

pub use self::surface_integrator::SurfaceIntegrator;
pub use self::volume_integrator::VolumeIntegrator;

mod surface_integrator;
mod volume_integrator;

pub trait Integrator {
    fn preprocess(&self, scene: &Scene, camera: &Camera);
}
