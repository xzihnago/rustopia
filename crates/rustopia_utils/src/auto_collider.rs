use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
#[component(on_add = auto_collider_hook)]
pub struct AutoCollider;

fn auto_collider_hook(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    let meshes = world.resource::<Assets<Mesh>>();

    match world.get::<Mesh3d>(entity) {
        Some(mesh) => match meshes.get(mesh) {
            Some(mesh) => match Collider::from_bevy_mesh(mesh, &ComputedColliderShape::default()) {
                Some(collider) => {
                    world
                        .commands()
                        .entity(entity)
                        .remove::<AutoCollider>()
                        .insert(collider);
                }

                None => error!("Failed to create collider for entity {:?}", entity),
            },

            None => error!("Failed to get {:?} of {:?}", mesh, entity),
        },

        None => error!("{:?} does not have a mesh", entity),
    }
}
