use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{DebugPanelPages, DebugPanelPagesBundle};

// impl DebugPanelPagesBundle {
//     pub fn physics() -> Self {
//         Self(
//             DebugPanelPages::Physics,
//             TextBundle::from_sections([TextSection::default(), TextSection::default()]),
//         )
//     }
// }

pub fn page_physics(
    rapier_config: Res<DebugRenderContext>,
    mut query: Query<(&DebugPanelPages, &mut Text)>,
) {
    if let Ok((DebugPanelPages::Physics, mut text)) = query.get_single_mut() {
        ["Render hitboxes: ", &rapier_config.enabled.to_string()]
            .iter()
            .enumerate()
            .for_each(|(i, value)| {
                // text.sections[i].value = value.to_string();
            });
    }
}
