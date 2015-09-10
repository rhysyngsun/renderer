use core::{
    Sampler,
    Scene,
};
use linalg::RayDifferential;
use film::Spectrum;

mod sampler_renderer;

pub use self::sampler_renderer::{
    SamplerRenderer,
};

trait Renderer {
    fn render(&self, scene: &Scene);
    fn li(&self, scene: &Scene, ray: &RayDifferential, sample: &Sampler) -> Box<Spectrum>;
    fn transmittance(&self, scene: &Scene, ray: &RayDifferential, sample: &Sampler) -> Box<Spectrum>;
}
