pub use self::camera::Camera;

mod camera;

use primative::Primative;
use light::Light;
use linalg::Ray;
use linalg::Intersection;

type Aggregate = Box<Primative + 'static>;

pub struct Scene {
    aggregate: Aggregate,
    lights: Vec<Box<Light>>,
    //volume_region: VolumeRegion,
}

impl Scene {
    pub fn intersect<'a>(&'a self, ray: &'a Ray, isect: &'a Intersection) -> bool {
        (*self.aggregate).intersect(ray, isect)
    }

    pub fn intersect_p<'a>(&'a self, ray: &'a Ray) -> bool {
        (*self.aggregate).intersect_p(ray)
    }
}
