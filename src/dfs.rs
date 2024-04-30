use crate::map::{GetSetMap, WALL_EAST, WALL_NORTH, WALL_SOUTH, WALL_WEST};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::vec::Vec;

const NEIGHBORHOOD_4: [[i32; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];
const _NEIGHBORHOOD_8: [[i32; 2]; 8] = [[1, 0], [-1, 0], [0, 1], [0, -1], [1, 1], [-1, -1], [-1, 1], [1, -1]];

pub fn get_neighborhood<const N: usize>(
    map: &impl GetSetMap<i32, 2>,
    row: i32,
    col: i32,
    offsets: &[[i32; 2]; N],
) -> Vec<[i32; 2]> {
    Vec::from_iter(
        offsets
            .into_iter()
            .map(|[row_offset, col_offset]| [row + row_offset, col + col_offset])
            .filter(|[r, c]| map.contains([*r, *c])),
    )
}

/// Randomized depth-first-search algorithm for maze generation
pub fn random_dfs(map: &mut impl GetSetMap<i32, 2>, start: [i32; 2]) {
    let mut stack: Vec<([i32; 2], Option<[i32; 2]>)> = vec![(start, None)];

    while let Some((current_position, previous)) = stack.pop() {
        let [row, col] = current_position;

        let mut test_cells = get_neighborhood(map, row, col, &NEIGHBORHOOD_4);
        test_cells.sort();

        if let Some([prev_row, prev_col]) = previous {
            if let Ok(pos) = test_cells.binary_search(&[prev_row, prev_col]) {
                test_cells.remove(pos);
            }
            for prev_neigh in get_neighborhood(map, prev_row, prev_col, &NEIGHBORHOOD_4) {
                if let Ok(pos) = test_cells.binary_search(&prev_neigh) {
                    test_cells.remove(pos);
                }
            }
        }

        let current_value = map.get_item([row, col]);
        if current_value != (WALL_EAST | WALL_NORTH | WALL_WEST | WALL_SOUTH) {
            continue;
        }

        if let Some(previous_position) = previous {
            if previous_position == [row, col - 1] {
                map.set_item(previous_position, map.get_item(previous_position) & !WALL_EAST);
                map.set_item(current_position, map.get_item(current_position) & !WALL_WEST);
            } else if previous_position == [row, col + 1] {
                map.set_item(previous_position, map.get_item(previous_position) & !WALL_WEST);
                map.set_item(current_position, map.get_item(current_position) & !WALL_EAST);
            } else if previous_position == [row - 1, col] {
                map.set_item(previous_position, map.get_item(previous_position) & !WALL_SOUTH);
                map.set_item(current_position, map.get_item(current_position) & !WALL_NORTH);
            } else if previous_position == [row + 1, col] {
                map.set_item(previous_position, map.get_item(previous_position) & !WALL_NORTH);
                map.set_item(current_position, map.get_item(current_position) & !WALL_SOUTH);
            }
        }

        let mut neighbours = get_neighborhood(map, row, col, &NEIGHBORHOOD_4);
        neighbours.shuffle(&mut thread_rng());
        for neighbour in neighbours {
            stack.push((neighbour, Some(current_position)));
        }
    }
}
