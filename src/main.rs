pub mod assets;
pub mod constants;
pub mod dfs;
pub mod drawable;
pub mod entities;
pub mod events;
pub mod game;
pub mod input;
pub mod map;
pub mod debug_ui;

use crate::{
    assets::AssetPack,
    constants::{
        MOUSE_SENSITIVITY, PLAYER_RADIUS, PLAYER_SPEED, PLAYER_SPRINT_SPEED, PLAYER_WALK_SPEED, SCREEN_H, SCREEN_W,
        TARGET_FPS, TILE_SIZE,
    },
    drawable::Drawable,
    entities::Entity,
    events::GameEventType,
    game::GameState,
    input::InputController,
    map::{GetSetMap, WALL_EAST, WALL_NORTH, WALL_SOUTH, WALL_WEST},
    debug_ui::draw_xyz_indicator,
};
use debug_ui::draw_debug_text;
use map::RectangularMap;
use raylib::{ffi::asinf, prelude::*};
use std::ops::Mul;

/// Raylib implements Rectangle::check_collision_circle_rec but it's bugged. Use this implementation instead
fn rec_circle_collision_point(rect: &Rectangle, center: Vector2, radius: f32) -> Option<Vector2> {
    let closest_point = Vector2::new(
        center.x.clamp(rect.x, rect.x + rect.width),
        center.y.clamp(rect.y, rect.y + rect.height),
    );
    if (center - closest_point).length_sqr() < radius * radius {
        Some(closest_point)
    } else {
        None
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(SCREEN_W, SCREEN_H).title("3d maze").build();

    let mut game = GameState::new(rl.get_time(), [8, 8]);

    let mut camera = raylib::camera::Camera3D::perspective(
        game.player_position,
        game.player_position + Vector3::forward().transform_with(game.camera_rotation),
        Vector3::up(),
        60.0,
    );
    rl.set_camera_mode(camera, CameraMode::CAMERA_CUSTOM);

    if MOUSE_SENSITIVITY != 0.0 {
        rl.disable_cursor();
    }
    rl.set_target_fps(TARGET_FPS);

    let texture = AssetPack::init(&mut rl, &thread);

    rl.set_mouse_scale(MOUSE_SENSITIVITY, MOUSE_SENSITIVITY);
    let mut input = InputController::new(&rl);

    while !rl.window_should_close() {
        game.clock = rl.get_time();

        let camera_z_rotation_axis = Vector3::left().transform_with(game.camera_rotation);
        let camera_z_rotation_angle =
            unsafe { asinf(Vector3::forward().transform_with(game.camera_rotation).normalized().y) };
        let mut translation_velocity = Vector3::zero();

        let player_input_enabed = game.game_start_event.is_none() && game.game_end_event.is_none();

        if player_input_enabed {
            translation_velocity += input
                .get_movement_vector(&rl)
                .transform_with(game.camera_rotation)
                .transform_with(Matrix::rotate(camera_z_rotation_axis, -camera_z_rotation_angle))
                .mul(Vector3::one() - Vector3::up())
                .normalized()
                .mul(input.get_move_speed_modifier(&rl, PLAYER_WALK_SPEED, PLAYER_SPEED, PLAYER_SPRINT_SPEED));

            game.camera_rotation *= Matrix::rotate(Vector3::up(), input.get_turning_angle(&rl) * 0.1);

            let vertical_rotation_axis = Vector3::left().transform_with(game.camera_rotation);
            let vertical_rotation_angle = input.get_vertical_look_angle(&rl) * 0.05;
            game.camera_rotation *= Matrix::rotate(vertical_rotation_axis, vertical_rotation_angle);

            // constrain vertical rotation to avoid spinning by looking up/down
            let new_up = Vector3::up().transform_with(game.camera_rotation) * Vector3::up();
            game.camera_rotation *= Matrix::rotate(
                vertical_rotation_axis,
                vertical_rotation_angle
                    * if new_up.y < 0.05 && camera.up.y - new_up.y != 0.0 {
                        (new_up.y - 0.05) / (camera.up.y - new_up.y)
                    } else {
                        0.0
                    },
            );
        }
        input.last_mouse_position = rl.get_mouse_position();

        // detect collision vs walls and adjust velocity accordingly
        let mut colliding = false;
        {
            let player_position2d = Vector2::new(game.player_position.x, game.player_position.z);
            let translated_position = Vector2::new(
                game.player_position.x + translation_velocity.x,
                game.player_position.z + translation_velocity.z,
            );
            let translated_bb = Rectangle::new(
                translated_position.x - PLAYER_RADIUS,
                translated_position.y - PLAYER_RADIUS,
                PLAYER_RADIUS * 2.0,
                PLAYER_RADIUS * 2.0,
            );
            // find all nearby walls
            let mut walls = Vec::<Rectangle>::new();
            for i in ((translated_bb.y) / TILE_SIZE) as i32
                ..((translated_bb.y + translated_bb.height) / TILE_SIZE) as i32 + 1
            {
                for j in ((translated_bb.x) / TILE_SIZE) as i32
                    ..((translated_bb.x + translated_bb.width) / TILE_SIZE) as i32 + 1
                {
                    if i < 0
                        || j < 0
                        || i as usize >= game.map.dimensions()[0]
                        || j as usize >= game.map.dimensions()[1]
                    {
                        continue;
                    }
                    if game.map.get_item([i, j]) & WALL_EAST == WALL_EAST {
                        walls.push(Rectangle::new(
                            (j + 1) as f32 * TILE_SIZE,
                            i as f32 * TILE_SIZE,
                            0.001,
                            TILE_SIZE,
                        ));
                    }
                    if game.map.get_item([i, j]) & WALL_WEST == WALL_WEST {
                        walls.push(Rectangle::new(
                            j as f32 * TILE_SIZE,
                            i as f32 * TILE_SIZE,
                            0.001,
                            TILE_SIZE,
                        ));
                    }
                    if game.map.get_item([i, j]) & WALL_SOUTH == WALL_SOUTH {
                        walls.push(Rectangle::new(
                            j as f32 * TILE_SIZE,
                            (i + 1) as f32 * TILE_SIZE,
                            TILE_SIZE,
                            0.001,
                        ));
                    }
                    if game.map.get_item([i, j]) & WALL_NORTH == WALL_NORTH {
                        walls.push(Rectangle::new(
                            j as f32 * TILE_SIZE,
                            i as f32 * TILE_SIZE,
                            TILE_SIZE,
                            0.001,
                        ));
                    }
                }
            }
            // apply wall collisions from closest to player to farthest
            let fdist = |a: &Rectangle| {
                Vector2::new(
                    a.x + a.width / 2.0 - player_position2d.x,
                    a.y + a.height / 2.0 - player_position2d.y,
                )
                .length_sqr()
            };
            walls.sort_by( |a, b| (-fdist(a)).total_cmp(&-fdist(b)) );
            while let Some(wall_bb) = walls.pop() {
                let translation_velocity2d = Vector2::new(translation_velocity.x, translation_velocity.z);
                let translated_position2d = player_position2d + translation_velocity2d;
                if let Some(collision_point) =
                    rec_circle_collision_point(&wall_bb, translated_position2d, PLAYER_RADIUS)
                {
                    colliding = true;
                    // find a vector to remove the player from the wall and add that to the velocity
                    let closest_point = Vector2::new(
                        player_position2d.x.clamp(wall_bb.x, wall_bb.x + wall_bb.width),
                        player_position2d.y.clamp(wall_bb.y, wall_bb.y + wall_bb.height),
                    );
                    let collision_edge_normal = (player_position2d - closest_point).normalized();
                    let correction = collision_edge_normal
                        * (PLAYER_RADIUS - collision_edge_normal.dot(translated_position2d - collision_point));
                    translation_velocity.x += correction.x;
                    translation_velocity.z += correction.y;
                }
            }
        }

        if game
            .game_start_event
            .as_ref()
            .is_some_and(|e| e.is_completed(game.clock))
        {
            game.game_start_event = None;
        }
        if game.game_end_event.as_ref().is_some_and(|e| e.is_completed(game.clock)) {
            game = GameState::new(game.clock, [8, 8]);
            continue;
        }

        // Collision vs other entities
        for e in game.entities.iter() {
            let player_position2d = Vector2::new(game.player_position.x, game.player_position.z);
            let entity_position2d = Vector2::new(e.position().x, e.position().z);
            match e {
                Entity::End { .. } => {
                    if check_collision_circles(entity_position2d, e.collision_radius(), player_position2d, PLAYER_RADIUS) {
                        colliding = true;
                        if game.game_end_event.is_none() {
                            game.game_end_event = Some(GameEventType::GameEnd {
                                start_time: game.clock, duration: 1.0,
                            });
                        }
                    }
                }
                _ => {}
            }
        }

        game.player_position += translation_velocity;
        camera.position = game.player_position;
        camera.target = game.player_position + Vector3::forward().transform_with(game.camera_rotation);
        camera.up = Vector3::up().transform_with(game.camera_rotation) * Vector3::up();
        rl.update_camera(&mut camera);

        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
            {
                let mut d3d = d.begin_mode3D(camera);
                // draw tile map
                game.map.draw(&game, &mut d3d, camera, &texture);

                // Draw entities z-ordered
                game.entities.sort_by(|a, b| {
                    f32::total_cmp(
                        &(b.position() - camera.position).length(),
                        &(a.position() - camera.position).length(),
                    )
                });
                d3d.draw_circle_3D(
                    Vector3::new(game.player_position.x, 0.1, game.player_position.z),
                    PLAYER_RADIUS,
                    Vector3::right(),
                    90.0,
                    if colliding { Color::RED } else { Color::GREEN },
                );
                for e in game.entities.iter() {
                    e.draw(&game, &mut d3d, camera, &texture);
                }
            }

            draw_debug_text(&mut d, &camera, &game, translation_velocity);
            draw_xyz_indicator(
                &mut d,
                game.camera_rotation,
                Vector2::new(SCREEN_W as f32 - 40.0, SCREEN_H as f32 - 40.0),
                30.0,
            );
        }
    }
}