use bevy::prelude::*;
use crate::input::input_events::*;

pub struct TestInputEvents;
impl Plugin for TestInputEvents {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(listen_for_reload.system());
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