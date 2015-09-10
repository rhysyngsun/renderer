use core::{
    Camera,
    Scene,
};

pub trait Integrator {
    fn preprocess<T : Camera>(&self, scene: &Scene, camera: &T);
}
