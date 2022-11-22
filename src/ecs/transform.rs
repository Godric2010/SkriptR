use std::any::Any;
use super::component::Component;

pub struct Transform{

    pub position: [f32; 3],
    pub rotation: f32,
    pub scale: f32,

    is_active: bool,

}

impl Transform{

    pub fn new() -> Self{
        Transform { position: [0.0, 0.0, 0.0], rotation: 0.0, scale: 1.0, is_active: true }
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32){
        self.position = [x, y, z];
    }

    pub fn set_rotation(&mut self, angle: f32){
        self.rotation = angle;
    }

    pub fn set_scale(&mut self, scale: f32){
        self.scale = scale;
    }

    pub fn get_transform_matrix(&self) -> [[f32; 4]; 4]{
        let c = self.rotation.to_radians().cos() * self.scale;
        let s = self.rotation.to_radians().sin() * self.scale;
        let [dx, dy, dz] = self.position;

        let matrix =
        [
            [c, 0.0, s, 0.0],
            [0.0, self.scale, 0.0, 0.0],
            [-s, 0.0, c, 0.0],
            [dx, dy, dz, 1.0],
        ];

        matrix
    }
}

/*impl Component<'_> for Transform{
    fn enable(&mut self) {
        self.is_active = true;
    }

    fn disable(&mut self) {

    }

    fn update(&mut self) {

       let mut rotation = self.rotation;
        rotation += 1.0;

       // self.set_position(position[0], position[1], position[2])
        self.set_rotation(rotation);
    }

    fn as_any(&self) -> &dyn Any {
       self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}*/