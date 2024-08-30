use crate::cell::{Cell, CellState};
use crate::color::ColorMethod;
use crate::rule::Rule;
use bevy::math::IVec3;
use bevy::prelude::*;
use rand::Rng;

#[derive(Resource, Debug)]
pub struct AutomatonGrid {
    pub size: usize,
    pub cells: Vec<Cell>,
    pub rule: Rule,
    pub color_method: ColorMethod,
    pub color_1: Color,
    pub color_2: Color,
}

impl AutomatonGrid {
    pub fn new(
        size: usize,
        rule: Rule,
        color_method: ColorMethod,
        color_1: Color,
        color_2: Color,
    ) -> Self {
        let cells = vec![Cell::default(); size.pow(3) as usize];
        let mut grid = Self {
            size,
            cells,
            rule,
            color_method,
            color_1,
            color_2,
        };
        // TODO: update this
        grid.spawn_noise();
        grid
    }

    pub fn center(&self) -> IVec3 {
        let half_size = self.size as i32 / 2;
        IVec3::new(half_size, half_size, half_size)
    }

    fn spawn_noise(&mut self) {
        let center = self.center();

        // TODO: check this values
        let amount = 12 * 12 * 12;
        let radius = 6;

        let mut rand = rand::thread_rng();
        for _ in 0..amount {
            let pos = center
                + IVec3::new(
                    rand.gen_range(-radius..=radius),
                    rand.gen_range(-radius..=radius),
                    rand.gen_range(-radius..=radius),
                );
            let index = self.pos_to_idx(self.wrap(pos));
            if self.cells[index].state == CellState::Empty {
                self.cells[index].state = CellState::Alive;
                self.update_neighbours(index, true);
            }
        }
    }

    fn update_neighbours(&mut self, idx: usize, increase: bool) {
        let pos = self.idx_to_pos(idx);
        for dir in self.rule.get_neighbour_iter() {
            let neighbour_pos = self.wrap(pos + *dir);
            let neighbour_idx = self.pos_to_idx(neighbour_pos);

            let neighbour_cell = &mut self.cells[neighbour_idx];
            if increase {
                neighbour_cell.increase_neighbours();
            } else {
                neighbour_cell.decrease_neighbours();
            }
        }
    }

    pub fn update(&mut self) {
        let mut spawns = vec![];
        let mut deaths = vec![];

        for idx in 0..self.cells.len() {
            let cell = &mut self.cells[idx];
            match cell.state {
                CellState::Empty => {
                    cell.state = self.rule.apply_birth_rule(cell.neighbours);
                    if cell.state == CellState::Alive {
                        spawns.push(idx);
                    }
                }
                CellState::Alive => {
                    cell.state = self.rule.apply_survival_rule(cell.neighbours);
                    if cell.state != CellState::Alive {
                        deaths.push(idx);
                    }
                }
                CellState::Dying(state) => {
                    cell.state = self.rule.apply_dying_rule(state);
                }
            }
        }
        for index in spawns {
            self.update_neighbours(index, true);
        }
        for index in deaths {
            self.update_neighbours(index, false);
        }
    }

    pub fn idx_to_pos(&self, idx: usize) -> IVec3 {
        IVec3::new(
            (idx % self.size) as i32,
            (idx / self.size % self.size) as i32,
            (idx / self.size / self.size) as i32,
        )
    }

    pub fn pos_to_idx(&self, pos: IVec3) -> usize {
        let x = pos.x as usize;
        let y = pos.y as usize;
        let z = pos.z as usize;
        x + y * self.size + z * self.size * self.size
    }

    pub fn wrap(&self, pos: IVec3) -> IVec3 {
        // this causes positions that would go outside the grid
        // to be considered as positions at the other end
        let bounds = self.size as i32;
        (pos + bounds) % bounds
    }

    pub fn get_color_by_idx(&self, idx: usize) -> Color {
        let cell_pos_centered = self.idx_to_pos(idx) - self.center();
        let dist_to_center = cell_pos_centered.as_vec3().length() / (self.size as f32 / 2.0);
        
        self.color_method.get_color(
            self.color_1,
            self.color_2,
            self.rule.states,
            self.cells[idx].get_value(self.rule.states),
            self.cells[idx].neighbours,
            dist_to_center,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_idx_to_pos() {
        let grid = AutomatonGrid::new(
            5,
            Rule::default(),
            ColorMethod::default(),
            Color::default(),
            Color::default(),
        );

        assert_eq!(grid.idx_to_pos(0), IVec3::new(0, 0, 0));
        assert_eq!(grid.idx_to_pos(10), IVec3::new(0, 2, 0));
        assert_eq!(grid.idx_to_pos(124), IVec3::new(4, 4, 4));
        assert_eq!(grid.idx_to_pos(34), IVec3::new(4, 1, 1));
    }

    #[test]
    fn grid_pos_to_idx() {
        let grid = AutomatonGrid::new(
            5,
            Rule::default(),
            ColorMethod::default(),
            Color::default(),
            Color::default(),
        );

        assert_eq!(grid.pos_to_idx(IVec3::new(0, 0, 0)), 0);
        assert_eq!(grid.pos_to_idx(IVec3::new(0, 2, 0)), 10);
        assert_eq!(grid.pos_to_idx(IVec3::new(4, 4, 4)), 124);
        assert_eq!(grid.pos_to_idx(IVec3::new(4, 1, 1)), 34);
    }

    #[test]
    fn wrap() {
        let grid = AutomatonGrid::new(
            5,
            Rule::default(),
            ColorMethod::default(),
            Color::default(),
            Color::default(),
        );

        assert_eq!(grid.wrap(IVec3::new(-1, 1, 2)), IVec3::new(4, 1, 2));
        assert_eq!(grid.wrap(IVec3::new(4, 4, 5)), IVec3::new(4, 4, 0));
        assert_eq!(grid.wrap(IVec3::new(4, 1, 1)), IVec3::new(4, 1, 1));
    }
}
