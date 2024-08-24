use crate::rule::*;

pub struct Cell {
    color: Color,
    state: CellState,
    next_state: CellState,
}

type Color = (u8, u8, u8);

pub enum CellState {
    Empty,
    Alive,
    Dying(u16),
}
