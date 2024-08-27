use crate::cell::{Cell, CellState};
use crate::rule::Rule;
use bevy::math::IVec3;
use bevy::prelude::*;
use rand::Rng;

#[derive(Resource, Debug)]
pub struct AutomatonGrid {
    pub size: usize,
    pub cells: Vec<Cell>,
    pub rule: Rule,
}

impl AutomatonGrid {
    pub fn new(size: usize, rule: Rule) -> Self {
        let cells = vec![Cell::default(); size.pow(3) as usize];
        let mut grid = Self { size, cells, rule };
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
            self.cells[index] = Cell::new_alive();
        }
    }

    fn count_neighbors(&self, idx: usize) -> u8 {
        let pos = self.idx_to_pos(idx);
        let mut neighbors = 0;
        for dir in self.rule.get_neighbour_iter() {
            let neighbour_pos = self.wrap(pos + *dir);
            let neighbour_cell = &self.cells[self.pos_to_idx(neighbour_pos)];
            if neighbour_cell.state == CellState::Alive {
                neighbors += 1;
            }
        }
        neighbors
    }

    fn calculate_next_cells_state(&mut self) {
        for idx in 0..self.cells.len() {
            let cell = &self.cells[idx];
            let next_state = match cell.state {
                CellState::Empty => {
                    let neighbors = self.count_neighbors(idx);
                    self.rule.apply_birth_rule(neighbors)
                }
                CellState::Alive => {
                    let neighbors = self.count_neighbors(idx);
                    self.rule.apply_survival_rule(neighbors)
                }
                CellState::Dying(state) => self.rule.apply_dying_rule(state),
            };
            self.cells[idx].next_state = Some(next_state);
        }
    }

    fn update_cells_state(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.state = cell.next_state.take().unwrap();
        }
    }

    pub fn update(&mut self) {
        self.calculate_next_cells_state();
        self.update_cells_state();
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_idx_to_pos() {
        let grid = AutomatonGrid::new(5, Rule::default());

        assert_eq!(grid.idx_to_pos(0), IVec3::new(0, 0, 0));
        assert_eq!(grid.idx_to_pos(10), IVec3::new(0, 2, 0));
        assert_eq!(grid.idx_to_pos(124), IVec3::new(4, 4, 4));
        assert_eq!(grid.idx_to_pos(34), IVec3::new(4, 1, 1));
    }

    #[test]
    fn grid_pos_to_idx() {
        let grid = AutomatonGrid::new(5, Rule::default());

        assert_eq!(grid.pos_to_idx(IVec3::new(0, 0, 0)), 0);
        assert_eq!(grid.pos_to_idx(IVec3::new(0, 2, 0)), 10);
        assert_eq!(grid.pos_to_idx(IVec3::new(4, 4, 4)), 124);
        assert_eq!(grid.pos_to_idx(IVec3::new(4, 1, 1)), 34);
    }

    #[test]
    fn wrap() {
        let grid = AutomatonGrid::new(5, Rule::default());

        assert_eq!(grid.wrap(IVec3::new(-1, 1, 2)), IVec3::new(4, 1, 2));
        assert_eq!(grid.wrap(IVec3::new(4, 4, 4)), IVec3::new(4, 4, 4));
        assert_eq!(grid.wrap(IVec3::new(4, 1, 1)), IVec3::new(4, 1, 1));
    }
}
