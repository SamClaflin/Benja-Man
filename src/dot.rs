use bevy::prelude::*;

pub struct Dot;

#[derive(Bundle)]
pub struct DotBundle {
    pub dot: Dot,

    #[bundle]
    pub sprite_bundle: SpriteBundle
}

impl Default for DotBundle {
    fn default() -> Self {
        Self {
            dot: Dot,
            sprite_bundle: SpriteBundle::default()
        }
    }
}
