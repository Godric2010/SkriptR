use super::entitiy_component::Component;

pub struct Transform{

    pub position: [f32; 3],
    pub rotation: f32,
    pub scale: f32,

    is_active: bool,

}

impl Transform{

    pub fn new() -> Self{
        Transform { position: [0.0, 0.0, 0.0], rotation: 0.0, scale: 0.0, is_active: true }
    }

    pub fn get_transform_matrix(&self) -> [[f32; 4]; 4]{
        let c = self.rotation.cos() * self.scale;
        let s = self.rotation.sin() * self.scale;
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

impl Component for Transform{
    fn enable(&mut self) {
        self.is_active = true;
    }

    fn disable(&mut self) {

    }

    fn update(&self) {
        println!("Update transform!")
    }
}