use linalg::{
    Point3,
    BaseNum,
};


pub struct BBox<N> {
    pub p_min: Point3<N>,
    pub p_max: Point3<N>,
}

impl<N : BaseNum> BBox<N> {
    fn volume(&self) -> N {
        let d = self.p_max - self.p_min;
        d.x * d.y * d.z
    }
}



