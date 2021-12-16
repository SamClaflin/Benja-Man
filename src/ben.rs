use bevy::prelude::*;
use crate::enums::Direction;

pub struct Ben;

pub struct BenLives(pub u8);

pub struct BenAnimationTimer(pub Timer);

pub struct BenSpeed(pub f32);

pub struct BenDirection(pub Direction);

#[derive(Bundle)]
pub struct BenBundle {
    pub ben: Ben,
    pub lives: BenLives,
    pub direction: BenDirection,
    pub animation_timer: BenAnimationTimer,
    pub speed: BenSpeed,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

impl Default for BenBundle {
    fn default() -> Self {
        Self {
            ben: Ben,
            lives: BenLives(3),
            direction: BenDirection(Direction::Right),
            animation_timer: BenAnimationTimer(Timer::from_seconds(0.2, true)),
            speed: BenSpeed(8.),
            sprite_bundle: SpriteBundle::default(),
        } 
    }
} 
