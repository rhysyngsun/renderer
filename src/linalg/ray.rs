use linalg::Point;
use linalg::Vector3;

pub struct Ray {
    o: Point,
    d: Vector3<f64>,

    min_t: f64,
    max_t: f64,
    time: f64,

    depth: usize,
}

impl Ray {
    fn new(origin: Point, dir: Vector3<f64>,
           start: f64, end: f64, t: f64, d: usize) -> Self {
        Ray {
            o: origin,
            d: dir,
            min_t: start,
            max_t: end,
            time: t,
            depth: d
        }
    }
    fn new_child(origin: Point, dir: Vector3<f64>, parent: &Ray,
           start: f64, end: f64) -> Self {
        Ray {
            o: origin,
            d: dir,
            min_t: start,
            max_t: end,
            time: parent.time,
            depth: parent.depth + 1
        }
    }

    fn along(&self, t: f64) -> Point {
        self.o * self.d * t
    }
}
