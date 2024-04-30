use raylib::prelude::*;
use std::cmp::Ordering;

pub enum Entity {
    Start { position: Vector3 },
    End { position: Vector3 },
    Rat { position: Vector3 },
    OpenGL { position: Vector3 },
}

impl Entity {
    pub fn position(&self) -> Vector3 {
        match self {
            Self::Start { position } | Self::End { position } | Self::Rat { position } | Self::OpenGL { position } => {
                *position
            }
        }
    }

    pub fn collision_radius(&self) -> f32 {
        match self {
            Self::End { .. } => 1.2,
            _ => 0.0,
        }
    }
}

pub struct EntityManager {
    entities: Vec<Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager { entities: Vec::new() }
    }

    pub fn add(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn sort_by(&mut self, compare: impl FnMut(&Entity, &Entity) -> Ordering) {
        self.entities.sort_by(compare);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.entities.iter()
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }
}
