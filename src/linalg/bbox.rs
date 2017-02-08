use linalg::point::Point3;


pub struct BBox {
    pub p_min: Point3,
    pub p_max: Point3,
}

impl BBox {
    /// Constructs a new bounding box
    ///
    /// # Examples
    ///
    /// ```
    /// use renderer::linalg::bbox::BBox;
    /// use renderer::linalg::point::Point3;
    ///
    /// let bb = BBox::new(Point3 {x: 0.0, y: 0.0, z: 0.0}, Point3 {x: 1.0, y: 2.0, z: 3.0});
    /// ```
    pub fn new(p_min: Point3, p_max: Point3) -> BBox {
        BBox {
            p_min: p_min,
            p_max: p_max,
        }
    }

    /// Computes the volume of a bounding box
    ///
    /// # Examples
    ///
    /// ```
    /// use renderer::linalg::bbox::BBox;
    /// use renderer::linalg::point::Point3;
    ///
    /// let bb = BBox::new(Point3 {x: 0.0, y: 0.0, z: 0.0}, Point3 {x: 1.0, y: 2.0, z: 3.0});
    /// let vol = bb.volume();
    ///
    /// assert_eq!(6.0, vol);
    /// ```
    pub fn volume(&self) -> f64 {
        let d = self.p_max - self.p_min;
        d.x * d.y * d.z
    }
}
