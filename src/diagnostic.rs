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
        .add_systems(PostStartup, toggle) // starts disabled
        .add_systems(Update, update);
    }
}

fn toggle(mut diags: ResMut<ScreenDiagnostics>) {
    diags.modify("ms/frame").toggle();
    diags.modify("fps").toggle();
}

fn update(input: Res<ButtonInput<KeyCode>>, diags: ResMut<ScreenDiagnostics>) {
    if input.just_pressed(KeyCode::KeyF) {
        toggle(diags);
    }
}
