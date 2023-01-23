use bevy::prelude::*;

use crate::tiles::TileTypeMap;

#[derive(Component, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
#[derive(Component)]
pub struct Collision;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Visible(VisibleTypes, ExploreTypes);

impl Visible {
    pub fn new() -> Visible {
        Visible(VisibleTypes::NotVisible, ExploreTypes::Unexplored)
    }
}

pub enum VisibleTypes {
    NotVisible,
    Visible,
}

pub enum ExploreTypes {
    Unexplored,
    Explored
}

#[derive(Component)]
pub struct POV {
    // Let the i32 represent light level
    pub visible_tiles : Vec<(Position, i32)>,
    // Represents the tiles that are revealed on the last move
    pub newly_revealed_tiles : Vec<Position>,
    pub range : i32,
}

/// Represents player or enemy.
#[derive(Component)]
pub struct Mob;

/// Represents a lit tile
#[derive(Component)]
pub struct LitTile;