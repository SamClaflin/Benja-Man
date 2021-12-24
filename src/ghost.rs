use bevy::prelude::*;
use crate::path::Path;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum AttackState {
    Attacking,
    Scared
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum ReleaseState {
    Caged,
    Releasing,
    Released
}

pub struct Ghost;

pub struct GhostPath(pub Path);

pub struct GhostSpeed(pub f32);

pub struct GhostScareTimer(pub Timer);

impl Default for GhostScareTimer {
    fn default() -> Self {
        GhostScareTimer(Timer::from_seconds(10., false))
    }
}

pub struct GhostReleaseTimer(pub Timer);

impl Default for GhostReleaseTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(5., false))
    }
}

#[derive(Bundle)]
pub struct GhostBundle {
    pub ghost: Ghost,
    pub attack_state: AttackState,
    pub release_state: ReleaseState,
    pub path: GhostPath,
    pub speed: GhostSpeed,

    #[bundle]
    pub sprite_bundle: SpriteBundle
}

impl Default for GhostBundle {
    fn default() -> Self {
        Self {
            ghost: Ghost,
            attack_state: AttackState::Attacking,
            release_state: ReleaseState::Caged,
            path: GhostPath(Path::new()),
            speed: GhostSpeed(2.),
            sprite_bundle: SpriteBundle::default()
        }
    }
}

pub struct Caleb;

pub struct CalebPathChangeTimer(pub Timer);

pub struct CalebMaterials {
    pub default_material: Handle<ColorMaterial>,
    pub scared_material: Handle<ColorMaterial>
}

#[derive(Bundle)]
pub struct CalebBundle {
    pub caleb: Caleb,
    pub path_change_timer: CalebPathChangeTimer,

    #[bundle]
    pub ghost_bundle: GhostBundle
}

impl Default for CalebBundle {
    fn default() -> Self {
        Self {
            caleb: Caleb,
            path_change_timer: CalebPathChangeTimer(Timer::from_seconds(2., true)),
            ghost_bundle: GhostBundle::default()
        }
    }
}

pub struct Harris;

pub struct HarrisMaterials {
    pub default_material: Handle<ColorMaterial>,
    pub scared_material: Handle<ColorMaterial>
}

#[derive(Bundle)]
pub struct HarrisBundle {
    pub sam: Harris,

    #[bundle]
    pub ghost_bundle: GhostBundle
}

impl Default for HarrisBundle {
    fn default() -> Self {
        Self {
            sam: Harris,
            ghost_bundle: GhostBundle::default()
        }
    }
}

pub struct Claflin;

pub struct ClaflinMaterials {
    pub default_material: Handle<ColorMaterial>,
    pub scared_material: Handle<ColorMaterial>
}

#[derive(Bundle)]
pub struct ClaflinBundle {
    pub neyton: Claflin,

    #[bundle]
    pub ghost_bundle: GhostBundle
}

impl Default for ClaflinBundle {
    fn default() -> Self {
        Self {
            neyton: Claflin,
            ghost_bundle: GhostBundle::default()
        }
    }
}

pub struct Samson;

pub struct SamsonMaterials {
    pub default_material: Handle<ColorMaterial>,
    pub scared_material: Handle<ColorMaterial>
}

#[derive(Bundle)]
pub struct SamsonBundle {
    pub samson: Samson,

    #[bundle]
    pub ghost_bundle: GhostBundle 
}

impl Default for SamsonBundle {
    fn default() -> Self {
        Self {
            samson: Samson,
            ghost_bundle: GhostBundle::default()
        }
    }
}
