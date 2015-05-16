use linalg::Point3f64;
use linalg::Vector3;

pub struct Ray {
    origin: Point3f64,
    dir: Vector3<f64>,

    min_t: f64,
    max_t: f64,
    time: f64,

    depth: usize,
}

impl Ray {
    fn new(origin: Point3f64, dir: Vector3<f64>,
           start: f64, end: f64, t: f64, d: usize) -> Self {
        Ray {
            origin: origin,
            dir: dir,
            min_t: start,
            max_t: end,
            time: t,
            depth: d
        }
    }
    fn new_child(origin: Point3f64, dir: Vector3<f64>, parent: &Ray,
           start: f64, end: f64) -> Self {
        Ray {
            origin: origin,
            dir: dir,
            min_t: start,
            max_t: end,
            time: parent.time,
            depth: parent.depth + 1
        }
    }

    fn along(&self, t: f64) -> Point3f64 {
        self.origin + self.dir * t
    }
}
