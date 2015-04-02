use scene::Scene;
use linalg::RayDifferential;
use sampler::Sampler;
use film::Spectrum;

trait Renderer {
    fn render(&self, scene: &Scene);
    fn li(&self, scene: &Scene, ray: &RayDifferential, sample: &Sampler) -> Spectrum;
    fn transmittance(&self, scene: &Scene, ray: &RayDifferential, sample: &Sampler) -> Spectrum;
}
