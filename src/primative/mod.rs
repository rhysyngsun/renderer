use linalg::Ray;
use linalg::Intersection;

pub trait Primative {
    fn can_intersect(&self) -> bool;
    fn intersect(&self, ray: &Ray, isect: &Intersection) -> bool;
    fn intersect_p(&self, ray: &Ray) -> bool;
}
