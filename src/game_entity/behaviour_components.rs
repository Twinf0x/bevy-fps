use bevy::prelude::*;

pub struct Mover {
    pub speed: f32
}

pub struct Destructable {
    pub max_health: f32,
    pub current_health: f32
}

pub struct Bullet {
    pub velocity: Vec3,
    pub hit_range: f32,
    pub damage_on_hit: f32,
}

pub struct Obstacle {
    pub bottom_left: Vec2,
    pub top_right: Vec2,
}