use bevy::prelude::{Bundle, Transform, GlobalTransform};
use crate::game_entity::behaviour_components::*;

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