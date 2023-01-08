use bevy::prelude::*;

#[derive(Resource)]
pub struct GlyphAssets {
    pub atlas: Handle<TextureAtlas>,
}