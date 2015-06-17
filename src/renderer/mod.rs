use scene::Scene;
use linalg::RayDifferential;
use sampler::Sampler;
use film::Spectrum;

pub use self::sampler_renderer::{
    SamplerRenderer,
};

mod sampler_renderer;

trait Renderer {
    fn render(&self, scene: &Scene);
    fn li(&self, scene: &Scene, ray: &RayDifferential, sample: &Sampler) -> Box<Spectrum>;
    fn transmittance(&self, scene: &Scene, ray: &RayDifferential, sample: &Sampler) -> Box<Spectrum>;
}
