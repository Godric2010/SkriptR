use glm::Vector3;
use crate::camera::Camera;
use crate::transform::Transform;

pub fn get_camera_mvp_matrix(camera: &Camera, transform: &Transform) -> [[f32; 4]; 4] {
    let projection =
        glm::ext::perspective(glm::radians(camera.fov), camera.ratio, camera.range_min, camera.range_max);

    let camera_pos: Vector3<f32> = glm::vec3(transform.position[0], transform.position[1], transform.position[2]);
    let look_point: Vector3<f32> = glm::vec3(0.0, 0.0, 0.0);
    let up_vector: Vector3<f32> = glm::vec3(0.0, 1.0, 0.0);
    let view = glm::ext::look_at(camera_pos, look_point, up_vector);

    let model = glm::Matrix4::new(
        glm::Vector4::new(1.0, 0.0, 0.0, 0.0),
            glm::Vector4::new(0.0, 1.0, 0.0, 0.0),
            glm::Vector4::new(0.0, 0.0, 1.0, 0.0),
            glm::Vector4::new(0.0, 0.0, 0.0, 1.0),
    );

    let mvp = projection * view * model;

    // let identity_mat = glm::Matrix4::new(
    //     glm::Vector4::new(1.0, 0.0, 0.0, 0.0),
    //     glm::Vector4::new(0.0, 1.0, 0.0, 0.0),
    //     glm::Vector4::new(0.0, 0.0, 1.0, 0.0),
    //     glm::Vector4::new(0.0, 0.0, 0.0, 1.0));
    //
    // let my_mat = identity_mat * vec3(10.0,0.0,0.0);

    return [[mvp.c0.x, mvp.c0.y, mvp.c0.z, mvp.c0.w], [mvp.c1.x, mvp.c1.y, mvp.c1.z, mvp.c1.w], [mvp.c2.x, mvp.c2.y, mvp.c2.z, mvp.c2.w], [mvp.c3.x, mvp.c3.y, mvp.c3.z, mvp.c3.w]];
}