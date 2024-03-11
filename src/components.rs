//! Entities are whatever property that can be attached to a given 
//! entity. Typically a position or the fact that an entity can be
//! rendered on the map are both examples of components.
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

use bracket_lib::{color::ColorPair, terminal::{FontCharType, Point}};

/// This component simply means that the entity it is attached to
/// IS a player. That's it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

/// The position of an entity on the map
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// The capacity of an entity to be rendered on screen
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

/// The name of a given entity
#[derive(Clone, Debug, PartialEq)]
pub struct Name(String);


impl Position {
    pub fn into_point(self) -> Point {
        Point::new(self.x, self.y)
    }
}