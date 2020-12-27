use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::game_entity::behaviour_components::*;
use crate::game_entity::player::player_components::*;

pub struct Hud;
impl Plugin for Hud {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup.system())
            .add_system(health_update_system.system())
            .add_system(ammo_update_system.system());
    }
}

struct HealthCount;
struct AmmoCount;

fn health_update_system(players: Query<&Destructable, With<Player>>, mut query: Query<&mut Text, With<HealthCount>>) {
    for mut text in query.iter_mut() {
        for p in players.iter() {
            let health = p.current_health;
            text.value = format!("Health: {:.0}", health);
        }
    }
}

// TODO: get weapon ammo
fn ammo_update_system(mut query: Query<&mut Text, With<AmmoCount>>) {
    for mut text in query.iter_mut() {
        if let ammo = 30 {
            text.value = format!("III: {:.0}", ammo);
        }
    }
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    println!("setup is running");
    commands
        // 2d camera
        .spawn(CameraUiBundle::default())
        // texture
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "HEALTH:".to_string(),
                font: asset_server.load("fonts/Kenney-Space.ttf"),
                style: TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(HealthCount)

        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(0.),
                    right: Val::Px(0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                value: "AMMO:".to_string(),
                font: asset_server.load("fonts/Kenney-Space.ttf"),
                style: TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(AmmoCount);
}