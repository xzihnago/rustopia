use bevy::{
    dev_tools::ui_debug_overlay::{DebugUiPlugin, UiDebugOptions},
    diagnostic::{FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

use crate::{
    page_camera, page_physics, page_system_info, DebugPanel, DebugPanelPages,
    DebugPanelPagesBundle, DebugPanelState,
};

pub struct DebugPanelPlugin;

impl Plugin for DebugPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SystemInformationDiagnosticsPlugin)
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(DebugUiPlugin);

        app.add_systems(
            Update,
            (
                toggle_debug_panel,
                spawn_debug_panel.run_if(|panel: Res<DebugPanelState>| resource_changed(panel)),
            ),
        )
        .add_systems(
            Update,
            (set_font_size, page_system_info, page_camera, page_physics)
                .after(spawn_debug_panel)
                .run_if(|panel: Res<DebugPanelState>| panel.enabled),
        );

        app.init_resource::<DebugPanelState>();
    }
}

fn toggle_debug_panel(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_ui: ResMut<UiDebugOptions>,
    mut panel: ResMut<DebugPanelState>,
) {
    if keyboard.just_pressed(KeyCode::Backquote) {
        debug_ui.toggle();
        panel.enabled = !panel.enabled;
    }
}

fn spawn_debug_panel(
    mut commands: Commands,
    panel_state: Res<DebugPanelState>,
    panel: Query<Entity, With<DebugPanel>>,
) {
    match panel_state.enabled {
        false => panel
            .iter()
            .for_each(|entity| commands.entity(entity).despawn_recursive()),

        true => {
            commands
                .spawn((
                    DebugPanel,
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(10.),
                            top: Val::Percent(10.),
                            width: Val::Percent(30.),
                            height: Val::Percent(80.),
                            padding: UiRect::all(Val::Px(10.)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)),
                        border_radius: BorderRadius::all(Val::Px(10.)),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(match panel_state.page {
                        DebugPanelPages::SystemInfo => DebugPanelPagesBundle::system_info(),
                        DebugPanelPages::Camera => DebugPanelPagesBundle::camera(),
                        DebugPanelPages::Physics => DebugPanelPagesBundle::physics(),
                        _ => DebugPanelPagesBundle::system_info(),
                    });
                });
        }
    };
}

fn set_font_size(mut query: Query<&mut Text, Added<DebugPanelPages>>) {
    query.iter_mut().for_each(|mut text| {
        text.sections.iter_mut().for_each(|section| {
            section.style.font_size = 20.;
        });
    });
}
