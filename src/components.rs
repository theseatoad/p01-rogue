use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    // Let this represent the character on the spritesheet.
    glyph: (usize, usize),
    fg: Color,
    bg: Option<Color>,
}