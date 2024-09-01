use std::time::Duration;

use crate::{
    automaton_grid::AutomatonGrid,
    color::ColorMethod,
    rule::{Indexes, NeighbourMethod},
};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, ui_system);
    }
}

#[derive(Default)]
struct UiState {
    birth_rule: String,
    survival_rule: String,
}

fn ui_system(
    mut grid: ResMut<AutomatonGrid>,
    mut contexts: EguiContexts,
    mut background_color: ResMut<ClearColor>,
    mut update_time: ResMut<Time<Fixed>>,
    mut ui_state: Local<UiState>,
) {
    egui::Window::new("Cellular Automata").show(contexts.ctx_mut(), |ui| {
        ui.label("Background Color");
        color_picker(ui, &mut background_color.0);
        ui.label("Simulator:");
        if ui.button("Reset").clicked() {
            grid.reset();
        }

        let mut size = grid.size;
        ui.add(egui::Slider::new(&mut size, 32..=96).text("Grid size"));
        grid.set_size(size);

        let mut millis = update_time.timestep().as_millis() as usize;
        ui.add(egui::Slider::new(&mut millis, 5..=50).text("Step (milliseconds)"));
        update_time.set_timestep(Duration::from_millis(millis as u64));

        ui.add_space(24.0);
        ui.heading("Rules");

        ui.label("Survival");
        let survival = ui.text_edit_singleline(&mut ui_state.survival_rule);
        if survival.changed() {
            if let Some(survival) = Indexes::parse_str(&ui_state.survival_rule) {
                grid.rule.survival_rule = survival;
            }
        } else if !survival.has_focus() {
            ui_state.survival_rule = grid.rule.survival_rule.to_string();
        }

        ui.label("Birth");
        let birth = ui.text_edit_singleline(&mut ui_state.birth_rule);
        if birth.changed() {
            if let Some(birth) = Indexes::parse_str(&ui_state.birth_rule) {
                grid.rule.birth_rule = birth;
            }
        } else if !birth.has_focus() {
            ui_state.birth_rule = grid.rule.birth_rule.to_string();
        }

        ui.add(egui::Slider::new(&mut grid.rule.states, 1..=50).text("States"));

        egui::ComboBox::from_label("Neighbour Method")
            .selected_text(format!("{:?}", grid.rule.neighbour_method))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut grid.rule.neighbour_method,
                    NeighbourMethod::Moore,
                    "Moore",
                );
                ui.selectable_value(
                    &mut grid.rule.neighbour_method,
                    NeighbourMethod::VonNeumann,
                    "Von Neumann",
                );
            });
        egui::ComboBox::from_label("Color Method")
            .selected_text(format!("{:?}", grid.color_method))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut grid.color_method,
                    ColorMethod::DistToCenter,
                    "Distance to Center",
                );
                ui.selectable_value(&mut grid.color_method, ColorMethod::StateLerp, "State Lerp");
                ui.selectable_value(&mut grid.color_method, ColorMethod::Neighbour, "Neighbours");
            });
        color_picker(ui, &mut grid.color_1);
        color_picker(ui, &mut grid.color_2);
    });
}

fn color_picker(ui: &mut egui::Ui, color: &mut Color) {
    let mut c = [
        (color.to_srgba().red * 255.0) as u8,
        (color.to_srgba().green * 255.0) as u8,
        (color.to_srgba().blue * 255.0) as u8,
    ];
    egui::color_picker::color_edit_button_srgb(ui, &mut c);
    *color = Color::srgb(c[0] as f32 / 255., c[1] as f32 / 255., c[2] as f32 / 255.);
}
