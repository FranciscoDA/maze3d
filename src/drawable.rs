use raylib::math::Rectangle;
use raylib::prelude::*;

use crate::assets::AssetPack;
use crate::constants::TILE_SIZE;
use crate::entities::Entity;
use crate::game::GameState;
use crate::map::{GetSetMap, Map, RectangularMap, WALL_EAST, WALL_NORTH, WALL_SOUTH, WALL_WEST};

pub trait Drawable<T> {
    fn draw(&self, game: &GameState, d3d: &mut RaylibMode3D<T>, camera: Camera3D, assets: &AssetPack);
}

impl<T> Drawable<T> for Entity {
    fn draw(&self, game: &GameState, d3d: &mut RaylibMode3D<T>, camera: Camera3D, assets: &AssetPack) {
        //let scale_vector = Vector3::new(1.0, y_axis_scale, 1.0);

        let draw_scaled_billboard = |d3d: &mut RaylibMode3D<T>, tex: &Texture2D, size: f32, color: Color| {
            let y_axis_scale = if let Some(e) = &game.game_start_event {
                e.elapsed_normalized(game.clock) as f32
            } else if let Some(e) = &game.game_end_event {
                1.0 - e.elapsed_normalized(game.clock) as f32
            } else {
                1.0
            };

            d3d.draw_billboard_rec(
                camera,
                &tex,
                Rectangle::new(
                    0.0,
                    tex.height() as f32 * (1.0 - y_axis_scale),
                    tex.width() as f32,
                    tex.height() as f32 * y_axis_scale,
                ),
                self.position() * Vector3::new(1.0, y_axis_scale, 1.0),
                size,
                color,
            );
        };

        if self.collision_radius() > 0.0 {
            d3d.draw_circle_3D(
                Vector3::new(self.position().x, 0.1, self.position().z),
                self.collision_radius(),
                Vector3::right(),
                90.0,
                Color::GREEN,
            )
        }

        match self {
            Entity::Start { .. } => {
                draw_scaled_billboard(d3d, &assets.tex_start, 3.0, Color::new(255, 255, 255, 128))
            }
            Entity::Rat { .. } => draw_scaled_billboard(d3d, &assets.tex_rat, 2.0, Color::WHITE),
            Entity::OpenGL { .. } => {
                draw_scaled_billboard(d3d, &assets.tex_opengl, 3.0, Color::new(255, 255, 255, 154))
            }
            Entity::End { .. } => draw_scaled_billboard(d3d, &assets.tex_smiley, 3.0, Color::new(255, 255, 255, 128)),
        }
    }
}

impl<T> Drawable<T> for Map<2> {
    fn draw(&self, game: &GameState, d3d: &mut RaylibMode3D<T>, _camera: Camera3D, texture: &AssetPack) {
        //let wall_height = game.game_time.min(1.0) as f32 * TILE_SIZE;
        let wall_height = TILE_SIZE
            * if let Some(e) = &game.game_start_event {
                e.elapsed_normalized(game.clock) as f32
            } else if let Some(e) = &game.game_end_event {
                1.0 - e.elapsed_normalized(game.clock) as f32
            } else {
                1.0
            };
        for i in 0..self.dimensions()[0] {
            for j in 0..self.dimensions()[1] {
                let x = j as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                let z = i as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                d3d.draw_cube_texture(
                    &texture.tex_floor,
                    Vector3::new(x, 0.0, z),
                    TILE_SIZE,
                    0.01,
                    TILE_SIZE,
                    Color::WHITE,
                );
                d3d.draw_cube_texture(
                    &texture.tex_ceiling,
                    Vector3::new(x, TILE_SIZE, z),
                    TILE_SIZE,
                    0.01,
                    TILE_SIZE,
                    Color::WHITE,
                );
                let v = self.get_item([i, j]);
                if v == WALL_EAST | WALL_NORTH | WALL_WEST | WALL_SOUTH {
                    d3d.draw_cube_texture(
                        &texture.tex_wall,
                        Vector3::new(x, wall_height / 2.0, z),
                        TILE_SIZE,
                        wall_height,
                        TILE_SIZE,
                        Color::WHITE,
                    );
                } else {
                    if v & WALL_EAST == WALL_EAST {
                        d3d.draw_cube_texture(
                            &texture.tex_wall,
                            Vector3::new(x + TILE_SIZE / 2.0, wall_height / 2.0, z),
                            0.001,
                            wall_height,
                            TILE_SIZE,
                            Color::WHITE,
                        );
                    }
                    if v & WALL_NORTH == WALL_NORTH {
                        d3d.draw_cube_texture(
                            &texture.tex_wall,
                            Vector3::new(x, wall_height / 2.0, z - TILE_SIZE / 2.0),
                            TILE_SIZE,
                            wall_height,
                            0.001,
                            Color::WHITE,
                        );
                    }
                    if v & WALL_WEST == WALL_WEST {
                        d3d.draw_cube_texture(
                            &texture.tex_wall,
                            Vector3::new(x - TILE_SIZE / 2.0, wall_height / 2.0, z),
                            0.001,
                            wall_height,
                            TILE_SIZE,
                            Color::WHITE,
                        );
                    }
                    if v & WALL_SOUTH == WALL_SOUTH {
                        d3d.draw_cube_texture(
                            &texture.tex_wall,
                            Vector3::new(x, wall_height / 2.0, z + TILE_SIZE / 2.0),
                            TILE_SIZE,
                            wall_height,
                            0.001,
                            Color::WHITE,
                        );
                    }
                }
            }
        }
    }
}
