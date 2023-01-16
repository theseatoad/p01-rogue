use std::collections::HashMap;

use crate::{
    resources::GlyphAssets,
    tiles::{FloorBundle, TileType},
};

use map_gen_2d::{bsp::*, Point, Tile};
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Resource)]
pub struct Level {
    pub tiles: HashMap<Point, Tile>,
    pub size: (usize, usize),
}

impl Level {
    pub fn new() -> Self {
        let map = BSPMap::new(Point::new(50,30), SeedableRng::seed_from_u64(1), Point::new(5,3), Point::new(10,8)).unwrap();
        Level {
            tiles: map.get_tiles().clone(),
            size: (50, 30),
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
            Tile::Wall => {}
            Tile::Floor => {
                commands.spawn(FloorBundle::new((tile.0.x, tile.0.y), atlas.atlas.clone()));
            }
        }
    }
    commands.insert_resource(level);
}
