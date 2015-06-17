
use sampler::Sampler;
use scene::{
    Camera,
    Scene,
};
use film::Spectrum;

use linalg::{
    RayDifferential,
};

use integrator::{
    Integrator,
    SurfaceIntegrator,
    VolumeIntegrator,
};

use renderer::Renderer;

pub struct SamplerRenderer<'a> {
    sampler: &'a Sampler,
    camera: &'a Camera,
    surface_integrator: &'a SurfaceIntegrator,
    volume_integrator: &'a VolumeIntegrator,
}

impl<'a> SamplerRenderer<'a> {
    fn new(sampler: &'a Sampler,
           camera: &'a Camera,
           surface_integrator: &'a SurfaceIntegrator,
           volume_integrator: &'a VolumeIntegrator) -> SamplerRenderer<'a> {
        SamplerRenderer {
            sampler: sampler,
            camera: camera,
            surface_integrator: surface_integrator,
            volume_integrator: volume_integrator,
        }
    }
}

impl<'a> Renderer for SamplerRenderer<'a> {
    fn render(&self, scene: &Scene) {
        self.surface_integrator.preprocess(scene, &self.camera);
        self.volume_integrator.preprocess(scene, &self.camera);
    }
    fn li(&self, scene: &Scene, ray: &RayDifferential, sample: &Sampler) -> Box<Spectrum> {
        unimplemented!()
    }
    fn transmittance(&self, scene: &Scene, ray: &RayDifferential, sample: &Sampler) -> Box<Spectrum> {
        unimplemented!()
    }
}
