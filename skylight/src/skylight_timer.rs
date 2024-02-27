use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource)]
pub struct SkylightTimer(Timer);

impl SkylightTimer {
    pub fn finished(&self) -> bool {
        self.0.finished()
    }

    pub fn tick(&mut self, delta: Duration) {
        self.0.tick(delta);
    }
}

impl Default for SkylightTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1. / 20., TimerMode::Repeating))
    }
}
