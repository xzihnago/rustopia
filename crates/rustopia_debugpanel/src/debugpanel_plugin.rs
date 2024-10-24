use bevy::{
    dev_tools::ui_debug_overlay::{DebugUiPlugin, UiDebugOptions},
    diagnostic::{FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin},
    prelude::*,
};
use bevy_rapier3d::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    page_camera, page_physics, page_system, DebugPanel, DebugPanelPages, DebugPanelPagesBundle,
    DebugPanelState,
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
                toggle_debug_ui,
                spawn_debug_panel.run_if(|panel: Res<DebugPanelState>| resource_changed(panel)),
                switch_page,
            ),
        )
        .add_systems(
            Update,
            (set_font_size, page_system, page_camera, page_physics)
                .after(spawn_debug_panel)
                .run_if(|panel: Res<DebugPanelState>| panel.enabled),
        );

        app.init_resource::<DebugPanelState>();
    }
}

fn toggle_debug_panel(keyboard: Res<ButtonInput<KeyCode>>, mut panel: ResMut<DebugPanelState>) {
    if keyboard.just_pressed(KeyCode::Backquote) {
        panel.enabled = !panel.enabled;
    }
}

fn toggle_debug_ui(keyboard: Res<ButtonInput<KeyCode>>, mut debug_ui: ResMut<UiDebugOptions>) {
    if keyboard.just_pressed(KeyCode::Tab) {
        debug_ui.toggle();
    }
}

fn spawn_debug_panel(
    mut commands: Commands,
    panel_state: Res<DebugPanelState>,
    panel: Query<Entity, With<DebugPanel>>,
) {
    panel
        .iter()
        .for_each(|entity| commands.entity(entity).despawn_recursive());

    if panel_state.enabled {
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
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.),
                        ..default()
                    },
                    background_color: Color::srgba(0.1, 0.1, 0.1, 0.99).into(),
                    border_radius: BorderRadius::all(Val::Px(10.)),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_wrap: FlexWrap::Wrap,
                            row_gap: Val::Px(5.),
                            column_gap: Val::Px(5.),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        DebugPanelPages::iter().for_each(|page| {
                            parent
                                .spawn((
                                    page,
                                    ButtonBundle {
                                        style: Style {
                                            justify_content: JustifyContent::Center,
                                            padding: UiRect::all(Val::Px(5.)),
                                            flex_grow: 1.,
                                            ..default()
                                        },
                                        border_radius: BorderRadius::all(Val::Px(5.)),
                                        background_color: if page == panel_state.page {
                                            Color::srgba(0.4, 0.4, 0.4, 0.8).into()
                                        } else {
                                            Color::srgba(0.2, 0.2, 0.2, 0.8).into()
                                        },
                                        ..default()
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        page.as_ref(),
                                        TextStyle {
                                            font_size: 22.,
                                            ..default()
                                        },
                                    ));
                                });
                        });
                    });
            })
            .with_children(|parent| {
                parent.spawn(match panel_state.page {
                    DebugPanelPages::System => DebugPanelPagesBundle::system(),
                    DebugPanelPages::Camera => DebugPanelPagesBundle::camera(),
                    DebugPanelPages::Physics => DebugPanelPagesBundle::physics(),
                    _ => return,
                });
            });
    }
}

fn set_font_size(mut query: Query<&mut Text, Added<DebugPanelPages>>) {
    query.iter_mut().for_each(|mut text| {
        text.sections.iter_mut().for_each(|section| {
            section.style.font_size = 18.;
        });
    });
}

fn switch_page(
    mut panel_state: ResMut<DebugPanelState>,
    mut query: Query<
        (&DebugPanelPages, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    query
        .iter_mut()
        .for_each(|(page, interaction, mut color)| match *interaction {
            Interaction::Pressed => panel_state.page = *page,
            Interaction::Hovered => *color = Color::srgba(0.4, 0.4, 0.4, 0.8).into(),
            Interaction::None if *page != panel_state.page => {
                *color = Color::srgba(0.2, 0.2, 0.2, 0.8).into()
            }
            _ => {}
        });
}
