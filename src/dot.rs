use bevy::prelude::*;

pub struct Dot;

pub struct DotCoordinates(pub f32, pub f32);

#[derive(Bundle)]
pub struct DotBundle {
    pub dot: Dot,
    pub coordinates: DotCoordinates,

    #[bundle]
    pub sprite_bundle: SpriteBundle
}

impl Default for DotBundle {
    fn default() -> Self {
        Self {
            dot: Dot,
            coordinates: DotCoordinates(0., 0.),
            sprite_bundle: SpriteBundle::default()
        }
    }
}
