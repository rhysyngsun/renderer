
use linalg::ops::Zero;
use linalg::point::Point3;
use linalg::ray::Ray;
use linalg::vector::Vector3;

pub struct RayDifferential {
    pub ray: Ray,
    pub has_differentials: bool,
    pub rx_origin: Point3,
    pub ry_origin: Point3,
    pub rx_direction: Vector3,
    pub ry_direction: Vector3,
}

impl RayDifferential {
    pub fn new(origin: Point3, dir: Vector3, start: f64, end: f64, t: f64, d: usize) -> Self {
        RayDifferential::from(Ray::new(origin, dir, start, end, t, d))
    }

    /// Creates a new child RayDifferential.
    pub fn new_child(origin: Point3, dir: Vector3, parent: &Ray, start: f64, end: f64) -> Self {
        RayDifferential::from(Ray::new_child(origin, dir, parent, start, end))
    }

    pub fn scale_differentials(&mut self, scale: f64) {
        self.rx_origin = self.ray.origin + (self.rx_origin - self.ray.origin) * scale;
        self.ry_origin = self.ray.origin + (self.ry_origin - self.ray.origin) * scale;
        self.rx_direction = self.ray.dir + (self.rx_direction - self.ray.dir) * scale;
        self.ry_direction = self.ray.dir + (self.ry_direction - self.ray.dir) * scale;
    }
}

impl From<Ray> for RayDifferential {
    fn from(ray: Ray) -> Self {
        RayDifferential {
            ray: ray,
            has_differentials: false,
            rx_origin: Point3::zero(),
            ry_origin: Point3::zero(),
            rx_direction: Vector3::zero(),
            ry_direction: Vector3::zero(),
        }
    }
}
