use bevy::prelude::*;

use crate::{
    components::Mob,
};

#[derive(Component)]
pub struct Health(pub i32);

pub struct HealthPlugin;
impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_health_to_despawn);
    }
}

fn check_health_to_despawn(
    mut commands: Commands,
    health_query: Query<(Entity, &Health, &Mob)>,
) {
    for mob in health_query.iter() {
        if mob.1.0 <= 0 {
            commands.entity(mob.0).despawn();
        }
    }
}
