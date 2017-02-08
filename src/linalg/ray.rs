use linalg::point::Point3;
use linalg::vector::Vector3;


/// A ray (point + vector + time)
pub struct Ray {
    pub origin: Point3,
    pub dir: Vector3,

    pub min_t: f64,
    pub max_t: f64,
    pub time: f64,

    pub depth: usize,
}

impl Ray {
    /// Constructs a new ray
    pub fn new(origin: Point3, dir: Vector3, start: f64, end: f64, t: f64, d: usize) -> Self {
        Ray {
            origin: origin,
            dir: dir,
            min_t: start,
            max_t: end,
            time: t,
            depth: d,
        }
    }

    /// Creates a new child Ray.
    pub fn new_child(origin: Point3, dir: Vector3, parent: &Ray, start: f64, end: f64) -> Self {
        Ray {
            origin: origin,
            dir: dir,
            min_t: start,
            max_t: end,
            time: parent.time,
            depth: parent.depth + 1,
        }
    }

    fn along(self, time: f64) -> Point3 {
        self.origin + self.dir * time
    }
}
