use bevy::prelude::*;
use std::collections::*;
use crate::graphics::animation_components::*;

const DEG2RAD: f32 = std::f32::consts::PI / 180.0;

pub struct TestSpriteAnimator;
impl Plugin for TestSpriteAnimator {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_test_animation.system());
    }
}

fn create_test_animation(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    let texture_h = asset_server.load("textures/letters/letter_H.png");
    let texture_e = asset_server.load("textures/letters/letter_E.png");
    let texture_l = asset_server.load("textures/letters/letter_L.png");
    let texture_o = asset_server.load("textures/letters/letter_O.png");

    let idle_animation = Animation {
        frames: vec![texture_h.clone(), texture_e.clone(), texture_l.clone(), texture_l.clone(), texture_o.clone()],
        current_frame: 0
    };

    let mut animations = HashMap::new();
    animations.insert(AnimationType::Idle, idle_animation);
    
    commands.spawn(PbrBundle{
        transform: Transform{
            translation: Vec3::new(0.0, 0.5, -6.0),
            rotation: Quat::identity(),
            scale: Vec3::one()
        },
        material: materials.add(texture_h.clone().into()),
        mesh: meshes.add(Mesh::from(shape::Quad { size: Vec2::one(), flip: false })),
        ..Default::default()
    })
    .with_bundle(AnimatedSpriteBundle{
        animations: animations,
        current_animation: AnimationType::Idle,
        ..Default::default()
    });
    info!("loading sprite animation test");
}