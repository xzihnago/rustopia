use bevy::prelude::*;

use crate::SkylightTimer;

pub struct SkylightTimerPlugin;

impl Plugin for SkylightTimerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SkylightTimer::default())
            .add_systems(PreUpdate, update);
    }
}

fn update(time: Res<Time>, mut timer: ResMut<SkylightTimer>) {
    timer.tick(time.delta());
}
