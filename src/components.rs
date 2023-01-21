use bevy::prelude::*;

#[derive(Component)]
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
    pub visible_tiles : Vec<Position>,
    pub range : i32,
}

/// Represents player or enemy.
#[derive(Component)]
pub struct Mob;