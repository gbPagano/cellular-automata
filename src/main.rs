use crate::automaton_grid::AutomatonGrid;
use crate::camera::CameraPlugin;
use crate::cell::{Cell, CellState};
use crate::instancing::CellMaterialPlugin;
use crate::instancing::{InstanceData, InstanceMaterialData};
use crate::rule::{Indexes, NeighbourMethod, Rule};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

mod automaton_grid;
mod camera;
mod cell;
mod instancing;
mod rule;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CellMaterialPlugin))
        .add_plugins((
            ScreenDiagnosticsPlugin::default(),
            ScreenFrameDiagnosticsPlugin,
        ))
        //.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()))
        .add_plugins(CameraPlugin)
        .insert_resource(AutomatonGrid::new(
            64,
            Rule {
                survival_rule: Indexes::new(&[2, 6, 9]),
                birth_rule: Indexes::new(&[4, 6, 8, 9, 10]),
                states: 10,
                neighbour_method: NeighbourMethod::Moore,
            },
        ))
        .add_systems(Update, update_automaton_grid)
        .run();
}

fn update_automaton_grid(
    mut grid: ResMut<AutomatonGrid>,
    mut query: Query<&mut InstanceMaterialData>,
) {
    grid.update();

    let grid_center = grid.center();
    let instance_data = &mut query.iter_mut().next().unwrap().0;
    instance_data.clear();
    for (idx, cell) in grid.cells.iter().enumerate() {
        if let CellState::Empty = cell.state {
            continue;
        }
        let pos = grid.idx_to_pos(idx);
        instance_data.push(InstanceData {
            position: (pos - grid_center).as_vec3(),
            scale: 1.0,
            color: LinearRgba::from(Color::hsla(265., 0.92, 0.67, 1.)).to_f32_array(),
        });
    }
}
