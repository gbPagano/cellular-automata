use crate::cell::{Cell, CellState};
use bevy::math::IVec3;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy)]
pub struct Rule {
    survival_rule: Indexes,
    birth_rule: Indexes,
    states: u8,
    neighbour_method: NeighbourMethod,
}
impl Rule {
    pub fn get_neighbour_iter(&self) -> &'static [IVec3] {
        self.neighbour_method.get_iter()
    }

    pub fn apply_birth_rule(&self, neighbors: u8) -> CellState {
        if self.birth_rule.has(neighbors) {
            CellState::Alive
        } else {
            CellState::Empty
        }
    }

    pub fn apply_survival_rule(&self, neighbors: u8) -> CellState {
        if !self.survival_rule.has(neighbors) {
            CellState::Dying(self.states)
        } else {
            CellState::Alive
        }
    }

    pub fn apply_dying_rule(&self, state: u8) -> CellState {
        if state == 0 {
            CellState::Empty
        } else {
            CellState::Dying(state - 1)
        }
    }
}
impl Default for Rule {
    fn default() -> Self {
        Self {
            survival_rule: Indexes::from_range(1..=3),
            birth_rule: Indexes::from_range(1..=3),
            states: 4,
            neighbour_method: NeighbourMethod::VonNeumann,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Indexes([bool; 27]);
impl Indexes {
    pub fn new(indexes: &[u8]) -> Self {
        let mut result = Self([false; 27]);
        for index in indexes {
            result.0[*index as usize] = true;
        }
        result
    }
    pub fn from_range(indexes: RangeInclusive<u8>) -> Self {
        let mut result = Self([false; 27]);
        for index in indexes {
            result.0[index as usize] = true;
        }
        result
    }
    pub fn has(&self, idx: u8) -> bool {
        assert!(idx < 26);
        self.0[idx as usize]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NeighbourMethod {
    Moore,
    VonNeumann,
}
impl NeighbourMethod {
    pub fn get_iter(&self) -> &'static [IVec3] {
        match self {
            NeighbourMethod::Moore => &MOOSE_NEIGHBOURS[..],
            NeighbourMethod::VonNeumann => &VONNEUMAN_NEIGHBOURS[..],
        }
    }
}

pub static VONNEUMAN_NEIGHBOURS: [IVec3; 6] = [
    IVec3::new(1, 0, 0),
    IVec3::new(1, 0, 0),
    IVec3::new(0, 1, 0),
    IVec3::new(0, -1, 0),
    IVec3::new(0, 0, -1),
    IVec3::new(0, 0, 1),
];

pub static MOOSE_NEIGHBOURS: [IVec3; 26] = [
    IVec3::new(1, -1, -1),
    IVec3::new(0, -1, -1),
    IVec3::new(1, -1, -1),
    IVec3::new(1, 0, -1),
    IVec3::new(0, 0, -1),
    IVec3::new(1, 0, -1),
    IVec3::new(1, 1, -1),
    IVec3::new(0, 1, -1),
    IVec3::new(1, 1, -1),
    IVec3::new(1, -1, 0),
    IVec3::new(0, -1, 0),
    IVec3::new(1, -1, 0),
    IVec3::new(1, 0, 0),
    IVec3::new(1, 0, 0),
    IVec3::new(1, 1, 0),
    IVec3::new(0, 1, 0),
    IVec3::new(1, 1, 0),
    IVec3::new(1, -1, 1),
    IVec3::new(0, -1, 1),
    IVec3::new(1, -1, 1),
    IVec3::new(1, 0, 1),
    IVec3::new(0, 0, 1),
    IVec3::new(1, 0, 1),
    IVec3::new(1, 1, 1),
    IVec3::new(0, 1, 1),
    IVec3::new(1, 1, 1),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_indexes_from_slice() {
        let indexes = Indexes::new(&[1, 2, 3, 26]);
        let mut expected = [false; 27];
        expected[1] = true;
        expected[2] = true;
        expected[3] = true;
        expected[26] = true;

        assert_eq!(indexes.0, expected);
    }

    #[test]
    fn new_indexes_from_range() {
        let indexes = Indexes::from_range(1..=3);
        let mut expected = [false; 27];
        expected[1] = true;
        expected[2] = true;
        expected[3] = true;

        assert_eq!(indexes.0, expected);
    }

    #[test]
    fn birth_rule() {
        let rule = Rule::default();

        assert_eq!(rule.apply_birth_rule(0), CellState::Empty);
        assert_eq!(rule.apply_birth_rule(2), CellState::Alive);
        assert_eq!(rule.apply_birth_rule(9), CellState::Empty);
    }

    #[test]
    fn survival_rule() {
        let rule = Rule::default();

        assert_eq!(rule.apply_survival_rule(0), CellState::Dying(rule.states));
        assert_eq!(rule.apply_survival_rule(3), CellState::Alive);
    }

    #[test]
    fn dying_rule() {
        let rule = Rule::default();

        assert_eq!(rule.apply_dying_rule(0), CellState::Empty);
        assert_eq!(rule.apply_dying_rule(3), CellState::Dying(2));
    }
}