use bevy::{prelude::*, utils::HashMap};

use crate::{
    resources::GlyphAssets,
    tiles::{FloorBundle, TileType, WallBundle},
};

#[derive(Resource)]
pub struct Level {
    pub tiles: HashMap<(i32, i32), TileType>,
    pub size: (usize, usize),
}

impl Level {
    pub fn new() -> Self {
        Level {
            tiles: HashMap::new(),
            size: (20, 20),
        }
    }
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, atlas: Res<GlyphAssets>) {
    let mut level = Level::new();
    for x in 0..level.size.0 {
        for y in 0..level.size.1 {
            if x == 0 || x == level.size.0 - 1 {
                level.tiles.insert((x as i32,y as i32), TileType::WALL);
                commands.spawn(WallBundle::new((x as i32, y as i32), atlas.atlas.clone()));
            } else if y == 0 || y == level.size.1 - 1 {
                level.tiles.insert((x as i32,y as i32), TileType::WALL);
                commands.spawn(WallBundle::new((x as i32, y as i32), atlas.atlas.clone()));
            } else {
                level.tiles.insert((x as i32,y as i32), TileType::FLOOR);
                commands.spawn(FloorBundle::new((x as i32, y as i32), atlas.atlas.clone()));
            }
        }
    }
    commands.insert_resource(level);
}
