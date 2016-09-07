use linalg::{Point3, Vector3};


/// A ray (point + vector + time)
pub struct Ray {
    origin: Point3,
    dir: Vector3,

    min_t: f64,
    max_t: f64,
    time: f64,

    depth: usize,
}

impl Ray {
    /// Constructs a new ray
    fn new(origin: Point3, dir: Vector3, start: f64, end: f64, t: f64, d: usize) -> Self {
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
    fn new_child(origin: Point3, dir: Vector3, parent: &Ray, start: f64, end: f64) -> Self {
        Ray {
            origin: origin,
            dir: dir,
            min_t: start,
            max_t: end,
            time: parent.time,
            depth: parent.depth + 1,
        }
    }
}
