use bevy::{prelude::*, render::camera::ScalingMode};
mod map;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "p01_3dr".to_string(),
                        width: 800.,
                        height: 500.,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("CGA8x8thick.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 16, 16, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedHorizontal(800.),
            ..default()
        },
        ..default()
    });
    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(1.0)),
        sprite : TextureAtlasSprite {
            index: 2,
            ..default()
        },
        ..default()
    },));
}
