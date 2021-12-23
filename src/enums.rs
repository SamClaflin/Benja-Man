use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(SystemLabel, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Label {
    BenControllerSystem,
    BenMovementSystem,
}
