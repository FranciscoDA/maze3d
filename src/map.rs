use std::vec::Vec;
use std::{array, fmt};

/// Converts a position from an N-dimensional array into a position from a 1-D array
fn pos2i<const D: usize>(dimensions: &[usize; D], position: [usize; D]) -> usize {
    if (0..D).any(|i| position[i] >= dimensions[i]) {
        return usize::max_value();
    }
    let mut multiplier: usize = 1;
    let mut index = 0;
    for i in (0..D).rev() {
        index += position[i] * multiplier;
        multiplier *= dimensions[i];
    }
    return index;
}

pub struct Map<const D: usize> {
    tiles: Vec<i8>,
    dimensions: [usize; D],
}

impl<const D: usize> Map<D> {
    pub fn from(value: i8, dimensions: [usize; D]) -> Self {
        let size: usize = dimensions.into_iter().fold(1, |a, e| a * e);
        let tiles = vec![value; size];
        Map { tiles, dimensions }
    }
}

pub const WALL_EAST: i8 = 1 << 0;
pub const WALL_NORTH: i8 = 1 << 1;
pub const WALL_WEST: i8 = 1 << 2;
pub const WALL_SOUTH: i8 = 1 << 3;

pub struct MapSlice<'a, const D: usize> {
    map: &'a mut Map<D>,
    origin: [usize; D],
    dimensions: [usize; D],
}

impl<'a, const D: usize> MapSlice<'a, D> {
    pub fn from(map: &'a mut Map<D>, origin: [usize; D], dimensions: [usize; D]) -> Self {
        Self {
            map: map,
            origin,
            dimensions,
        }
    }
}

pub trait RectangularMap<const D: usize> {
    fn dimensions(&self) -> [usize; D];
}

impl<const D: usize> RectangularMap<D> for Map<D> {
    fn dimensions(&self) -> [usize; D] {
        self.dimensions
    }
}

impl<'a, const D: usize> RectangularMap<D> for MapSlice<'a, D> {
    fn dimensions(&self) -> [usize; D] {
        self.dimensions
    }
}

pub trait GetSetMap<P: TryInto<usize> + Copy, const D: usize>
where
    Self: RectangularMap<D>,
{
    fn contains(&self, position: [P; D]) -> bool;
    fn get_item(&self, position: [P; D]) -> i8;
    fn set_item(&mut self, position: [P; D], value: i8);
}

impl<P: TryInto<usize> + Copy, const D: usize> GetSetMap<P, D> for Map<D> {
    fn contains(&self, position: [P; D]) -> bool {
        (0..D).all(|i| match position[i].try_into() {
            Ok(ui) => ui < self.dimensions[i],
            _ => false,
        })
    }
    fn get_item(&self, position: [P; D]) -> i8 {
        let p = position.map(|v| v.try_into().unwrap_or(0));
        *self.tiles.get(pos2i(&self.dimensions, p)).unwrap_or(&0)
    }
    fn set_item(&mut self, position: [P; D], value: i8) {
        let p = position.map(|v| v.try_into().unwrap_or(0));
        let i = pos2i(&self.dimensions, p);
        if i < self.tiles.len() {
            self.tiles[i] = value;
        }
    }
}

impl<'a, P: TryInto<usize> + Copy, const D: usize> GetSetMap<P, D> for MapSlice<'a, D> {
    fn contains(&self, position: [P; D]) -> bool {
        (0..D).all(|i| match position[i].try_into() {
            Ok(ui) => ui + self.origin[i] < self.dimensions[i],
            _ => false,
        })
    }
    fn get_item(&self, position: [P; D]) -> i8 {
        let p = array::from_fn(|i| position[i].try_into().unwrap_or(0) + self.origin[i]);
        *self.map.tiles.get(pos2i(&self.map.dimensions, p)).unwrap_or(&0)
    }

    fn set_item(&mut self, position: [P; D], value: i8) {
        let p = array::from_fn(|i| position[i].try_into().unwrap_or(0) + self.origin[i]);
        let i = pos2i(&self.map.dimensions, p);
        if i < self.map.tiles.len() {
            self.map.tiles[i] = value;
        }
    }
}

impl fmt::Display for Map<2> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.dimensions[0] {
            for vertical_edge in [WALL_NORTH, WALL_SOUTH] {
                for j in 0..self.dimensions[1] {
                    let v = self.get_item([i, j]);
                    for horizontal_edge in [WALL_WEST, WALL_EAST] {
                        let flags = v & (vertical_edge | horizontal_edge);
                        write!(
                            f,
                            "{}",
                            if flags == WALL_WEST | WALL_NORTH {
                                "┌─"
                            } else if flags == WALL_WEST {
                                "│ "
                            } else if flags == WALL_WEST | WALL_SOUTH {
                                "└─"
                            } else if flags == WALL_EAST | WALL_NORTH {
                                "─┐"
                            } else if flags == WALL_EAST {
                                " │"
                            } else if flags == WALL_EAST | WALL_SOUTH {
                                "─┘"
                            } else if flags == WALL_NORTH {
                                "──"
                            } else if flags == WALL_SOUTH {
                                "──"
                            } else {
                                "  "
                            }
                            .to_string(),
                        )?;
                    }
                }
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}
