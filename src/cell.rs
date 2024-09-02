#[derive(Debug, Clone)]
pub struct Cell {
    pub state: CellState,
    pub neighbours: u8,
}

impl Cell {
    pub fn increase_neighbours(&mut self) {
        self.neighbours += 1;
    }
    pub fn decrease_neighbours(&mut self) {
        if self.neighbours > 0 {
            self.neighbours -= 1;
        }
    }
    pub fn get_value(&self, total_states: u8) -> u8 {
        match self.state {
            CellState::Empty => 0,
            CellState::Alive => total_states - 1,
            CellState::Dying(state) => state,
        }

    }
}
impl Default for Cell {
    fn default() -> Self {
        Self {
            state: CellState::Empty,
            neighbours: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Empty,
    Alive,
    Dying(u8),
}
