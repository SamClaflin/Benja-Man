use bevy::prelude::*;

pub struct Ben;

pub struct BenLives(u8);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BenDirection {
    Up,
    Right,
    Down,
    Left
}

#[derive(Bundle)]
pub struct BenBundle {
    pub lives: BenLives,
    #[bundle]        
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub direction: BenDirection
}

impl Default for BenBundle {
    fn default() -> Self {
        Self {
            lives: BenLives(3),
            sprite_sheet_bundle: SpriteSheetBundle::default(),
            direction: BenDirection::Up 
        } 
    }
} 
