pub struct Camera{
    pub fov: f32,
    pub ratio: f32,
    pub range_min: f32,
    pub range_max: f32,
    pub is_perspective: bool,
}

impl Camera {
    pub fn new(fov: f32, range: [f32;2], is_perspective: bool) -> Self{
        Camera{
            fov,
            range_min: range[0],
            range_max: range[1],
            is_perspective,
            ratio: 4.0 / 3.0,
        }
    }
}

