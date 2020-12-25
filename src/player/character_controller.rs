use bevy::prelude::*;
use bevy::app::Events;
use crate::input::input_events::*;
use crate::player::player_components::*;

pub struct PlayerController;
impl Plugin for PlayerController {
    fn build(&self, app: &mut AppBuilder){
        app.add_startup_system(setup_player_character.system())
            .add_system(update_walking.system())
            .add_system(update_rotation.system());
    }
}

fn setup_player_character(
    commands: &mut Commands
) {
    commands.spawn(PlayerBundle{
        player: Player{},
        mover: Mover{
            speed: 5.0
        },
        destructable: Destructable{
            max_health: 100.0,
            current_health: 100.0,
        },
    })
    .with_bundle(Camera3dBundle{
        transform: Transform {
            translation: Vec3::unit_y(),
            rotation: Quat::from_axis_angle(Vec3::unit_y(), 0.0),
            scale: Vec3::one()
        },
        ..Default::default()
    });
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

const DEG2RAD: f32 = std::f32::consts::PI / 180.0;

fn update_rotation(
    mut look_reader: Local<EventReader<LookInputEvent>>,
    look_events: Res<Events<LookInputEvent>>,
    mut players: Query<&mut Transform, With<Player>>
) {
    for look_event in look_reader.iter(&look_events){

        for mut transform in players.iter_mut() {
            let rotation_delta = Quat::from_rotation_ypr(
                look_event.direction.x * DEG2RAD * -1.0,
                look_event.direction.y * DEG2RAD * -1.0,
                0.0,
            );

            transform.rotation = transform.rotation.mul_quat(rotation_delta);
        }
    }
}