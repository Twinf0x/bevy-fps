use bevy::prelude::*;

pub struct Weapon {
    pub is_active: bool,
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
    pub bullet_texture: Handle<Texture>
}

#[derive(Bundle)]
pub struct WeaponBundle {
    pub weapon: Weapon,
    pub transform: Transform,
    pub global_transform: GlobalTransform
}

impl Weapon {
    pub fn reload(&mut self) {
        // No more bullets to load into the gun
        if self.current_ammo <= self.current_magazine {
            return;
        }

        if self.current_ammo >= self.magazine_size {
            self.current_magazine = self.magazine_size;
        }
        else {
            self.current_magazine = self.current_ammo;
        }
    }
}