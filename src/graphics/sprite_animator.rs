use bevy::prelude::*;
use std::collections::*;
use crate::graphics::animation_components::*;

pub struct SpriteAnimationPlugin;
impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update_animated_sprites.system());
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