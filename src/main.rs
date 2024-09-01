use crate::{
    automaton_grid::AutomatonGrid,
    camera::CameraPlugin,
    cell::CellState,
    color::ColorMethod,
    diagnostic::DiagnosticPlugin,
    instancing::CellMaterialPlugin,
    instancing::{InstanceData, InstanceMaterialData},
    rule::{Indexes, NeighbourMethod, Rule},
    ui::UiPlugin,
};
use bevy::prelude::*;
use std::time::Duration;

mod automaton_grid;
mod camera;
mod cell;
mod color;
mod diagnostic;
mod instancing;
mod rule;
mod ui;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CellMaterialPlugin))
        .add_plugins(CameraPlugin)
        .add_plugins(DiagnosticPlugin)
        .add_plugins(UiPlugin)
        .insert_resource(ClearColor(Color::srgb(0.65f32, 0.9f32, 0.96f32)))
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
            color: color.to_srgba().to_vec4().into(),
        });
    }
}
