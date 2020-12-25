use bevy::prelude::Vec2;

// How to use the input events

// 1) Subscribe to events
// Create a system that takes a local event reader as a parameter i.e.:
// mut reaload_reader: Local<EventReader<RealoadInputEvent>>
// In the system iterate the events and act accordingly i.e.:
// for _ in reload_reader.iter(&reload_events) { ... }
// More examples can be found int test_input_events.rs

// 2) Create a new event
// Add it as a pub struct to this file.
// Add it to the InputAdapter's build function
// Add it to the InputAdapter's update_events system
// Add it to one of the existing update functions or create a new one
// to trigger the event when needed

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
