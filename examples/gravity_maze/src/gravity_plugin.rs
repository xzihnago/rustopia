use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::get_sensor_event;

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_gravity);
    }
}

fn update_gravity(mut physic: Query<&mut RapierConfiguration>) {
    let mut physic = physic.single_mut();

    let event = get_sensor_event();
    let data = unsafe { event.__bindgen_anon_1.__bindgen_anon_1.data };
    let gravity = -Vec3::new(data[0], data[1], data[2]);
    debug!("Sensor gravity: {:?}", gravity);

    physic.gravity = gravity;
}
