use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_rapier3d::physics::RapierPhysicsPlugin;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::geometry::ColliderBuilder;

mod input;
use input::input_adapter::*;

mod game_entity;
use game_entity::player::player_controller::*;

mod graphics;
use graphics::sprite_animator::*;
use graphics::test_sprite_animation::*;

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
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(InputAdapter)
        .add_plugin(PlayerController)
        .add_plugin(SpriteAnimationPlugin)
        .add_plugin(TestSpriteAnimator)
        .add_startup_system(setup.system())
        .run();
}

// Just for testing. Can be removed later
fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let floor_body = RigidBodyBuilder::new_dynamic().translation(0.0, 0.0, 0.0).lock_translations().lock_rotations();
    let floor_collider = ColliderBuilder::cuboid(10.0, 1.0, 10.0);

    commands
        // Plane
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(floor_body)
        .with(floor_collider)
        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 4.0, 4.0)),
            ..Default::default()
        });
}