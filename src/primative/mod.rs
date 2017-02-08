use linalg::ray::Ray;
use linalg::intersection::Intersection;


pub trait Primative {
    fn can_intersect(&self) -> bool;
    fn intersect(&self, ray: &Ray, isect: &Intersection) -> bool;
    fn intersect_p(&self, ray: &Ray) -> bool;
}

pub struct GeometricPrimative;

pub struct TransformedPrimative {
    pub dummy: bool,
}
pub struct AnimatedPrimative;

pub trait Aggregate: Primative {}

impl Primative for TransformedPrimative {
    fn can_intersect(&self) -> bool {
        false
    }

    #[allow(unused_variables)]
    fn intersect(&self, ray: &Ray, isect: &Intersection) -> bool {
        false
    }

    #[allow(unused_variables)]
    fn intersect_p(&self, ray: &Ray) -> bool {
        false
    }
}
impl Aggregate for TransformedPrimative {}
