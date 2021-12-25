use bevy::prelude::*;

pub struct SoundMaterials {
    pub background_sound: Handle<AudioSource>,
    pub slurp_sound: Handle<AudioSource>,
    pub death_sound: Handle<AudioSource>
}

pub struct BackgroundMusicTimer(pub Timer);

pub struct FontMaterial {
    pub handle: Handle<Font>
}

pub struct StartMessage;

pub struct RestartMessage;

pub struct EndMessage;

pub struct EndMessageText(pub String);

impl Default for EndMessageText {
    fn default() -> Self {
        Self(String::new())
    }
}
