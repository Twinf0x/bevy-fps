use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct Hud;
impl Plugin for Hud {
    fn build(&self, app: &mut AppBuilder) {
        
        app
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup.system())
            .add_system(text_update_system.system());
    }
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
struct FpsText;

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
                println!("{:?}", text.value);
            }
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
                value: "FPS:".to_string(),
                font: asset_server.load("fonts/Kenney-Space.ttf"),
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::RED,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(FpsText);
}