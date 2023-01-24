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
                if let Some(light) =
                    bresenham_line_enhanced(&(position.x, position.y), &(x, y), &map)
                {
                    pov.visible_tiles.push((Position { x, y }, light));
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
            let color : Color;
            // HARD CODE LIGHT LEVEL TO COLOR
            if tile.1 == 0 {
                color = Color::rgb(1.0, 1.0, 0.0);
            } else if tile.1 == 1 {
                color = Color::rgb(1.0 - (0.1  * 1.0), 1.0 - (0.1  * 1.0), 0.0 + (0.05 * 1.0));
            }else if tile.1 == 2 {
                color = Color::rgb(1.0 - (0.1  * 1.0), 1.0 - (0.1  * 1.0), 0.0 + (0.05 * 1.0));
            }else if tile.1 == 3 {
                color = Color::rgb(1.0 - (0.1  * 3.0), 1.0 - (0.1  * 3.0), 0.0 + (0.05 * 3.0));
            }else if tile.1 == 4 {
                color = Color::rgb(1.0 - (0.1  * 3.0), 1.0 - (0.1  * 3.0), 0.0 + (0.05 * 3.0));
            }else if tile.1 == 5 {
                color = Color::rgb(1.0 - (0.1  * 5.0), 1.0 - (0.1  * 5.0), 0.0 + (0.05 * 5.0));
            } else if tile.1 == 6 {
                color = Color::rgb(1.0 - (0.1  * 5.0), 1.0 - (0.1  * 5.0), 0.0 + (0.05 * 5.0));
            } else {
                color = Color::rgb(1.0 - (0.1  * 7.0), 1.0 - (0.1  * 7.0), 0.0 + (0.05 * 7.0));
            }
            match map.tiles.get(&Point {
                x: tile.0.x as usize,
                y: tile.0.y as usize,
            }) {
                Some(TileTypeMap(TileType::WALL)) => {
                    commands
                        .spawn(WallBundle::new(
                            (tile.0.x.try_into().unwrap(), tile.0.y.try_into().unwrap()),
                            atlas.atlas.clone(),
                            color,
                        ))
                        .insert(LitTile);
                }
                Some(TileTypeMap(TileType::FLOOR)) => {
                    commands
                        .spawn(FloorBundle::new(
                            (tile.0.x.try_into().unwrap(), tile.0.y.try_into().unwrap()),
                            atlas.atlas.clone(),
                            color
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
                    commands.spawn(WallBundle::new(
                        (tile.x.try_into().unwrap(), tile.y.try_into().unwrap()),
                        atlas.atlas.clone(),
                        Color::GRAY,
                    ));
                }
                Some(TileTypeMap(TileType::FLOOR)) => {
                    commands.spawn(FloorBundle::new(
                        (tile.x.try_into().unwrap(), tile.y.try_into().unwrap()),
                        atlas.atlas.clone(),
                        Color::GRAY,
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
// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm#Algorithm_for_integer_arithmetic
fn bresenham_line_enhanced(
    position: &(i32, i32),
    tile_position: &(i32, i32),
    map: &Level,
) -> Option<i32> {
    // position = (x0,y0) and tile_position = (x1,y1)

    if (tile_position.1 - position.1).abs() < (tile_position.0 - position.0) {
        if position.0 > tile_position.0 {
            return bresenham_line_enhanced_low(tile_position, position, map);
        } else {
            return bresenham_line_enhanced_low(position, tile_position, map);
        }
    } else {
        if position.1 > tile_position.1 {
            return bresenham_line_enhanced_high(tile_position, position, map);
        } else {
            return bresenham_line_enhanced_high(position, tile_position, map);
        }
    }
}

fn bresenham_line_enhanced_high(
    position: &(i32, i32),
    tile_position: &(i32, i32),
    map: &Level,
) -> Option<i32> {
    // Make sure all positions are positive
    if position.0 < 0 || position.1 < 0 || tile_position.0 < 0 || tile_position.1 < 0 {
        return None;
    }
    // If we were passed a wall, obivously we can see the wall so pass Some(distance)
    if let Some(tile) = map.tiles.get(&Point {
        x: position.0.try_into().unwrap(),
        y: position.1.try_into().unwrap(),
    }) {
        if tile.0 == TileType::WALL {
            let distance =
                (position.0 - tile_position.0).abs() + (position.1 - tile_position.1).abs();
            return Some(distance);
        }
    }
    // If we were passed a wall, obivously we can see the wall so pass Some(distance)
    if let Some(tile) = map.tiles.get(&Point {
        x: tile_position.0.try_into().unwrap(),
        y: tile_position.1.try_into().unwrap(),
    }) {
        if tile.0 == TileType::WALL {
            let distance =
                (position.0 - tile_position.0).abs() + (position.1 - tile_position.1).abs();
            return Some(distance);
        }
    }
    // Algorithm
    let mut dx: i32 = tile_position.0 - position.0;
    let dy: i32 = tile_position.1 - position.1;
    let mut xi: i32 = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx
    }
    let mut d: i32 = (2 * dx) - dy;
    let mut x: i32 = position.0;
    for y in position.1..=tile_position.1 {
        // If this is a wall, we can not see the next tile.
        if let Some(tile) = map.tiles.get(&Point {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        }) {
            if tile.0 == TileType::WALL {
                return None;
            }
        }
        // Progress algorithm
        if d > 0 {
            x = x + xi;
            d = d + (2 * (dx - dy))
        } else {
            d = d + 2 * dx;
        }
    }
    // If we have gotten here, we have succesfully reached the tile.
    let distance = (position.0 - tile_position.0).abs() + (position.1 - tile_position.1).abs();
    Some(distance)
}

fn bresenham_line_enhanced_low(
    position: &(i32, i32),
    tile_position: &(i32, i32),
    map: &Level,
) -> Option<i32> {
    // Make sure all positions are postive
    if position.0 < 0 || position.1 < 0 || tile_position.0 < 0 || tile_position.1 < 0 {
        return None;
    }
    // If we were passed a wall, obivously we can see the wall so pass Some(distance)
    if let Some(tile) = map.tiles.get(&Point {
        x: position.0.try_into().unwrap(),
        y: position.1.try_into().unwrap(),
    }) {
        if tile.0 == TileType::WALL {
            let distance =
                (position.0 - tile_position.0).abs() + (position.1 - tile_position.1).abs();
            return Some(distance);
        }
    }
    // If we were passed a wall, obivously we can see the wall so pass Some(distance)
    if let Some(tile) = map.tiles.get(&Point {
        x: tile_position.0.try_into().unwrap(),
        y: tile_position.1.try_into().unwrap(),
    }) {
        if tile.0 == TileType::WALL {
            let distance =
                (position.0 - tile_position.0).abs() + (position.1 - tile_position.1).abs();
            return Some(distance);
        }
    }
    let dx: i32 = tile_position.0 - position.0;
    let mut dy: i32 = tile_position.1 - position.1;
    let mut yi: i32 = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy
    }
    let mut d: i32 = (2 * dy) - dx;
    let mut y: i32 = position.1;
    for x in position.0..=tile_position.0 {
        // If this is a wall, we can not see the next tile.
        if let Some(tile) = map.tiles.get(&Point {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        }) {
            if tile.0 == TileType::WALL {
                return None;
            }
        }
        // Progress algorithm
        if d > 0 {
            y = y + yi;
            d = d + (2 * (dy - dx))
        } else {
            d = d + 2 * dy;
        }
    }
    // If we have gotten here, we have succesfully reached the tile.
    let distance = (position.0 - tile_position.0).abs() + (position.1 - tile_position.1).abs();
    Some(distance)
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
