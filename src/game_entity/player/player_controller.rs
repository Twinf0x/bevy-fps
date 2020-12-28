use bevy::prelude::*;
use bevy::app::Events;
use bevy::math::*;
use bevy::utils::Duration;
use crate::input::input_events::*;
use crate::game_entity::behaviour_components::*;
use crate::game_entity::player::player_components::*;
use crate::game_entity::weapons::weapon_components::*;
use crate::graphics::sprite_components::*;

const DEG2RAD: f32 = std::f32::consts::PI / 180.0;

pub struct PlayerController;
impl Plugin for PlayerController {
    fn build(&self, app: &mut AppBuilder){
        app.add_startup_system(setup_player_character.system())
            .add_system(update_walking.system())
            .add_system(update_first_person_camera.system())
            .add_system(update_weapons.system())
            .add_system(update_player_death.system());
    }
}

fn setup_player_character(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
) {
    // Parent entity
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

    // Child entity for the camera
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

    // Child entity containing the weapon logic
    let mut weapon_bundle = WeaponBundle{
        transform: Transform {
            translation: Vec3::unit_y(),
            rotation: Quat::from_axis_angle(Vec3::unit_y(), 0.0),
            scale: Vec3::one()
        },
        global_transform: GlobalTransform::default(),
        weapon: Weapon{
            is_active: true,
            name: "Shotgun".to_string(),
            max_ammo: 48,
            current_ammo: 48,
            magazine_size: 8,
            current_magazine: 8,
            reload_timer: Timer::new(Duration::from_millis(2000), false),
            fire_timer: Timer::new(Duration::from_millis(400), false),
            bullet_hit_range: 0.15,
            bullet_damage_on_hit: 50.0,
            bullet_speed: 25.0,
            bullet_texture: asset_server.load("textures/bullet/bullet_air.png")
        }
    };
    weapon_bundle.weapon.reload_timer.pause();
    weapon_bundle.weapon.fire_timer.pause();
    
    commands.spawn(weapon_bundle)
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

fn update_weapons(
    time: Res<Time>,
    commands: &mut Commands,
    mut reload_reader: Local<EventReader<ReloadInputEvent>>,
    reload_events: Res<Events<ReloadInputEvent>>,
    mut shoot_reader: Local<EventReader<ShootInputEvent>>,
    shoot_events: Res<Events<ShootInputEvent>>,
    mut weapons: Query<(&GlobalTransform, &mut Weapon)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    for (transform, mut weapon) in weapons.iter_mut() {
        // Only update the active weapon
        if !weapon.is_active {
            continue;
        }

        // Do nothing if we're currently reloading
        weapon.reload_timer.tick(time.delta_seconds());
        if !weapon.reload_timer.paused() && !weapon.reload_timer.finished() {
            continue;
        }

        // Tell weapon if it has been reloaded
        if weapon.reload_timer.just_finished() {
            weapon.reload();
        }

        // Check if we should start reloading
        if weapon.current_magazine < weapon.magazine_size {
            for _ in reload_reader.iter(&reload_events) {
                weapon.reload_timer.reset();
                weapon.reload_timer.unpause();
                break;
            }
        }
        
        // Check if the firerate allows us to shoot
        weapon.fire_timer.tick(time.delta_seconds());
        if !weapon.fire_timer.paused() && !weapon.fire_timer.finished() {
            continue;
        }

        // Check if we should fire the weapon
        if weapon.current_magazine > 0 {
            for _ in shoot_reader.iter(&shoot_events) {
                // Blast the sh** out of our enemies
                weapon.current_magazine -= 1;
                weapon.current_ammo -= 1;

                // TODO Maybe add a light to the bullets.. they're pretty invisible right now
                // Buuut you can only have 10 Light Sources before Bevy crashes, so no
                commands.spawn(PbrBundle{
                    transform: Transform::from_translation(transform.translation),
                    mesh: meshes.add(Mesh::from(shape::Quad { size: Vec2::splat(0.2), flip: true })),
                    material: materials.add(weapon.bullet_texture.clone().into()),
                    visible: Visible {
                        is_transparent: true,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(BillboardSprite{})
                .with(Bullet {
                    hit_range: weapon.bullet_hit_range,
                    damage_on_hit: weapon.bullet_damage_on_hit,
                    velocity: transform.forward() * weapon.bullet_speed * -1.0
                });

                weapon.fire_timer.reset();
                weapon.fire_timer.unpause();
                break;
            }
        }
    }
}

fn update_player_death(players: Query<&Destructable, With<Player>>) {
    for player in players.iter() {
        if player.current_health <= 0.0 {
            info!("You died! To do: restart or something");
        }
    }
}