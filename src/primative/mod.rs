use linalg::{
    Ray,
    Intersection,
};

use light::AreaLight;

pub trait Primative {
    fn can_intersect(&self) -> bool;
    fn intersect(&self, ray: &Ray<f64>, isect: &Intersection) -> bool;
    fn intersect_p(&self, ray: &Ray<f64>) -> bool;
}


pub struct GeometricPrimative;
pub struct TransformedPrimative;
pub struct AnimatedPrimative;


pub trait Aggregate: Primative {
}
