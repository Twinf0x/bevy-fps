use bevy::core::Timer;
use bevy::prelude::*;
use bevy::utils::Duration;
use std::collections::*;

#[derive(Bundle)]
pub struct AnimatedSpriteBundle {
    pub animations: HashMap<AnimationType, Animation>,
    pub current_animation: AnimationType,
    pub frame_timer: Timer,
}

impl Default for AnimatedSpriteBundle {
    fn default() -> Self {
        AnimatedSpriteBundle {
            animations: HashMap::new(),
            current_animation: AnimationType::None,
            frame_timer: Timer::new(Duration::from_millis(200), true),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum AnimationType {
    None,
    Idle,
    Walk,
    Attack,
    Death,
    Custom01,
    Custom02,
    Custom03,
}

#[derive(Clone)]
pub struct Animation {
    pub frames: Vec<Handle<Texture>>,
    pub current_frame: usize,
}