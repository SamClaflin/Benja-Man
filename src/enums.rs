use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, SystemLabel)]
pub enum Label {
    BenControllerSystem,
    BenMovementSystem,
    BenGhostCollisionSystem,
    RestartGameSystem,
    WaitForRestartSystem,
    ThrottleFramerateSystem
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameState {
    Wait,
    Default,
    End,
    Reset
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CollisionType {
    Approximate,
    Exact
}
