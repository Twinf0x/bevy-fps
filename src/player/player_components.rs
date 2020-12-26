use bevy::prelude::{Bundle, Transform, GlobalTransform};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub transform: Transform,
    pub mover: Mover,
    pub destructable: Destructable,
    pub global_transform: GlobalTransform
}

pub struct Player;
pub struct PlayerCamera {
    pub max_pitch: f32,
    pub min_pitch: f32,
    pub current_pitch: f32
}

// These are probably relevant for enemies, too.
// Maybe we should move them elsewhere?
pub struct Mover {
    pub speed: f32
}

pub struct Destructable {
    pub max_health: f32,
    pub current_health: f32
}