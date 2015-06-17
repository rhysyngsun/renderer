use linalg::{
    Point3,
    Vector3,
    BaseNum,
};


/// A ray (point + vector + time)
pub struct Ray<N> {
    origin: Point3<N>,
    dir: Vector3<N>,

    min_t: N,
    max_t: N,
    time: N,

    depth: usize,
}

impl<N : BaseNum> Ray<N> {
    fn new(origin: Point3<N>,
           dir: Vector3<N>,
           start: N,
           end: N,
           t: N,
           d: usize) -> Self {
        Ray {
            origin: origin,
            dir: dir,
            min_t: start,
            max_t: end,
            time: t,
            depth: d
        }
    }

    /// Creates a new child Ray.
    fn new_child(origin: Point3<N>,
                 dir: Vector3<N>,
                 parent: &Ray<N>,
                 start: N,
                 end: N) -> Self {
        Ray {
            origin: origin,
            dir: dir,
            min_t: start,
            max_t: end,
            time: parent.time,
            depth: parent.depth + 1
        }
    }
}
