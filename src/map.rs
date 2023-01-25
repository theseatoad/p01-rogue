use std::collections::{HashMap, HashSet};

use crate::{
    components::MobType,
    tiles::{TileType, TileTypeMap},
};

use bevy::prelude::*;
use map_gen_2d::{bsp::*, Point};
use rand::prelude::*;

#[derive(Resource)]
pub struct Level {
    pub tiles: HashMap<Point, TileTypeMap>,
    pub size: (usize, usize),
    pub revealed_tiles: HashSet<Point>,
    pub rng: StdRng,
    pub mobs: Vec<(Point,MobType)>,
}

impl Level {
    pub fn new() -> Self {
        let rng = SeedableRng::seed_from_u64(5);
        let map = BSPMap::new(
            Point::new(50, 50),
            SeedableRng::seed_from_u64(5),
            Point::new(3, 5),
            Point::new(10, 15),
        )
        .unwrap();
        let mut tiles: HashMap<Point, TileTypeMap> = HashMap::new();
        // Convert map_gen_2d tiles to p01_3dr tiles
        for tile in map.get_tiles() {
            match tile.1 {
                map_gen_2d::Tile::Floor => {
                    tiles.insert(tile.0.clone(), TileTypeMap(TileType::FLOOR));
                }
                map_gen_2d::Tile::Wall => {
                    tiles.insert(tile.0.clone(), TileTypeMap(TileType::WALL));
                }
            }
        }

        Level {
            tiles: tiles,
            size: (50, 50),
            revealed_tiles: HashSet::new(),
            rng,
            mobs: Vec::new(),
        }
    }
}

// ============================
// ========== PLUGIN ==========
pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup.at_start());
    }
}

fn setup(mut commands: Commands) {
    let mut level = Level::new();
    // Randomly select one point for player
    place_at_random_valid_point(&mut level, MobType::PLAYER);
    // Randomly select 4 spots for goblins
    for _ in 0..4 {
        place_at_random_valid_point(&mut level, MobType::GOBLIN);
    }

    // Randomly select 2 spots for orcs
    for _ in 0..2 {
        place_at_random_valid_point(&mut level, MobType::ORC);
    }
    commands.insert_resource(level);
}

fn place_at_random_valid_point(level: &mut Level, mob: MobType) {
    let mut try_valid = false;
    let mut x: usize = 0;
    let mut y: usize = 0;
    while !try_valid {
        x = level.rng.gen_range(1..level.size.0);
        y = level.rng.gen_range(1..level.size.1);
        match level.tiles.get(&Point { x, y }) {
            Some(tile) => {
                if tile.0 == TileType::FLOOR {
                    try_valid = true;
                }
            }
            _ => {}
        }
    }
    level.mobs.push((Point{x,y}, mob.clone()));
}
