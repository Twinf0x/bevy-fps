use bevy::prelude::*;
use bevy::window::WindowMode;

mod input;
use input::input_adapter::*;

fn main() {
    App::build()
        .add_resource(Msaa {samples: 4})
        .add_resource(WindowDescriptor {
            title: "Bevy FPS".to_string(),
            width: 854.0,
            height: 480.0,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(InputAdapter)
        .run();
}