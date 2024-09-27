use bevy::prelude::*;

#[derive(Deref, DerefMut, Resource)]
pub struct SkylightTimer(Timer);

impl Default for SkylightTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1. / 20., TimerMode::Repeating))
    }
}
