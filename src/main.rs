pub mod assets;
pub mod constants;
pub mod debug_ui;
pub mod dfs;
pub mod drawable;
pub mod entities;
pub mod events;
pub mod game;
pub mod input;
pub mod map;
pub mod camera;

use crate::{
    assets::AssetPack,
    constants::{
        MOUSE_SENSITIVITY, PI, PLAYER_SPEED, PLAYER_SPRINT_SPEED, PLAYER_WALK_SPEED, SCREEN_H, SCREEN_W,
        TARGET_FPS, TILE_SIZE,
    },
    debug_ui::{draw_debug_text, draw_xyz_indicator},
    drawable::Drawable,
    entities::Entity,
    events::GameEventType,
    game::GameState,
    input::InputController,
    map::{GetSetMap, WALL_EAST, WALL_NORTH, WALL_SOUTH, WALL_WEST},
};
use camera::{get_xz_plane_parallel_rotation_matrix, get_camera_rotation_matrix};
use raylib::prelude::*;
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

    let mut game = GameState::new(rl.get_time(), [5, 5]);

    let player_position = game.player().position();

    let mut camera = raylib::camera::Camera3D::perspective(
        player_position,
        player_position + Vector3::forward().transform_with(game.camera_rotation),
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

        let (player_position, player_collision_radius, player_id) = {
            let player = game.player();
            (player.position(), player.collision_radius(), player.id())
        };

        let mut translation_velocity = Vector3::zero();

        let player_input_enabed = game.game_start_event.as_ref().is_none()
            && game.game_end_event.as_ref().is_none()
            && !game
                .roll_events
                .last()
                .as_ref()
                .is_some_and(|e| !e.is_completed(game.clock));

        if player_input_enabed {
            translation_velocity += input
                .get_movement_vector(&rl)
                .transform_with(get_xz_plane_parallel_rotation_matrix(game.camera_rotation))
                .normalized()
                .mul(input.get_move_speed_modifier(&rl, PLAYER_WALK_SPEED, PLAYER_SPEED, PLAYER_SPRINT_SPEED));

            game.camera_rotation *= get_camera_rotation_matrix(
                game.camera_rotation,
                input.get_turning_angle(&rl) * 0.1,
                input.get_vertical_look_angle(&rl) * 0.05
            );
        }
        input.last_mouse_position = rl.get_mouse_position();

        // detect collision vs walls and adjust velocity accordingly
        let mut colliding = false;
        {
            let player_position2d = Vector2::new(player_position.x, player_position.z);
            let translated_position = Vector2::new(
                player_position.x + translation_velocity.x,
                player_position.z + translation_velocity.z,
            );
            let translated_bb = Rectangle::new(
                translated_position.x - player_collision_radius,
                translated_position.y - player_collision_radius,
                player_collision_radius * 2.0,
                player_collision_radius * 2.0,
            );
            // find all nearby walls
            let mut walls = Vec::<Rectangle>::new();
            for i in ((translated_bb.y) / TILE_SIZE) as i32
                ..((translated_bb.y + translated_bb.height) / TILE_SIZE) as i32 + 1
            {
                for j in ((translated_bb.x) / TILE_SIZE) as i32
                    ..((translated_bb.x + translated_bb.width) / TILE_SIZE) as i32 + 1
                {
                    if !game.map.contains([i, j]) {
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
            walls.sort_by(|a, b| (-fdist(a)).total_cmp(&-fdist(b)));
            while let Some(wall_bb) = walls.pop() {
                let translation_velocity2d = Vector2::new(translation_velocity.x, translation_velocity.z);
                let translated_position2d = player_position2d + translation_velocity2d;
                if let Some(collision_point) =
                    rec_circle_collision_point(&wall_bb, translated_position2d, player_collision_radius)
                {
                    colliding = true;
                    // find a vector to remove the player from the wall and add that to the velocity
                    let closest_point = Vector2::new(
                        player_position2d.x.clamp(wall_bb.x, wall_bb.x + wall_bb.width),
                        player_position2d.y.clamp(wall_bb.y, wall_bb.y + wall_bb.height),
                    );
                    let collision_edge_normal = (player_position2d - closest_point).normalized();
                    let correction = collision_edge_normal
                        * (player_collision_radius - collision_edge_normal.dot(translated_position2d - collision_point));
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
        for e in game.roll_events.iter() {
            if e.is_completed(game.clock) {
                match e {
                    GameEventType::Roll { entity_id, .. } => {
                        game.entities.remove_by_id(*entity_id);
                    }
                    _ => {}
                }
            }
        }
        if game.roll_events.iter().all(|e| e.is_completed(game.clock)) && game.roll_events.len() % 2 == 0 {
            game.roll_events.clear();
        }

        // Collision vs other entities
        {
            let mut i = 0;
            while i < game.entities.len() {
                let player_position2d = Vector2::new(player_position.x, player_position.z);
                let e = game.entities.get_mut_by_index(i);
                let entity_position2d = Vector2::new(e.position().x, e.position().z);

                let objects_colliding = e.collision_radius() > 0.0
                    && check_collision_circles(
                        entity_position2d,
                        e.collision_radius(),
                        player_position2d,
                        player_collision_radius,
                    )
                    && e.id() != player_id;
                colliding |= objects_colliding;
                match e {
                    Entity::End { .. } => {
                        if objects_colliding && game.game_end_event.is_none() {
                            game.game_end_event = Some(GameEventType::GameEnd {
                                start_time: game.clock,
                                duration: 1.0,
                            });
                        }
                    }
                    Entity::Dodecahedron { .. } => {
                        // When colliding with a diamond create a new camera roll event
                        if objects_colliding {
                            if game.roll_events.iter().all(|ev| match ev {
                                GameEventType::Roll { entity_id, .. } => *entity_id != e.id(),
                                _ => false,
                            }) {
                                let new_event = GameEventType::Roll {
                                    start_time: game.clock,
                                    duration: 1.0,
                                    entity_id: e.id(),
                                };
                                game.roll_events.push(new_event);
                            }
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
        }

        game.player_mut().move_position(translation_velocity);
        camera.position = game.player().position();
        camera.target = game.player().position() + Vector3::forward().transform_with(game.camera_rotation);
        camera.up = Vector3::up()
            .transform_with(game.camera_rotation)
            .transform_with(Matrix::rotate(
                Vector3::forward().transform_with(game.camera_rotation),
                game.roll_events
                    .iter()
                    .map(|e| e.elapsed_normalized(game.clock) as f32)
                    .fold(0.0, |a, b| a + b)
                    * PI,
            ));

        rl.update_camera(&mut camera);

        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
            {
                let mut d3d = d.begin_mode3D(camera);
                // draw tile map
                game.map.draw(&game, &mut d3d, &camera, &texture);

                // Draw entities z-ordered
                game.entities.sort_drawables_by(|a, b| {
                    f32::total_cmp(
                        &(b.position() - camera.position).length(),
                        &(a.position() - camera.position).length(),
                    )
                });
                d3d.draw_circle_3D(
                    Vector3::new(game.player().position().x, 0.1, game.player().position().z),
                    game.player().collision_radius(),
                    Vector3::right(),
                    90.0,
                    if colliding { Color::RED } else { Color::GREEN },
                );
                for e in game.entities.draw_iter() {
                    e.draw(&game, &mut d3d, &camera, &texture);
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
