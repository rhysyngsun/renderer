pub use self::camera::Camera;

mod camera;

use primative::Aggregate;
use light::Light;
use linalg::{
    Ray,
    Intersection,
};

pub struct Scene {
    aggregate: Box<Aggregate>,
    lights: Vec<Box<Light>>,
    //volume_region: VolumeRegion,
}

impl Scene {
    pub fn intersect<'a>(&'a self, ray: &'a Ray<f64>, isect: &'a Intersection) -> bool {
        (*self.aggregate).intersect(ray, isect)
    }

    pub fn intersect_p<'a>(&'a self, ray: &'a Ray<f64>) -> bool {
        (*self.aggregate).intersect_p(ray)
    }
}

