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
pub struct Visible;