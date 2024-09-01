use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum ColorMethod {
    DistToCenter,
    StateLerp,
    Neighbour,
}
impl Default for ColorMethod {
    fn default() -> Self {
        ColorMethod::DistToCenter
    }
}
impl ColorMethod {
    pub fn get_color(
        &self,
        color_1: Color,
        color_2: Color,
        max_state: u8,
        state: u8,
        neighbours: u8,
        max_neighbours: u8,
        dist_to_center: f32,
    ) -> Color {
        match self {
            ColorMethod::StateLerp => {
                let dt = state as f32 / (max_state - 1) as f32;
                lerp_color(color_1, color_2, dt)
            }
            ColorMethod::DistToCenter => lerp_color(color_1, color_2, dist_to_center),
            ColorMethod::Neighbour => {
                let dt = neighbours as f32 / max_neighbours as f32;
                lerp_color(color_1, color_2, dt)
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
