use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{DebugPanelPages, DebugPanelPagesBundle};

// impl DebugPanelPagesBundle {
//     pub fn camera() -> Self {
//         Self(
//             DebugPanelPages::Camera,
//             TextBundle::from_sections([
//                 // FPS
//                 TextSection::default(),
//                 TextSection::default(),
//                 TextSection::default(),
//                 // Position
//                 TextSection::default(),
//                 TextSection::default(),
//                 TextSection::default(),
//                 // Rotation
//                 TextSection::default(),
//                 TextSection::default(),
//                 TextSection::default(),
//             ]),
//         )
//     }
// }

pub fn page_camera(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<(&DebugPanelPages, &mut Text)>,
    camera: Query<&Transform, With<Camera>>,
) {
    if let (Ok((DebugPanelPages::Camera, mut text)), Ok(transform)) =
        (query.get_single_mut(), camera.get_single())
    {
        [
            "FPS:      ",
            &format!(
                "{:.2}",
                diagnostics
                    .get(&FrameTimeDiagnosticsPlugin::FPS)
                    .unwrap()
                    .smoothed()
                    .unwrap()
            ),
            "\n",
            "Position: ",
            &format!(
                "{:.3} / {:.3} / {:.3}",
                transform.translation.x, transform.translation.y, transform.translation.z
            ),
            "\n",
            "Rotation: ",
            &format!(
                "{:.3} / {:.3} / {:.3} / {:.3}",
                transform.rotation.x,
                transform.rotation.y,
                transform.rotation.z,
                transform.rotation.w
            ),
        ]
        .iter()
        .enumerate()
        .for_each(|(i, value)| {
            // text.sections[i].value = value.to_string();
        });
    }
}

pub struct PageCameraPlugin;

impl Plugin for PageCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            draw_axis.run_if(|state: Res<PageCameraState>| state.show_axis),
        );
    }
}

fn draw_axis(
    mut gizmos: Gizmos,
    mut gizmo_config: ResMut<GizmoConfigStore>,
    state: Res<PageCameraState>,
    query: Query<&Transform>,
) {
    if state.show_axis {
        for (_, config, _) in gizmo_config.iter_mut() {
            config.depth_bias = -1.;
        }
    } else {
        for (_, config, _) in gizmo_config.iter_mut() {
            config.depth_bias = 0.;
        }
    }

    let transform = query.single();

    let start = transform.translation + transform.forward() * 10.;
    gizmos.line(start, start + Vec3::X, Color::from(RED));
    gizmos.line(start, start + Vec3::Y, Color::from(GREEN));
    gizmos.line(start, start + Vec3::Z, Color::from(BLUE));
}

#[derive(Resource)]
pub struct PageCameraState {
    pub show_axis: bool,
}

impl Default for PageCameraState {
    fn default() -> Self {
        Self { show_axis: false }
    }
}
