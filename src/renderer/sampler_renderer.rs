use core::{
    Camera,
    Integrator,
    Sampler,
    Scene,
};
use film::Spectrum;

use linalg::{
    RayDifferential,
};

use integrator::{
    SurfaceIntegrator,
    VolumeIntegrator,
};

use renderer::Renderer;

pub struct SamplerRenderer <'a, S : Sampler, C : Camera> {
    pub sampler: S,
    pub camera: C,
    pub surface_integrator: &'a SurfaceIntegrator,
    pub volume_integrator: &'a VolumeIntegrator,
}

impl<'a, S : Sampler, C : Camera> SamplerRenderer<'a, S, C> {
    fn new(sampler: S,
           camera: C,
           surface_integrator: &'a SurfaceIntegrator,
           volume_integrator: &'a VolumeIntegrator) -> SamplerRenderer<'a, S, C> {
        SamplerRenderer {
            sampler: sampler,
            camera: camera,
            surface_integrator: surface_integrator,
            volume_integrator: volume_integrator,
        }
    }
}

impl<'a, S : Sampler, C : Camera> Renderer for SamplerRenderer<'a, S, C> {
    fn render(&self, scene: &Scene) {
        self.surface_integrator.preprocess(scene, &self.camera);
        self.volume_integrator.preprocess(scene, &self.camera);
    }

    #[allow(unused_variables)]
    fn li(&self, scene: &Scene, ray: &RayDifferential, sample: &Sampler) -> Box<Spectrum> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn transmittance(&self, scene: &Scene, ray: &RayDifferential, sample: &Sampler) -> Box<Spectrum> {
        unimplemented!()
    }
}
