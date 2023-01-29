use bevy::prelude::*;

use crate::{components::{Position, POV, Mob, MobType}, tiles::TILESIZE, resources::GlyphAssets, map::Level, player::Player, health::Health};

#[derive(Component, Default, Debug)]
pub struct Enemy;

#[derive(Component, Default, Debug)]
pub struct Orc;

#[derive(Bundle)]
pub struct OrcBundle {
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    orc: Orc,
    enemy : Enemy,
    position: Position,
    pov: POV,
    mob: Mob,
    health : Health
}

impl OrcBundle {
    pub fn new(location: (i32, i32), texture_atlas_handle: Handle<TextureAtlas>) -> OrcBundle {
        OrcBundle {
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
                    index: 111,
                    ..default()
                },
                ..default()
            },
            orc: Orc,
            enemy : Enemy,
            position: Position {
                x: location.0,
                y: location.1,
            },
            pov: POV {
                visible_tiles: Vec::new(),
                newly_revealed_tiles: Vec::new(),
                range: 8,
            },
            mob: Mob(MobType::ORC),
            health : Health(4),
        }
    }
}

#[derive(Component, Default, Debug)]
pub struct Goblin;

#[derive(Bundle)]
pub struct GoblinBundle {
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    goblin: Goblin,
    enemy : Enemy,
    position: Position,
    pov: POV,
    mob: Mob,
    health : Health
}

impl GoblinBundle {
    pub fn new(location: (i32, i32), texture_atlas_handle: Handle<TextureAtlas>) -> GoblinBundle {
        GoblinBundle {
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
                    index: 103,
                    ..default()
                },
                ..default()
            },
            goblin: Goblin,
            enemy : Enemy,
            position: Position {
                x: location.0,
                y: location.1,
            },
            pov: POV {
                visible_tiles: Vec::new(),
                newly_revealed_tiles: Vec::new(),
                range: 8,
            },
            mob: Mob(MobType::GOBLIN),
            health : Health(3),
        }
    }
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, atlas: Res<GlyphAssets>, map: Res<Level>) {
    for mob in map.mobs.iter() {
        if mob.1 == MobType::ORC {
            commands.spawn(OrcBundle::new((mob.0.x.try_into().unwrap(), mob.0.y.try_into().unwrap()), atlas.atlas.clone()));
        } else if mob.1 == MobType::GOBLIN {
            commands.spawn(GoblinBundle::new((mob.0.x.try_into().unwrap(), mob.0.y.try_into().unwrap()), atlas.atlas.clone()));
        }
    };
}

fn movement(
    mut commands: Commands,
    enemy_query: Query<(&mut Position, &POV, With<Enemy>)>,
    player_query: Query<&Position, (Changed<Position>, With<Player>)>,
    map: Res<Level>,
) {

}