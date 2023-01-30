use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use components::MainCamera;
use enemies::EnemyPlugin;
use health::HealthPlugin;
use map::MapPlugin;
use player::PlayerPlugin;
use rendering::{RenderingPlugin, WINDOWSIZE};
use resources::GlyphAssets;
mod components;
mod enemies;
mod health;
mod map;
mod player;
mod rendering;
mod resources;
mod tiles;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "p01_3dr".to_string(),
                        width: WINDOWSIZE.0,
                        height: WINDOWSIZE.1,
                        ..default()
                    },
                    ..default()
                })
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.at_start())
        .add_plugin(MapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(RenderingPlugin)
        .add_plugin(HealthPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load texture
    let texture_handle = asset_server.load("CGA8x8thick.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 16, 16, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Insert glyph resource
    commands.insert_resource(GlyphAssets {
        atlas: texture_atlas_handle.clone(),
    });

    // Spawn camera
    commands
        .spawn(Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::WindowSize,
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: 200.0,
                    y: 000.0,
                    z: 0.0,
                },
                ..default()
            },
            ..default()
        })
        .insert(MainCamera);
}
