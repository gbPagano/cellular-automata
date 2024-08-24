use bevy::math::IVec3;
use std::ops::RangeInclusive;

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
}

pub struct Rule {
    survival_rule: Indexes,
    birth_rule: Indexes,
    states: u16,
    neighbour_method: NeighbourMethod,
}

pub enum NeighbourMethod {
    Moore,
    VonNeumann,
}
impl NeighbourMethod {
    pub fn get_neighbour_iter(&self) -> &'static [IVec3] {
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
}
