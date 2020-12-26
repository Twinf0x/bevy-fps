use bevy::prelude::*;
use bevy::app::Events;
use bevy::math::*;
use crate::input::input_events::*;
use crate::game_entity::player::player_components::*;
use crate::game_entity::behaviour_components::*;

const DEG2RAD: f32 = std::f32::consts::PI / 180.0;

pub struct PlayerController;
impl Plugin for PlayerController {
    fn build(&self, app: &mut AppBuilder){
        app.add_startup_system(setup_player_character.system())
            .add_system(update_walking.system())
            .add_system(update_first_person_camera.system());
    }
}

fn setup_player_character(
    commands: &mut Commands
) {
    let player = commands.spawn(PlayerBundle{
        player: Player{},
        transform: Transform{
            translation: Vec3::zero(),
            rotation: Quat::from_axis_angle(Vec3::unit_y(), 0.0),
            scale: Vec3::one()
        },
        mover: Mover{
            speed: 5.0
        },
        destructable: Destructable{
            max_health: 100.0,
            current_health: 100.0,
        },
        global_transform: GlobalTransform::default()
    })
    .current_entity()
    .unwrap();

    commands.spawn(Camera3dBundle{
        transform: Transform {
            translation: Vec3::unit_y() * 2.0,
            rotation: Quat::from_axis_angle(Vec3::unit_y(), 0.0),
            scale: Vec3::one()
        },
        ..Default::default()
    })
    .with(PlayerCamera{
        max_pitch: 60.0,
        min_pitch: -60.0,
        current_pitch: 0.0
    })
    .with(Parent(player));
}

fn update_walking(
    mut walk_reader: Local<EventReader<WalkInputEvent>>,
    walk_events: Res<Events<WalkInputEvent>>,
    mut players: Query<(&Mover, &mut Transform), With<Player>>,
    time: Res<Time>
) {
    for movement in walk_reader.iter(&walk_events){
        for (mover, mut transform) in players.iter_mut() {
            let forward = Vec3 {x: transform.forward().x, y: 0.0, z: transform.forward().z}.normalize();
            let forward_movement = forward * movement.direction.y * mover.speed * time.delta_seconds() * -1.0;
            let right = transform.forward().cross(Vec3::unit_y()).normalize();
            let side_movement = right * movement.direction.x * mover.speed * time.delta_seconds() * -1.0;
            transform.translation += forward_movement + side_movement;
        }
    }
}

fn update_first_person_camera(
    mut look_reader: Local<EventReader<LookInputEvent>>,
    look_events: Res<Events<LookInputEvent>>,
    mut player_cameras: Query<(&mut Transform, &mut PlayerCamera)>,
    mut players: Query<&mut Transform, With<Player>>
) {
    for look_event in look_reader.iter(&look_events){

        for mut transform in players.iter_mut() {
            let yaw_delta = Quat::from_rotation_ypr(
                look_event.direction.x * DEG2RAD * -1.0,
                0.0,
                0.0,
            );

            transform.rotation = transform.rotation * yaw_delta;
        }

        for (mut transform, mut player_camera) in player_cameras.iter_mut() {
            player_camera.current_pitch = player_camera.current_pitch + look_event.direction.y;
            player_camera.current_pitch = clamp(player_camera.current_pitch, player_camera.min_pitch, player_camera.max_pitch);
            let pitch = Quat::from_rotation_ypr(
                0.0,
                player_camera.current_pitch * DEG2RAD * -1.0,
                0.0
            );

            transform.rotation = pitch;
        }
    }
}