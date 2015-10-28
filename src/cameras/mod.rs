

pub struct PerspectiveCamera {
    pub pos: Vector3,


    dx_camera: Vector3,
    dy_camera: Vector3,
}


pub impl PerspectiveCamera {
    pub fn new(pos: Vector3) -> PerspectiveCamera {
        PerspectiveCamera {
            pos: pos,
        }
    }
}
