use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum GhostState {
    Default,
    Scared,
    Respawning
}

pub struct Ghost;

#[derive(Bundle)]
pub struct GhostBundle {
    pub ghost: Ghost,
    pub state: GhostState,

    #[bundle]
    pub sprite_bundle: SpriteBundle
}

impl Default for GhostBundle {
    fn default() -> Self {
        Self {
            ghost: Ghost,
            state: GhostState::Default,
            sprite_bundle: SpriteBundle::default()
        }
    }
}

pub struct Caleb;

#[derive(Bundle)]
pub struct CalebBundle {
    pub caleb: Caleb,

    #[bundle]
    pub ghost_bundle: GhostBundle
}

impl Default for CalebBundle {
    fn default() -> Self {
        Self {
            caleb: Caleb,
            ghost_bundle: GhostBundle::default()
        }
    }
}

pub struct Harris;

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
