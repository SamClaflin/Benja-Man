use bevy::prelude::*;

pub struct PowerUp;

pub struct PowerUpAnimationTimer(pub Timer);

pub struct PowerUpMaterials {
    pub material_1: Handle<ColorMaterial>,
    pub material_2: Handle<ColorMaterial>
}

#[derive(Bundle)]
pub struct PowerUpBundle {
    pub power_up: PowerUp,
    pub animation_timer: PowerUpAnimationTimer,

    #[bundle]
    pub sprite_bundle: SpriteBundle
}

impl Default for PowerUpBundle {
    fn default() -> Self {
        Self {
            power_up: PowerUp,
            animation_timer: PowerUpAnimationTimer(Timer::from_seconds(1., true)),
            sprite_bundle: SpriteBundle::default()
        } 
    }
}
