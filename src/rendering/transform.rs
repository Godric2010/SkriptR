pub struct Transform {
    pub position: [f32; 3],
    pub angle: f32,
    pub scale: f32,
}

impl Transform {
    pub fn idle() -> Self{
        Transform{
            position: [0.0, 0.0, 0.0],
            angle: 0.0,
            scale: 1.0,
        }
    }
}

pub fn make_transform_matrix(transform: &Transform) -> [[f32; 4]; 4]{
    let c = transform.angle.cos() * transform.scale;
    let s = transform.angle.sin() * transform.scale;
    let [dx, dy, dz] = transform.position;

    [
        [c, 0.0, s, 0.0],
        [0.0, transform.scale, 0.0, 0.0],
        [-s, 0.0, c, 0.0],
        [dx, dy, dz, 1.0],
    ]
}