use bevy::prelude::*;
use bevy::app::Events;

// Compound events
pub struct WalkInputEvent {
    pub direction: Vec2
}
pub struct LookInputEvent {
    pub direction: Vec2
}

// Simple action events
pub struct ReloadInputEvent {}
pub struct InteractInputEvent {}
pub struct ShootInputEvent {}
