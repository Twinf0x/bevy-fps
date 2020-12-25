use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, Input};
use bevy::input::mouse::MouseMotion;
use bevy::app::Events;
use crate::input::input_events::*;

pub struct InputAdapter;
impl Plugin for InputAdapter {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ReloadInputEvent>()
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

#[derive(Default)]
struct MouseReader {
    motion_reader: EventReader<MouseMotion>
}

// Bevy events need to be update once per frame
fn update_events(
    mut reload_events: ResMut<Events<ReloadInputEvent>>,
    mut interact_events: ResMut<Events<InteractInputEvent>>,
    mut shoot_events: ResMut<Events<ShootInputEvent>>,
    mut walk_events: ResMut<Events<WalkInputEvent>>,
    mut look_events: ResMut<Events<LookInputEvent>>,
) {
    reload_events.update();
    interact_events.update();
    shoot_events.update();
    walk_events.update();
    look_events.update();
}

fn update_simple_actions(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut reload_events: ResMut<Events<ReloadInputEvent>>,
    mut interact_events: ResMut<Events<InteractInputEvent>>,
    mut shoot_events: ResMut<Events<ShootInputEvent>>
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        reload_events.send(ReloadInputEvent{});
    }

    if keyboard_input.just_pressed(KeyCode::E) {
        info!("Interacting");
        interact_events.send(InteractInputEvent{});
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        info!("Shooting");
        shoot_events.send(ShootInputEvent{});
    }
}

fn update_walking(
    keyboard_input: Res<Input<KeyCode>>,
    mut walk_events: ResMut<Events<WalkInputEvent>>,
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
        walk_events.send(WalkInputEvent{direction: direction});
    }
}

fn update_looking(
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut mouse_reader: Local<MouseReader>,
    mut look_events: ResMut<Events<LookInputEvent>>
) {
    let mut direction = Vec2::zero();
    for event in mouse_reader.motion_reader.iter(&mouse_motion_events) {
        direction += event.delta;
    }

    if direction.length() != 0.0 {
        direction = direction.normalize();
        info!("Looking in direction: {}", direction.to_string());
        look_events.send(LookInputEvent{direction: direction});
    }
}