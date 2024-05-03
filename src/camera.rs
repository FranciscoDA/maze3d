use raylib::{ffi::asinf, prelude::*};


/// Returns a matrix with the same rotation as the given matrix, except that the new rotation matrix will only rotate around the world's y-axis
pub fn get_xz_plane_parallel_rotation_matrix(camera_rotation: Matrix) -> Matrix {
    camera_rotation * Matrix::rotate(
        Vector3::right().transform_with(camera_rotation),
        unsafe { asinf(Vector3::forward().transform_with(camera_rotation).normalized().y) }
    )
}


/// Returns a rotation matrix with the given horizontal and vertical angles relative to the camera's rotation matrix
pub fn get_camera_rotation_matrix(camera_rotation: Matrix, horizontal_turn_angle: f32, vertical_look_angle: f32) -> Matrix {
    let horizontal_rotation = Matrix::rotate(Vector3::up(), horizontal_turn_angle);

    let vertical_rotation_axis = Vector3::left().transform_with(camera_rotation * horizontal_rotation);
    let vertical_rotation_angle = vertical_look_angle;

    let old_up = Vector3::up().transform_with(camera_rotation);
    let vertical_rotation = Matrix::rotate(vertical_rotation_axis, vertical_rotation_angle);

    // constrain vertical rotation to avoid spinning by looking up/down
    let new_up = Vector3::up().transform_with(camera_rotation * horizontal_rotation * vertical_rotation);
    horizontal_rotation * vertical_rotation * Matrix::rotate(
        vertical_rotation_axis,
        vertical_rotation_angle
            * if new_up.y < 0.05 && old_up.y - new_up.y != 0.0 {
                (new_up.y - 0.05) / (old_up.y - new_up.y)
            } else {
                0.0
            },
    )
}
