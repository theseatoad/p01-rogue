use bevy::prelude::*;

use crate::components::{Collision};
pub const TILESIZE : usize = 12;
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum TileType {
    WALL,
    FLOOR,
}

pub struct TileTypeMap(pub TileType);

#[derive(Component)]
pub struct Tile(pub TileType);

#[derive(Bundle)]
pub struct WallBundle {
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    collision: Collision,
    tile: Tile,
}

impl WallBundle {
    pub fn new(location: (usize, usize), texture_atlas_handle: Handle<TextureAtlas>, color : Color) -> WallBundle {
        WallBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 35,
                    color : color,
                    ..default()
                },
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3 {
                        x: location.0 as f32 * TILESIZE as f32,
                        y: location.1 as f32 * TILESIZE as f32,
                        z: 0.0,
                    },
                    scale : Vec3 {
                        x : TILESIZE as f32 / 8.0,
                        y : TILESIZE as f32 / 8.0,
                        z : TILESIZE as f32 / 8.0,
                    },
                    ..default()
                },
                ..default()
            },
            collision: Collision,
            tile: Tile(TileType::WALL),
        }
    }
}


#[derive(Bundle)]
pub struct FloorBundle {
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    tile: Tile,
}

impl FloorBundle {
    pub fn new(location: (usize, usize), texture_atlas_handle: Handle<TextureAtlas>, color : Color) -> FloorBundle {
        FloorBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 250,
                    color : color,
                    ..default()
                },
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3 {
                        x: location.0 as f32 * TILESIZE as f32,
                        y: location.1 as f32 * TILESIZE as f32,
                        z: 0.0,
                    },
                    scale : Vec3 {
                        x : TILESIZE as f32 / 8.0,
                        y : TILESIZE as f32 / 8.0,
                        z : TILESIZE as f32 / 8.0,
                    },
                    ..default()
                },
                ..default()
            },
            tile: Tile(TileType::WALL)
        }
    }
}
