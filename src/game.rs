use crate::{
    constants::TILE_SIZE,
    dfs::random_dfs,
    entities::{Entity, EntityManager},
    events::GameEventType,
    map::{GetSetMap, Map, RectangularMap, WALL_EAST, WALL_NORTH, WALL_SOUTH, WALL_WEST},
};
use rand::seq::SliceRandom;
use raylib::ffi::atan2f;
use raylib::prelude::*;

pub struct GameState {
    pub map: Map<2>,
    pub entities: EntityManager,

    pub clock: f64,
    pub player_position: Vector3,
    pub camera_rotation: Matrix,

    pub game_start_event: Option<GameEventType>,
    pub game_end_event: Option<GameEventType>,
    pub roll_events: Vec<GameEventType>,
}

impl GameState {
    pub fn new(clock: f64, map_dimensions: [usize; 2]) -> Self {
        let mut map = Map::<2>::from(WALL_EAST | WALL_NORTH | WALL_WEST | WALL_SOUTH, map_dimensions);
        let mut entities = EntityManager::new();
        random_dfs(&mut map, [0, 0]);

        // Find all free tiles where we can put game objects
        let mut free_tiles = Vec::<[usize; 2]>::new();
        for i in 0..map.dimensions()[0] {
            for j in 0..map.dimensions()[1] {
                if map.get_item([i, j]) != WALL_EAST | WALL_NORTH | WALL_WEST | WALL_SOUTH {
                    free_tiles.push([i, j]);
                }
            }
        }
        free_tiles.shuffle(&mut rand::thread_rng());

        // find start and end positions
        let [start_row, start_col] = free_tiles.pop().unwrap_or([0, 0]);
        let [facing_row, facing_col] = {
            if map.get_item([start_row, start_col]) & WALL_SOUTH == 0 {
                [start_row + 1, start_col]
            } else if map.get_item([start_row, start_col]) & WALL_NORTH == 0 {
                [start_row - 1, start_col]
            } else if map.get_item([start_row, start_col]) & WALL_EAST == 0 {
                [start_row, start_col + 1]
            } else if map.get_item([start_row, start_col]) & WALL_WEST == 0 {
                [start_row, start_col - 1]
            } else {
                [start_row, start_col]
            }
        };

        free_tiles.retain(|&v| v != [facing_row as usize, facing_col as usize]);
        let [end_row, end_col] = free_tiles.pop().unwrap_or([start_row, start_col]);

        // initialize entities: 3 rats, 2 opengl logos, 1 start indicator
        let map_offset = Vector3::new(TILE_SIZE / 2.0, 0.0, TILE_SIZE / 2.0);
        while let Some([entity_row, entity_col]) = free_tiles.pop() {
            let (x, z) = (entity_col as f32 * TILE_SIZE, entity_row as f32 * TILE_SIZE);
            let entity = if entities.len() < 3 {
                Entity::Rat {
                    id: entities.generate_id(),
                    position: Vector3::new(x, 0.5, z) + map_offset,
                }
            } else if entities.len() < 5 {
                Entity::OpenGL {
                    id: entities.generate_id(),
                    position: Vector3::new(x, 1.5, z) + map_offset,
                }
            } else if entities.len() < 9 {
                Entity::Dodecahedron {
                    id: entities.generate_id(),
                    position: Vector3::new(x, 1.5, z) + map_offset,
                }
            } else {
                break;
            };
            entities.add(entity);
        }
        let start_banner = Entity::Start {
            id: entities.generate_id(),
            position: Vector3::new(
                facing_col as f32 * TILE_SIZE,
                TILE_SIZE / 2.0,
                facing_row as f32 * TILE_SIZE,
            ) + map_offset,
        };
        let end_banner = Entity::End {
            id: entities.generate_id(),
            position: Vector3::new(end_col as f32 * TILE_SIZE, TILE_SIZE / 2.0, end_row as f32 * TILE_SIZE)
                + map_offset,
        };

        let player_position = Vector3::new(
            start_col as f32 * TILE_SIZE,
            TILE_SIZE / 2.0,
            start_row as f32 * TILE_SIZE,
        ) + map_offset;
        let camera_rotation = Matrix::rotate_y(unsafe {
            atan2f(
                player_position.x - start_banner.position().x,
                start_banner.position().z - player_position.z,
            )
        });

        entities.add(start_banner);
        entities.add(end_banner);

        return Self {
            map,
            entities,
            player_position,
            camera_rotation,
            clock,
            game_start_event: Some(GameEventType::GameStart {
                start_time: clock,
                duration: 1.0,
            }),
            game_end_event: None,
            roll_events: Vec::new(),
        };
    }

    pub fn update_events(&mut self) {
        if let Some(e) = &self.game_start_event {
            if e.is_completed(self.clock) {
                self.game_start_event.take();
            }
        }
        if let Some(e) = &self.game_end_event {
            if e.is_completed(self.clock) {
                self.game_end_event.take();
            }
        }
    }
}
