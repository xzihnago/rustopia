use bevy::{
    ecs::{
        component::{ComponentHooks, ComponentId, StorageType},
        world::DeferredWorld,
    },
    prelude::*,
};
use bevy_rapier3d::prelude::*;

/// A component that automatically creates a collider for an entity based on its mesh
///
/// # Panics
/// Panics if the entity does not have a mesh or if the mesh is empty
// TODO: Bevy 0.15 (https://github.com/bevyengine/bevy/pull/14005)
// #[derive(Component)]
// #[component(on_add = auto_collider_hook)]
pub struct AutoCollider;

impl Component for AutoCollider {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(auto_collider_hook);
    }
}

fn auto_collider_hook(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    let meshes = world.get_resource::<Assets<Mesh>>().unwrap();

    match world.get::<Handle<Mesh>>(entity) {
        Some(mesh) => match meshes.get(mesh) {
            Some(mesh) => match Collider::from_bevy_mesh(mesh, &ComputedColliderShape::default()) {
                Some(collider) => {
                    world
                        .commands()
                        .entity(entity)
                        .remove::<AutoCollider>()
                        .insert(collider);
                }

                None => panic!("Failed to create collider for entity {:?}", entity),
            },

            None => panic!("Failed to get {:?} of {:?}", mesh, entity),
        },

        None => panic!("{:?} does not have a mesh", entity),
    }
}
