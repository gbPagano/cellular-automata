#[derive(Debug, Clone)]
pub struct Cell {
    pub state: CellState,
    pub neighbours: u8,
}

impl Cell {
    pub fn new_alive() -> Self {
        Self {
            state: CellState::Alive,
            neighbours: 0,
        }
    }
    pub fn increase_neighbours(&mut self) {
        self.neighbours += 1;
    }
    pub fn decrease_neighbours(&mut self) {
        if self.neighbours > 0 {
            self.neighbours -= 1;
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
