use crate::{
    automaton_grid::{AutomatonGrid, Example, Examples},
    camera::CameraPlugin,
    cell::CellState,
    color::ColorMethod,
    diagnostic::DiagnosticPlugin,
    instancing::CellMaterialPlugin,
    instancing::{InstanceData, InstanceMaterialData},
    rule::{Indexes, NeighbourMethod, Rule},
    ui::UiPlugin,
};
use bevy::color::palettes::basic::*;
use bevy::prelude::*;
use std::f32::consts::TAU;
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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cellular Automata".into(),
                fit_canvas_to_parent: true,
                window_theme: Some(bevy::window::WindowTheme::Dark),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CellMaterialPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DiagnosticPlugin)
        .add_plugins(UiPlugin)
        .insert_resource(ClearColor(Color::srgb(30. / 255., 30. / 255., 46. / 255.)))
        .insert_resource(AutomatonGrid::default())
        .insert_resource(Examples::default())
        .add_systems(Startup, add_examples)
        .add_systems(
            FixedUpdate,
            update_automaton_grid.run_if(in_state(SimulationState::Running)),
        )
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(30)))
        .init_state::<SimulationState>()
        .add_event::<TogglePauseEvent>()
        .add_systems(Update, toggle_pause)
        .add_systems(Update, rotate_grid)
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

fn rotate_grid(
    mut cube: Query<&mut Transform, With<InstanceMaterialData>>,
    timer: Res<Time>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if !input.pressed(MouseButton::Left) {
        if let Ok(mut transform) = cube.get_single_mut() {
            transform.rotate_y(0.03 * TAU * timer.delta_seconds());
        };
    }
}

fn add_examples(mut examples: ResMut<Examples>) {
    examples.add(Example {
        name: "Builder".to_string(),
        rule: Rule {
            survival_rule: Indexes::new(&[2, 6, 9]),
            birth_rule: Indexes::new(&[4, 6, 8, 9, 10]),
            states: 10,
            neighbour_method: NeighbourMethod::Moore,
        },
        color_method: ColorMethod::DistToCenter,
        color_1: YELLOW.into(),
        color_2: RED.into(),
    });
    examples.add(Example {
        name: "Amoeba".to_string(),
        rule: Rule {
            survival_rule: Indexes::from_range(9..=26),
            birth_rule: Indexes::new(&[5, 6, 7, 12, 13, 15]),
            states: 20,
            neighbour_method: NeighbourMethod::Moore,
        },
        color_method: ColorMethod::StateLerp,
        color_1: YELLOW.into(),
        color_2: BLUE.into(),
    });
    examples.add(Example {
        name: "Large Lines".to_string(),
        rule: Rule {
            survival_rule: Indexes::parse_str("5").unwrap(),
            birth_rule: Indexes::parse_str("4,6,9-11,16-24").unwrap(),
            states: 35,
            neighbour_method: NeighbourMethod::Moore,
        },
        color_method: ColorMethod::StateLerp,
        color_1: BLUE.into(),
        color_2: TEAL.into(),
    });
    examples.add(Example {
        name: "Pretty Crystals".to_string(),
        rule: Rule {
            survival_rule: Indexes::parse_str("5-8").unwrap(),
            birth_rule: Indexes::parse_str("6,7,9").unwrap(),
            states: 15,
            neighbour_method: NeighbourMethod::Moore,
        },
        color_method: ColorMethod::DistToCenter,
        color_1: GREEN.into(),
        color_2: BLUE.into(),
    });
    examples.add(Example {
        name: "Architecture".to_string(),
        rule: Rule {
            survival_rule: Indexes::parse_str("4-6").unwrap(),
            birth_rule: Indexes::parse_str("3").unwrap(),
            states: 25,
            neighbour_method: NeighbourMethod::Moore,
        },
        color_method: ColorMethod::StateLerp,
        color_1: BLUE.into(),
        color_2: RED.into(),
    });
    examples.add(Example {
        name: "Coral".to_string(),
        rule: Rule {
            survival_rule: Indexes::parse_str("5-8").unwrap(),
            birth_rule: Indexes::parse_str("6,7,9,12").unwrap(),
            states: 30,
            neighbour_method: NeighbourMethod::Moore,
        },
        color_method: ColorMethod::StateLerp,
        color_1: YELLOW.into(),
        color_2: GRAY.into(),
    });
    examples.add(Example {
        name: "Diamond".to_string(),
        rule: Rule {
            survival_rule: Indexes::parse_str("5,6").unwrap(),
            birth_rule: Indexes::parse_str("1-3").unwrap(),
            states: 15,
            neighbour_method: NeighbourMethod::VonNeumann,
        },
        color_method: ColorMethod::StateLerp,
        color_1: YELLOW.into(),
        color_2: RED.into(),
    });
    examples.add(Example {
        name: "Infestation".to_string(),
        rule: Rule {
            survival_rule: Indexes::parse_str("3").unwrap(),
            birth_rule: Indexes::parse_str("4").unwrap(),
            states: 50,
            neighbour_method: NeighbourMethod::Moore,
        },
        color_method: ColorMethod::StateLerp,
        color_1: WHITE.into(),
        color_2: BLACK.into(),
    });
    examples.add(Example {
        name: "Expand then die".to_string(),
        rule: Rule {
            survival_rule: Indexes::parse_str("4").unwrap(),
            birth_rule: Indexes::parse_str("3").unwrap(),
            states: 20,
            neighbour_method: NeighbourMethod::Moore,
        },
        color_method: ColorMethod::StateLerp,
        color_1: BLACK.into(),
        color_2: RED.into(),
    });
    examples.add(Example {
        name: "Expand, die, expand, die".to_string(),
        rule: Rule {
            survival_rule: Indexes::parse_str("4").unwrap(),
            birth_rule: Indexes::parse_str("3").unwrap(),
            states: 13,
            neighbour_method: NeighbourMethod::Moore,
        },
        color_method: ColorMethod::StateLerp,
        color_1: BLACK.into(),
        color_2: RED.into(),
    });
    examples.add(Example {
        name: "445".to_string(),
        rule: Rule {
            survival_rule: Indexes::parse_str("4").unwrap(),
            birth_rule: Indexes::parse_str("4").unwrap(),
            states: 5,
            neighbour_method: NeighbourMethod::Moore,
        },
        color_method: ColorMethod::StateLerp,
        color_1: WHITE.into(),
        color_2: RED.into(),
    });
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

#[derive(Event)]
pub struct TogglePauseEvent;

fn toggle_pause(
    mut event_reader: EventReader<TogglePauseEvent>,
    state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    for _ in event_reader.read() {
        match state.get() {
            SimulationState::Paused => next_state.set(SimulationState::Running),
            SimulationState::Running => next_state.set(SimulationState::Paused),
        }
    }
}
