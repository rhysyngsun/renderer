
use core::Light;
use primative::{
    Aggregate,
    TransformedPrimative,
};
use linalg::{
    Ray,
    Intersection,
};

pub struct Scene {
    pub aggregate: Box<Aggregate>,
    pub lights: Vec<Box<Light>>,
    //volume_region: VolumeRegion,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            aggregate: box TransformedPrimative {
                dummy: true,
            },
            lights: vec![],
        }
    }

    pub fn intersect<'a>(&'a self, ray: &'a Ray<f64>, isect: &'a Intersection) -> bool {
        (*self.aggregate).intersect(ray, isect)
    }

    pub fn intersect_p<'a>(&'a self, ray: &'a Ray<f64>) -> bool {
        (*self.aggregate).intersect_p(ray)
    }
}
