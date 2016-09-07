use linalg::{Vector3, Point3};

use std::collections::HashMap;

pub enum ParamType {
    Float,
    Integer,
    String,
    Point,
    Vector,
    Rgb,
}

pub struct ParamSet {
    floats: HashMap<String, f64>,
    ints: HashMap<String, i32>,
    strings: HashMap<String, String>,
    points: HashMap<String, Point3>,
    vectors: HashMap<String, Vector3>
}


impl ParamSet {
    pub fn new() -> ParamSet {
        ParamSet {
            floats: HashMap::new(),
            ints: HashMap::new(),
            strings: HashMap::new(),
            points: HashMap::new(),
            vectors: HashMap::new(),
        }
    }
}
