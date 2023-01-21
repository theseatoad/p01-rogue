// Responsible for keeping camera on player
// Updating lighting
// Despawning tile under enemies and players
use crate::{
    components::{MainCamera, Position, Mob, POV},
    player::Player, tiles::{TILESIZE, Tile},
};
use bevy::prelude::*;

pub const WINDOWSIZE: (f32, f32) = (800.0, 500.0);
pub struct RenderingPlugin;
impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(update_camera_position)
                .with_system(update_lighting)
                .with_system(update_tile_vis),
        );
    }
}

// Despawn tiles under mobs
fn update_tile_vis(
    mob_query: Query<(&Position, &mut POV), (Changed<Position>, With<Mob>)>,
    mut tile_query: Query<(&Position, &mut Visibility, &Tile), Without<Mob>>,
) {

    for (position, mut pov) in mob_query.iter() {
        
    }
}

fn update_lighting() {}

fn update_camera_position(
    player_query: Query<&Position, (Changed<Position>, With<Player>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    for player_pos in player_query.iter() {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation = Vec3::new(player_pos.x as f32 * TILESIZE as f32, player_pos.y as f32 * TILESIZE as f32, 999.0);
    }
}
