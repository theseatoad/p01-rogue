use bevy::prelude::*;

use crate::{components::Position, resources::GlyphAssets, tiles::TILESIZE};

#[derive(Component, Default, Debug)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    player: Player,
    position: Position,
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
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(movement);
    }
}

fn setup(mut commands: Commands, atlas: Res<GlyphAssets>) {
    commands.spawn(PlayerBundle::new((1, 1), atlas.atlas.clone()));
}

fn movement(
    mut player_query: Query<(&mut Position, &mut Transform), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for mut player in player_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            player.0.y += 1;
            player.1.translation.y += 1.0 * TILESIZE as f32;
        } else if keyboard_input.pressed(KeyCode::A) {
            player.0.x -= 1;
            player.1.translation.x -= 1.0 * TILESIZE as f32;
        } else if keyboard_input.pressed(KeyCode::S) {
            player.0.y -= 1;
            player.1.translation.y -= 1.0 * TILESIZE as f32;
        } else if keyboard_input.pressed(KeyCode::D) {
            player.0.x += 1;
            player.1.translation.x += 1.0 * TILESIZE as f32;
        }
    }
}
