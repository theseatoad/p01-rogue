use bevy::prelude::*;

#[derive(Resource)]
struct Level {
    tiles: Vec<Vec<TileType>>,
    tile_size: Vec2,
}


#[derive(PartialEq, Copy, Clone)]
enum TileType {
    WALL,
    FLOOR,
}