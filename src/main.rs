use bevy::prelude::*;
use bevy::window::WindowMode;

mod input;
use input::input_adapter::*;

mod game_entity;
use game_entity::player::player_controller::*;
use game_entity::weapons::weapons_plugin::*;

mod graphics;
use graphics::sprite_animator::*;
use graphics::test_sprite_animation::*;

mod ui;
use ui::hud::*;
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
        .add_plugin(SpriteAnimationPlugin)
        .add_plugin(WeaponsPlugin)
        .add_plugin(TestSpriteAnimator)
        .add_plugin(Hud)
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
            mesh: meshes.add(Mesh::from(shape::Box::new(20.0, 1.0, 20.0))),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -0.5, 0.0)),
            ..Default::default()
        })
        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 4.0, 4.0)),
            ..Default::default()
        });
}