use bevy::prelude::*;

pub struct Weapon {
    pub name: String,
    pub max_ammo: i32,
    pub current_ammo: i32,
    pub magazine_size: i32,
    pub current_magazine: i32,
    pub reload_timer: Timer,
    pub fire_timer: Timer,
    pub bullet_hit_range: f32,
    pub bullet_damage_on_hit: f32,
    pub bullet_speed: f32,
}