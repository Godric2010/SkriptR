use glm::{Matrix4, Vector3};
use crate::rendering::camera::Camera;
use crate::rendering::transform::Transform;

pub fn get_camera_projection_matrix(camera: &Camera) -> [[f32; 4]; 4]{
    let mat =
        glm::ext::perspective(glm::radians(camera.fov), camera.ratio, camera.range_min, camera.range_max);

    to_matrix_array(mat)
}

pub fn get_camera_view_matrix(transform: &Transform) -> [[f32; 4]; 4] {
   
    let camera_pos: Vector3<f32> = glm::vec3(transform.position[0], transform.position[1], transform.position[2]);
    let look_point: Vector3<f32> = glm::vec3(0.0, 0.0, 0.0);
    let up_vector: Vector3<f32> = glm::vec3(0.0, 1.0, 0.0);
    let view = glm::ext::look_at(camera_pos, look_point, up_vector);

    to_matrix_array(view)
}

fn to_matrix_array(mat :Matrix4<f32>) -> [[f32; 4]; 4]{
    return [[mat.c0.x, mat.c0.y, mat.c0.z, mat.c0.w], [mat.c1.x, mat.c1.y, mat.c1.z, mat.c1.w], [mat.c2.x, mat.c2.y, mat.c2.z, mat.c2.w], [mat.c3.x, mat.c3.y, mat.c3.z, mat.c3.w]];
}