//! The resources are the 'passive' stuffs that are interacted with during the game
//! even though they do not strictly belong to the game.
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

use std::ops::Index;

use crate::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map{
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileType>
}

impl Index<Position> for Map {
    type Output = TileType;

    fn index(&self, Position{x, y}: Position) -> &Self::Output {
        &self.tiles[y*self.width + x]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
}
