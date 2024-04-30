use raylib::prelude::*;

use crate::game::GameState;

pub fn draw_debug_text(d: &mut RaylibDrawHandle, camera: &Camera, game: &GameState, player_velocity: Vector3) {
    let debug_string = format!(
        "\
        - Position: {:.3?}\n\
        - Target: {:.3?}\n\
        - Up: {:.3?}\n\
        - Direction: {:.3?}\n\
        - Velocity: {:.3?}\n\
        - Clock: {:.2}",
        camera.position,
        camera.target,
        camera.up,
        Vector3::forward().transform_with(game.camera_rotation),
        player_velocity,
        game.clock,
    );
    d.draw_text(debug_string.as_str(), 10, 10, 20, Color::WHITE);
}

pub fn draw_xyz_indicator(d: &mut RaylibDrawHandle, camera_rotation: Matrix, position: Vector2, scale: f32) {
    for (vec, col) in [(Vector3::right(), Color::RED), (Vector3::up(), Color::GREEN), (Vector3::forward(), Color::BLUE)]
    {
        let rosetta_vec = vec.transform_with(camera_rotation) * scale;
        d.draw_line_ex(
            position,
            position + Vector2::new(rosetta_vec.x, -rosetta_vec.y),
            scale / 10.0,
            col,
        );
    }
}
