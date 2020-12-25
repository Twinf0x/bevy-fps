use bevy::prelude::*;
use bevy::window::WindowMode;

mod input;
use input::input_adapter::*;

mod player;
use player::character_controller::*;

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
        .add_resource(ClearColor { 0: Color::rgb(0.1, 0.1, 0.1)})
        .add_plugins(DefaultPlugins)
        .add_plugin(InputAdapter)
        .add_plugin(PlayerController)
        .add_startup_system(setup.system())
        .run();
}

// Just for testing. Can be removed later
fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands
        // Plane
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 4.0, 4.0)),
            ..Default::default()
        });
}