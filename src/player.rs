use bevy::prelude::*;
use map_gen_2d::Point;

use crate::{
    components::{Mob, MobType, Position, POV},
    health::Health,
    map::Level,
    resources::GlyphAssets,
    tiles::{TileType, TileTypeMap, TILESIZE},
};

#[derive(Component, Default, Debug)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    player: Player,
    position: Position,
    pov: POV,
    mob: Mob,
}

impl PlayerBundle {
    pub fn new(location: (i32, i32), texture_atlas_handle: Handle<TextureAtlas>) -> PlayerBundle {
        PlayerBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3 {
                        x: location.0 as f32 * TILESIZE as f32,
                        y: location.1 as f32 * TILESIZE as f32,
                        z: 0.0,
                    },
                    scale: Vec3 {
                        x: TILESIZE as f32 / 8.0,
                        y: TILESIZE as f32 / 8.0,
                        z: TILESIZE as f32 / 8.0,
                    },
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    index: 64,
                    ..default()
                },
                ..default()
            },
            player: Player,
            position: Position {
                x: location.0,
                y: location.1,
            },
            pov: POV {
                visible_tiles: Vec::new(),
                newly_revealed_tiles: Vec::new(),
                range: 8,
            },
            mob: Mob(MobType::PLAYER),
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(movement);
    }
}

fn setup(mut commands: Commands, atlas: Res<GlyphAssets>, map: Res<Level>) {
    for mob in map.mobs.iter() {
        if mob.1 == MobType::PLAYER {
            commands.spawn(PlayerBundle::new(
                (mob.0.x.try_into().unwrap(), mob.0.y.try_into().unwrap()),
                atlas.atlas.clone(),
            ));
        }
    }
}

fn movement(
    mut player_query: Query<(&mut Position, &mut Transform), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    map: Res<Level>,
    mut mob_query: Query<(&Position, &Mob, &mut Health), Without<Player>>,
) {
    for mut player in player_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::W) {
            match map
                .tiles
                .get(&Point::new(player.0.x as usize, player.0.y as usize + 1))
            {
                Some(TileTypeMap(TileType::WALL)) => { //nothing
                }
                _ => {
                    let mut is_attack = false;
                    for mut mob in mob_query.iter_mut() {
                        if mob.0.x == player.0.x && mob.0.y == player.0.y + 1 {
                            // Attack mob
                            is_attack = true;
                            mob.2.0 = mob.2.0 - 1;
                            break;
                        }
                    }
                    if !is_attack {
                        player.0.y += 1;
                        player.1.translation.y += 1.0 * TILESIZE as f32;
                    }
                }
            }
        } else if keyboard_input.just_pressed(KeyCode::A) {
            match map
                .tiles
                .get(&Point::new(player.0.x as usize - 1, player.0.y as usize))
            {
                Some(TileTypeMap(TileType::WALL)) => { //nothing
                }
                _ => {
                    let mut is_attack = false;
                    for mut mob in mob_query.iter_mut() {
                        if mob.0.x == player.0.x - 1 && mob.0.y == player.0.y {
                            // Attack mob
                            is_attack = true;
                            mob.2.0 = mob.2.0 - 1;
                            break;
                        }
                    }
                    if !is_attack {
                        player.0.x -= 1;
                        player.1.translation.x -= 1.0 * TILESIZE as f32;
                    }
                }
            }
        } else if keyboard_input.just_pressed(KeyCode::S) {
            match map
                .tiles
                .get(&Point::new(player.0.x as usize, player.0.y as usize - 1))
            {
                Some(TileTypeMap(TileType::WALL)) => { //nothing
                }
                _ => {
                    let mut is_attack = false;
                    for mut mob in mob_query.iter_mut() {
                        if mob.0.x == player.0.x && mob.0.y == player.0.y - 1 {
                            // Attack mob
                            is_attack = true;
                            mob.2.0 = mob.2.0 - 1;
                            break;
                        }
                    }
                    if !is_attack {
                        player.0.y -= 1;
                        player.1.translation.y -= 1.0 * TILESIZE as f32;
                    }
                }
            }
        } else if keyboard_input.just_pressed(KeyCode::D) {
            match map
                .tiles
                .get(&Point::new(player.0.x as usize + 1, player.0.y as usize))
            {
                Some(TileTypeMap(TileType::WALL)) => { //nothing
                }
                _ => {
                    let mut is_attack = false;
                    for mut mob in mob_query.iter_mut() {
                        if mob.0.x == player.0.x + 1 && mob.0.y == player.0.y {
                            // Attack mob
                            mob.2.0 = mob.2.0 - 1;
                            is_attack = true;
                            break;
                        }
                    }
                    if !is_attack {
                        player.0.x += 1;
                        player.1.translation.x += 1.0 * TILESIZE as f32;
                    }
                }
            }
        }
    }
}
