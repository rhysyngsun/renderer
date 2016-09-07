
use core::Light;
use primative::{Aggregate, TransformedPrimative};
use linalg::{Ray, Intersection};

pub struct Scene {
    pub aggregate: Box<Aggregate>,
    pub lights: Vec<Box<Light>>, // volume_region: VolumeRegion,
}

impl Scene {
    /// Construct a  new scene
    pub fn new() -> Scene {
        Scene {
            aggregate: Box::new(TransformedPrimative { dummy: true }),
            lights: vec![],
        }
    }

    /// Calculate the intersection of a camera ray with the scene
    pub fn intersect<'a>(&'a self, ray: &'a Ray, isect: &'a Intersection) -> bool {
        (*self.aggregate).intersect(ray, isect)
    }
    /// Determine if a camera ray intersects with the scene
    pub fn intersect_p<'a>(&'a self, ray: &'a Ray) -> bool {
        (*self.aggregate).intersect_p(ray)
    }
}
