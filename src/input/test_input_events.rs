use bevy::prelude::*;
use crate::input::input_events::*;

pub struct TestInputEvents;
impl Plugin for TestInputEvents {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(listen_for_reload.system())
            .add_system(listen_for_interaction.system())
            .add_system(listen_for_shoot.system())
            .add_system(listen_for_walk.system())
            .add_system(listen_for_look.system());
    }
}

fn listen_for_reload (
    mut reload_reader: Local<EventReader<ReloadInputEvent>>,
    reload_events: Res<Events<ReloadInputEvent>>
) {
    for _ in reload_reader.iter(&reload_events) {
        info!("Reloading");
    }
}

fn listen_for_interaction (
    mut interact_reader: Local<EventReader<InteractInputEvent>>,
    interact_events: Res<Events<InteractInputEvent>>
) {
    for _ in interact_reader.iter(&interact_events) {
        info!("Interact");
    }
}

fn listen_for_shoot (
    mut shoot_reader: Local<EventReader<ShootInputEvent>>,
    shoot_events: Res<Events<ShootInputEvent>>
) {
    for _ in shoot_reader.iter(&shoot_events) {
        info!("Shooting")
    }
}

fn listen_for_walk (
    mut walk_reader: Local<EventReader<WalkInputEvent>>,
    walk_events: Res<Events<WalkInputEvent>>
) {
    for event in walk_reader.iter(&walk_events) {
        info!("Walking in direction: {}", event.direction.to_string());
    }
}

fn listen_for_look (
    mut look_reader: Local<EventReader<LookInputEvent>>,
    look_events: Res<Events<LookInputEvent>>
) {
    for event in look_reader.iter(&look_events) {
        info!("Looking in direction: {}", event.direction.to_string());
    }
}