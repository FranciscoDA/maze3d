use raylib::prelude::*;
use std::cmp::Ordering;

pub enum Entity {
    Start { id: usize, position: Vector3 },
    End { id: usize, position: Vector3 },
    Rat { id: usize, position: Vector3 },
    OpenGL { id: usize, position: Vector3 },
    Dodecahedron { id: usize, position: Vector3 },
}

impl Entity {
    pub fn id(&self) -> usize {
        match self {
            Self::Start { id, .. }
            | Self::End { id, .. }
            | Self::Rat { id, .. }
            | Self::OpenGL { id, .. }
            | Self::Dodecahedron { id, .. } => *id,
        }
    }

    pub fn position(&self) -> Vector3 {
        match self {
            Self::Start { position, .. }
            | Self::End { position, .. }
            | Self::Rat { position, .. }
            | Self::OpenGL { position, .. }
            | Self::Dodecahedron { position, .. } => *position,
        }
    }

    pub fn collision_radius(&self) -> f32 {
        match self {
            Self::End { .. } => 1.2,
            Self::Dodecahedron { .. } => 1.2,
            _ => 0.0,
        }
    }
}

pub struct EntityManager {
    entities: Vec<Entity>,
    id_sequence: usize,
    draw_order: Vec<usize>,
}

fn update_index_after_removal(vec: &mut Vec<usize>, removed_index: usize) {
    let mut i = 0;
    while i < vec.len() {
        match vec[i].cmp(&removed_index) {
            Ordering::Equal => {
                vec.remove(i);
            }
            Ordering::Greater => {
                vec[i] -= 1;
                i += 1;
            }
            _ => {
                i += 1;
            }
        };
    }
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            entities: Vec::new(),
            id_sequence: 0,
            draw_order: Vec::new(),
        }
    }

    pub fn generate_id(&mut self) -> usize {
        let result = self.id_sequence;
        self.id_sequence += 1;
        result
    }

    pub fn get_mut_by_index(&mut self, index: usize) -> &mut Entity {
        &mut self.entities[index]
    }

    pub fn remove_by_index(&mut self, index: usize) {
        self.entities.remove(index);
        update_index_after_removal(&mut self.draw_order, index);
    }

    pub fn get_mut_by_id(&mut self, id: usize) -> Option<&mut Entity> {
        if let Ok(i) = self.entities.binary_search_by_key(&id, |a| a.id()) {
            return Some(&mut self.entities[i]);
        }
        return None;
    }

    pub fn remove_by_id(&mut self, id: usize) -> bool {
        if let Ok(i) = self.entities.binary_search_by_key(&id, |a| a.id()) {
            self.entities.remove(i);
            update_index_after_removal(&mut self.draw_order, i);
            return true;
        }
        return false;
    }

    pub fn add(&mut self, entity: Entity) {
        self.draw_order.push(self.entities.len());
        self.entities.push(entity);
    }

    pub fn sort_drawables_by<F>(&mut self, mut compare: F)
    where
        F: FnMut(&Entity, &Entity) -> Ordering,
    {
        self.draw_order
            .sort_by(|&i, &j| compare(&self.entities[i], &self.entities[j]));
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.entities.iter()
    }

    pub fn draw_iter(&self) -> impl Iterator<Item = &Entity> {
        self.draw_order.iter().map(|&i| &self.entities[i]).into_iter()
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }
}
