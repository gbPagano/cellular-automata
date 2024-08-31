use bevy::prelude::*;
use bevy_screen_diagnostics::{
    ScreenDiagnostics, ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin,
};

pub struct DiagnosticPlugin;

impl Plugin for DiagnosticPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ScreenDiagnosticsPlugin::default(),
            ScreenFrameDiagnosticsPlugin,
        ))
        .add_systems(Startup, toggle) // starts disabled
        .add_systems(Update, update);
    }
}

fn toggle(mut diags: ResMut<ScreenDiagnostics>) {
    diags.modify("ms/frame").toggle();
    diags.modify("fps").toggle();
}

fn update(input: Res<ButtonInput<KeyCode>>, mut diags: ResMut<ScreenDiagnostics>) {
    diags
        .modify("fps")
        .diagnostic_color(Color::srgb(1., 0., 0.))
        .name_color(Color::srgb(0., 0., 0.));
    diags
        .modify("ms/frame")
        .diagnostic_color(Color::srgb(1., 0., 0.))
        .name_color(Color::srgb(0., 0., 0.));

    if input.just_pressed(KeyCode::KeyF) {
        toggle(diags); 
    }
}
