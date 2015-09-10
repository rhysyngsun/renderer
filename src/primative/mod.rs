use linalg::{
    Ray,
    Intersection,
};

pub trait Primative {
    fn can_intersect(&self) -> bool;
    fn intersect(&self, ray: &Ray<f64>, isect: &Intersection) -> bool;
    fn intersect_p(&self, ray: &Ray<f64>) -> bool;
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
    fn intersect(&self, ray: &Ray<f64>, isect: &Intersection) -> bool {
        false
    }

    #[allow(unused_variables)]
    fn intersect_p(&self, ray: &Ray<f64>) -> bool {
        false
    }
}
impl Aggregate for TransformedPrimative {}
