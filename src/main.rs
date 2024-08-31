use crate::automaton_grid::AutomatonGrid;
use crate::camera::CameraPlugin;
use crate::cell::{Cell, CellState};
use crate::instancing::CellMaterialPlugin;
use crate::instancing::{InstanceData, InstanceMaterialData};
use crate::rule::{Indexes, NeighbourMethod, Rule};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use color::ColorMethod;
use std::time::Duration;

mod automaton_grid;
mod camera;
mod cell;
mod color;
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
        .insert_resource(ClearColor(Color::rgb(0.65f32, 0.9f32, 0.96f32)))
        .insert_resource(AutomatonGrid::new(
            64,
            Rule {
                survival_rule: Indexes::new(&[2, 6, 9]),
                birth_rule: Indexes::new(&[4, 6, 8, 9, 10]),
                states: 10,
                neighbour_method: NeighbourMethod::Moore,
            },
            //Rule {
            //    survival_rule: Indexes::from_range(9..=26),
            //    birth_rule: Indexes::new(&[5, 6, 7, 12, 13, 15]),
            //    states: 20,
            //    neighbour_method: NeighbourMethod::Moore,
            //},
            //ColorMethod::StateLerp,
            ColorMethod::DistToCenter,
            Srgba::rgb(1., 1., 0.).into(),
            Srgba::rgb(1., 0., 0.).into(),
            //Srgba::rgb(0., 0., 1.).into(),
        ))
        .add_systems(FixedUpdate, update_automaton_grid)
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(20)))
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
        let color = grid.get_color_by_idx(idx);
        instance_data.push(InstanceData {
            position: (pos - grid_center).as_vec3(),
            scale: 1.0,
            color: color.to_linear().to_vec4().into(),
        });
    }
}
