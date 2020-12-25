use bevy::prelude::{Bundle, Transform};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub mover: Mover,
    pub destructable: Destructable,
}

pub struct Player;

pub struct PlayerInputSettings {
    pub mouse_sensitivity: f32
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