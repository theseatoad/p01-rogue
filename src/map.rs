use std::collections::HashMap;

use crate::{
    resources::GlyphAssets,
    tiles::{FloorBundle, TileType, WallBundle},
};

use bevy::prelude::*;
use map_gen_2d::{bsp::*, Point, Tile};
use rand::prelude::*;

#[derive(Resource)]
pub struct Level {
    pub tiles: HashMap<Point, Tile>,
    pub size: (usize, usize),
}

impl Level {
    pub fn new() -> Self {
        let map = BSPMap::new(
            Point::new(50, 50),
            SeedableRng::seed_from_u64(5),
            Point::new(3, 5),
            Point::new(10, 15),
        )
        .unwrap();
        Level {
            tiles: map.get_tiles().clone(),
            size: (50, 50),
        }
    }
}

// ============================
// ========== PLUGIN ==========
pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, atlas: Res<GlyphAssets>) {
    let level = Level::new();
    for tile in level.tiles.iter() {
        match tile.1 {
            Tile::Floor => {
                if tile.0.x <= level.size.0 && tile.0.y <= level.size.1 {
                    commands.spawn(FloorBundle::new((tile.0.x, tile.0.y), atlas.atlas.clone()));
                }
            }
            Tile::Wall => {
                if tile.0.x <= level.size.0 && tile.0.y <= level.size.1 {
                    commands.spawn(WallBundle::new((tile.0.x, tile.0.y), atlas.atlas.clone()));
                }
            }
        }
    }
    commands.insert_resource(level);
}
