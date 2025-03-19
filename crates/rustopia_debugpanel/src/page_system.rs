use bevy::{
    diagnostic::{DiagnosticsStore, SystemInfo, SystemInformationDiagnosticsPlugin},
    prelude::*,
};

use crate::{DebugPanelPages, DebugPanelPagesBundle};

// impl DebugPanelPagesBundle {
//     pub fn system() -> Self {
//         Self(
//             DebugPanelPages::System,
//             TextBundle::from_sections([
//                 // OS
//                 TextSection::default(),
//                 TextSection::default(),
//                 TextSection::default(),
//                 TextSection::default(),
//                 TextSection::default(),
//                 TextSection::default(),
//                 // CPU
//                 TextSection::default(),
//                 TextSection::default(),
//                 TextSection::default(),
//                 TextSection::default(),
//                 TextSection::default(),
//                 TextSection::default(),
//                 // Memory
//                 TextSection::default(),
//                 TextSection::default(),
//                 // Usage
//                 TextSection::default(),
//                 TextSection::default(),
//                 TextSection::default(),
//                 TextSection::default(),
//                 TextSection::default(),
//                 TextSection::default(),
//             ])
//             .with_style(Style {
//                 align_items: AlignItems::Stretch,
//                 ..default()
//             }),
//         )
//     }
// }

pub fn page_system(
    sysinfo: Res<SystemInfo>,
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<(&DebugPanelPages, &mut Text)>,
) {
    if let Ok((DebugPanelPages::System, mut text)) = query.get_single_mut() {
        [
            "OS:     ",
            &sysinfo.os,
            "\n",
            "Kernel: ",
            &sysinfo.kernel,
            "\n",
            "CPU:    ",
            &sysinfo.cpu,
            "\n",
            "Cores:  ",
            &sysinfo.core_count,
            "\n",
            "Memory: ",
            &sysinfo.memory,
            "\n\n",
            "CPU Usage:    ",
            &format!(
                "{:.2}%",
                diagnostics
                    .get(&SystemInformationDiagnosticsPlugin::CPU_USAGE)
                    .unwrap()
                    .value()
                    .unwrap()
            ),
            "\n",
            "Memory Usage: ",
            &format!(
                "{:.2}%",
                diagnostics
                    .get(&SystemInformationDiagnosticsPlugin::MEM_USAGE)
                    .unwrap()
                    .value()
                    .unwrap()
            ),
        ]
        .iter()
        .enumerate()
        .for_each(|(i, value)| {
            // text.sections[i].value = value.to_string();
        });
    }
}
