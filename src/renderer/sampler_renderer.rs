
use sampler::Sampler;
use scene::Camera;
use film::Spectrum;

pub struct SamplerRenderer {
    sampler: Sampler,
    camera: Camera,
}

impl Renderer for SamplerRenderer {
    fn render(&self, scene: &Scene) {
    }
    fn li(&self, scene: &Scene, ray: &RayDifferential, sample: &Sampler) -> Spectrum {
    }
    fn transmittance(&self, scene: &Scene, ray: &RayDifferential, sample: &Sampler) -> Spectrum {
    }
}
