use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, Input};
use bevy::input::mouse::MouseMotion;
use bevy::app::Events;
use crate::input::input_events::*;

#[derive(Default)]
struct InputState {
    reload_events: Events<ReloadInputEvent>,
    interact_events: Events<InteractInputEvent>,
    shoot_events: Events<ShootInputEvent>,
    walk_events: Events<WalkInputEvent>,
    look_events: Events<LookInputEvent>,
    mouse_motion_reader: EventReader<MouseMotion>
}

pub struct InputAdapter;
impl Plugin for InputAdapter {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_events.system())
            .add_event::<ReloadInputEvent>()
            .add_event::<InteractInputEvent>()
            .add_event::<ShootInputEvent>()
            .add_event::<WalkInputEvent>()
            .add_event::<LookInputEvent>()
            .add_system(update_events.system())
            .add_system(update_simple_actions.system())
            .add_system(update_walking.system())
            .add_system(update_looking.system());
    }
}

fn init_events(mut state: Local<InputState>) {
    state.reload_events = Events::<ReloadInputEvent>::default();
    state.interact_events = Events::<InteractInputEvent>::default();
    state.shoot_events = Events::<ShootInputEvent>::default();
    state.walk_events = Events::<WalkInputEvent>::default();
    state.look_events = Events::<LookInputEvent>::default();
}

// Bevy events need to be update once per frame
fn update_events(mut state: Local<InputState>) {
    state.reload_events.update();
    state.interact_events.update();
    state.shoot_events.update();
    state.walk_events.update();
    state.look_events.update();
}

fn update_simple_actions(
    mut state: Local<InputState>, 
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        info!("Reloading");
        state.reload_events.send(ReloadInputEvent{});
    }

    if keyboard_input.just_pressed(KeyCode::E) {
        info!("Interacting");
        state.interact_events.send(InteractInputEvent{});
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        info!("Shooting");
        state.shoot_events.send(ShootInputEvent{});
    }
}

fn update_walking(
    mut state: Local<InputState>,
    keyboard_input: Res<Input<KeyCode>>
) {
    let mut direction = Vec2::zero();

    if keyboard_input.pressed(KeyCode::W) {
        info!("Walking forward");
        direction.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        info!("Walking backwards");
        direction.y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        info!("Walking to the right");
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::A) {
        info!("Walking to the left");
        direction.x -= 1.0;
    }

    if direction.length() != 0.0 {
        direction = direction.normalize();
        info!("Walking in direction: {}", direction.to_string());
        state.walk_events.send(WalkInputEvent{direction: direction});
    }
}

fn update_looking(
    mut state: Local<InputState>,
    mouse_motion_events: Res<Events<MouseMotion>>
) {
    let mut direction = Vec2::zero();
    for event in state.mouse_motion_reader.iter(&mouse_motion_events) {
        direction += event.delta;
    }

    if direction.length() != 0.0 {
        direction = direction.normalize();
        info!("Looking in direction: {}", direction.to_string());
        state.look_events.send(LookInputEvent{direction: direction});
    }
}