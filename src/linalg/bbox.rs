use linalg::{
    Point3,
    BaseNum,
};


pub struct BBox {
    pub p_min: Point3,
    pub p_max: Point3,
}

impl BBox {
    fn volume(&self) -> f64 {
        let d = self.p_max - self.p_min;
        d.x * d.y * d.z
    }
}



