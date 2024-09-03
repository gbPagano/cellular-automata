use crate::automaton_grid::AutomatonGrid;
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum ColorMethod {
    #[default]
    DistToCenter,
    StateLerp,
    Neighbour,
}
impl ColorMethod {
    #[allow(clippy::too_many_arguments)]
    pub fn get_color(&self, grid: &AutomatonGrid, cell_idx: usize) -> Color {
        let cell_pos_centered = grid.idx_to_pos(cell_idx) - grid.center();
        let dist_to_center = cell_pos_centered.as_vec3().length() / (grid.size as f32 / 2.0);
        let state = grid.cells[cell_idx].get_value(grid.rule.states);
        let neighbours = grid.cells[cell_idx].neighbours;
        let max_neighbours = grid.rule.get_max_neighbours();

        match self {
            ColorMethod::StateLerp => {
                let dt = state as f32 / (grid.rule.states - 1) as f32;
                lerp_color(grid.color_1, grid.color_2, dt)
            }
            ColorMethod::DistToCenter => lerp_color(grid.color_1, grid.color_2, dist_to_center),
            ColorMethod::Neighbour => {
                let dt = neighbours as f32 / max_neighbours as f32;
                lerp_color(grid.color_1, grid.color_2, dt)
            }
        }
    }
}

fn lerp_color(color_1: Color, color_2: Color, dt: f32) -> Color {
    let color_1 = color_1.to_linear();
    let color_2 = color_2.to_linear();
    let dt = dt.clamp(0.0, 1.0);

    Color::LinearRgba((1.0 - dt) * color_1 + dt * color_2)
}
