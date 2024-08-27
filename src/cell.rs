use crate::rule::*;

#[derive(Debug, Clone)]
pub struct Cell {
    pub state: CellState,
    pub next_state: Option<CellState>,
}

impl Cell {
    pub fn new_alive() -> Self {
        Self {
            state: CellState::Alive,
            next_state: None,
        }
    }
}
impl Default for Cell {
    fn default() -> Self {
        Self {
            state: CellState::Empty,
            next_state: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Empty,
    Alive,
    Dying(u8),
}
