// Responsible for keeping camera on player
// Updating lighting
// Despawning tile under enemies and players
use crate::{
    components::{LitTile, MainCamera, Mob, Position, POV},
    map::Level,
    player::Player,
    resources::GlyphAssets,
    tiles::{FloorBundle, Tile, TileType, TileTypeMap, WallBundle, TILESIZE},
};
use bevy::prelude::*;
use map_gen_2d::Point;

pub const WINDOWSIZE: (f32, f32) = (800.0, 500.0);
pub struct RenderingPlugin;
impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(update_camera_position)
                .with_system(update_tile_vis_and_explore)
                .with_system(update_tiles),
        );
    }
}

// Despawn tiles under mobs
fn update_tile_vis_and_explore(
    mut mob_query: Query<(&Position, &mut POV), (Changed<Position>, With<Mob>)>,
    mut map: ResMut<Level>,
) {
    // Need to calculate what the mob can see.
    for (position, mut pov) in mob_query.iter_mut() {
        pov.visible_tiles.clear();
        pov.newly_revealed_tiles.clear();
        // Calculate visible tiles near position.
        // loop over an range x range loop centered around player.
        for x in (position.x - pov.range)..(position.x + pov.range) {
            for y in (position.y - pov.range)..(position.y + pov.range) {
                if let Some(l) =
                    bresenham_line_enhanced(&(position.x, position.y), &(x, y), &pov.range, &map)
                {
                    pov.visible_tiles.push((Position { x, y }, l));
                    if map.revealed_tiles.insert(Point {
                        x: x as usize,
                        y: y as usize,
                    }) == true
                    {
                        pov.newly_revealed_tiles.push(Position { x, y })
                    }
                }
            }
        }
        pov.visible_tiles.retain(|p| {
            p.0.x >= 0
                && p.0.x < map.size.0.try_into().unwrap()
                && p.0.y >= 0
                && p.0.y < map.size.1.try_into().unwrap()
        });
    }
}

fn update_tiles(
    mut commands: Commands,
    atlas: Res<GlyphAssets>,
    tile_query: Query<(Entity, With<LitTile>)>,
    player_query: Query<(&Position, &POV), (Changed<Position>, With<Player>)>,
    map: Res<Level>,
) {
    for player in player_query.iter() {
        for entity in tile_query.iter() {
            // Despawn all lit-tiles
            commands.entity(entity.0).despawn();
        }

        // Spawn lit tiles
        for tile in player.1.visible_tiles.iter() {
            match map.tiles.get(&Point {
                x: tile.0.x as usize,
                y: tile.0.y as usize,
            }) {
                Some(TileTypeMap(TileType::WALL)) => {
                    commands
                        .spawn(WallBundle::new(
                            (tile.0.x.try_into().unwrap(), tile.0.y.try_into().unwrap()),
                            atlas.atlas.clone(),
                            Color::WHITE
                        ))
                        .insert(LitTile);
                }
                Some(TileTypeMap(TileType::FLOOR)) => {
                    commands
                        .spawn(FloorBundle::new(
                            (tile.0.x.try_into().unwrap(), tile.0.y.try_into().unwrap()),
                            atlas.atlas.clone(),
                            Color::WHITE
                        ))
                        .insert(LitTile);
                }
                None => {
                    //nothing
                }
            }
        }

        // Spawn newly revealed tiles
        for tile in player.1.newly_revealed_tiles.iter() {
            match map.tiles.get(&Point {
                x: tile.x as usize,
                y: tile.y as usize,
            }) {
                Some(TileTypeMap(TileType::WALL)) => {
                    commands
                        .spawn(WallBundle::new(
                            (tile.x.try_into().unwrap(), tile.y.try_into().unwrap()),
                            atlas.atlas.clone(),
                             Color::GRAY
                        ));
                }
                Some(TileTypeMap(TileType::FLOOR)) => {
                    commands
                        .spawn(FloorBundle::new(
                            (tile.x.try_into().unwrap(), tile.y.try_into().unwrap()),
                            atlas.atlas.clone(),
                            Color::GRAY
                        ));
                }
                None => {
                    //nothing
                }
            }
        }
    }
}

// Returns none if line is out of range / not visible
// Returns some(i32) where i32 is light level
// https://sites.google.com/site/jicenospam/visibilitydetermination
fn bresenham_line_enhanced(
    position: &(i32, i32),
    tile_position: &(i32, i32),
    range: &i32,
    map: &Level,
) -> Option<i32> {
    let distance = (position.0 - tile_position.0).abs() + (position.1 - tile_position.1).abs();
    if range > &distance {
        return Some(1);
    }
    None
}

fn update_camera_position(
    player_query: Query<&Position, (Changed<Position>, With<Player>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    for player_pos in player_query.iter() {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation = Vec3::new(
            player_pos.x as f32 * TILESIZE as f32,
            player_pos.y as f32 * TILESIZE as f32,
            999.0,
        );
    }
}
