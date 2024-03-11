//! This module only comprises a bunch of utility functions
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

use crate::{Map, Position, TileType};

/// Returns true iff the entity is allowed to move on to the next position (x,y)
pub fn can_enter(map: &Map, x: isize, y: isize) -> bool {
    if x < 0 || x >= map.width as isize {
        return false;
    }
    if y < 0 || y >= map.height as isize {
        return false;
    }

    let dest = Position {x: x as usize, y: y as usize};
    map[dest] == TileType::Floor
}

/// Returns the next position after the movement has been applied. If the movement is not
/// legal, then the position is simply not updated
pub fn next_position(map: &Map, curr: Position, movement: (isize, isize)) -> Position {
    let x = curr.x as isize + movement.0;
    let y = curr.y as isize + movement.1;

    if !can_enter(map, x, y) {
        curr
    } else {
        Position {x: x as usize, y: y as usize}
    }
}