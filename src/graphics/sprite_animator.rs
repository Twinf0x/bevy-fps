use bevy::prelude::*;
use std::collections::*;
use crate::graphics::sprite_components::*;
use crate::game_entity::player::player_components::*;

pub struct SpriteAnimationPlugin;
impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update_animated_sprites.system())
            .add_system(update_billboard_sprites.system());
    }
}

fn update_animated_sprites(
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animated_sprites: Query<(
        &mut Timer, 
        &Handle<StandardMaterial>, 
        &mut HashMap<AnimationType, Animation>,
        &AnimationType
    )>
) {
    for (mut timer, material, mut animations, current_animation) in animated_sprites.iter_mut() {
        if current_animation == &AnimationType::None {
            continue;
        }

        if timer.tick(time.delta_seconds()).just_finished() {
            let mut animation = animations.get_mut(current_animation).unwrap();
            animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
            let texture = animation.frames[animation.current_frame].clone();
            materials.set(material.id, texture.into());
        }
    }
}

fn update_billboard_sprites(
    mut billboards: Query<&mut Transform, With<BillboardSprite>>,
    player_cameras: Query<&GlobalTransform, With<PlayerCamera>>
) {
    for camera in player_cameras.iter() {
        for mut billboard in billboards.iter_mut() {
            let x = billboard.translation.x;
            let y = billboard.translation.y;
            let z = billboard.translation.z;
            let delta = (camera.translation - billboard.translation) * -1.0;
            billboard.look_at(Vec3{
                x: x + delta.x,
                y: y,
                z: z + delta.z
            }, 
            Vec3{x: 0.0, y: 1.0, z: 0.0});
        }
    }
}