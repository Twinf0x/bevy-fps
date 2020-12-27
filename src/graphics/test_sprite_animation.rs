use bevy::prelude::*;
use std::collections::*;
use crate::graphics::sprite_components::*;

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

    let first_animation = Animation {
        frames: vec![texture_h.clone(), texture_e.clone(), texture_l.clone(), texture_l.clone(), texture_o.clone()],
        current_frame: 0
    };
    let second_animation = Animation {
        frames: vec![texture_h.clone(), texture_e.clone(), texture_l.clone(), texture_l.clone(), texture_o.clone()],
        current_frame: 1
    };
    let third_animation = Animation {
        frames: vec![texture_h.clone(), texture_e.clone(), texture_l.clone(), texture_l.clone(), texture_o.clone()],
        current_frame: 2
    };
    let fourth_animation = Animation {
        frames: vec![texture_h.clone(), texture_e.clone(), texture_l.clone(), texture_l.clone(), texture_o.clone()],
        current_frame: 3
    };
    let fifth_animation = Animation {
        frames: vec![texture_h.clone(), texture_e.clone(), texture_l.clone(), texture_l.clone(), texture_o.clone()],
        current_frame: 4
    };

    let mut first_animations = HashMap::new();
    first_animations.insert(AnimationType::Idle, first_animation);
    let mut second_animations = HashMap::new();
    second_animations.insert(AnimationType::Idle, second_animation);
    let mut third_animations = HashMap::new();
    third_animations.insert(AnimationType::Idle, third_animation);
    let mut fourth_animations = HashMap::new();
    fourth_animations.insert(AnimationType::Idle, fourth_animation);
    let mut fifth_animations = HashMap::new();
    fifth_animations.insert(AnimationType::Idle, fifth_animation);
    
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
        animations: first_animations,
        current_animation: AnimationType::Idle,
        ..Default::default()
    })
    .with(BillboardSprite{});
    commands.spawn(PbrBundle{
        transform: Transform{
            translation: Vec3::new(1.5, 0.5, -6.0),
            rotation: Quat::identity(),
            scale: Vec3::one()
        },
        material: materials.add(texture_h.clone().into()),
        mesh: meshes.add(Mesh::from(shape::Quad { size: Vec2::one(), flip: false })),
        ..Default::default()
    })
    .with_bundle(AnimatedSpriteBundle{
        animations: second_animations,
        current_animation: AnimationType::Idle,
        ..Default::default()
    })
    .with(BillboardSprite{});
    commands.spawn(PbrBundle{
        transform: Transform{
            translation: Vec3::new(3.0, 0.5, -6.0),
            rotation: Quat::identity(),
            scale: Vec3::one()
        },
        material: materials.add(texture_h.clone().into()),
        mesh: meshes.add(Mesh::from(shape::Quad { size: Vec2::one(), flip: false })),
        ..Default::default()
    })
    .with_bundle(AnimatedSpriteBundle{
        animations: third_animations,
        current_animation: AnimationType::Idle,
        ..Default::default()
    })
    .with(BillboardSprite{});
    commands.spawn(PbrBundle{
        transform: Transform{
            translation: Vec3::new(4.5, 0.5, -6.0),
            rotation: Quat::identity(),
            scale: Vec3::one()
        },
        material: materials.add(texture_h.clone().into()),
        mesh: meshes.add(Mesh::from(shape::Quad { size: Vec2::one(), flip: false })),
        ..Default::default()
    })
    .with_bundle(AnimatedSpriteBundle{
        animations: fourth_animations,
        current_animation: AnimationType::Idle,
        ..Default::default()
    })
    .with(BillboardSprite{});
    commands.spawn(PbrBundle{
        transform: Transform{
            translation: Vec3::new(6.0, 0.5, -6.0),
            rotation: Quat::identity(),
            scale: Vec3::one()
        },
        material: materials.add(texture_h.clone().into()),
        mesh: meshes.add(Mesh::from(shape::Quad { size: Vec2::one(), flip: false })),
        ..Default::default()
    })
    .with_bundle(AnimatedSpriteBundle{
        animations: fifth_animations,
        current_animation: AnimationType::Idle,
        ..Default::default()
    })
    .with(BillboardSprite{});
}