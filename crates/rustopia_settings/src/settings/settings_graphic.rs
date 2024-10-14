use bevy::{
    core_pipeline::{core_3d::ScreenSpaceTransmissionQuality, fxaa::Sensitivity, smaa::SmaaPreset},
    pbr::ScreenSpaceAmbientOcclusionQualityLevel,
    render::view::Msaa,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GraphicSettings {
    pub hdr: bool,
    pub aa: AntiAliasing,
    #[serde(with = "ScreenSpaceAmbientOcclusionQualityLevelDef")]
    pub ssao: ScreenSpaceAmbientOcclusionQualityLevel,
    pub specular_transmission: SpecularTransmission,
}

#[derive(Default, Serialize, Deserialize)]
pub enum AntiAliasing {
    #[default]
    Off,
    #[serde(with = "SensitivityDef")]
    FXAA(Sensitivity),
    #[serde(with = "SmaaPresetDef")]
    SMAA(SmaaPreset),
    #[serde(with = "MsaaDef")]
    MSAA(Msaa),
    TAA,
}

#[derive(Serialize, Deserialize)]
pub struct SpecularTransmission {
    pub step: usize,
    #[serde(with = "ScreenSpaceTransmissionQualityDef")]
    pub quality: ScreenSpaceTransmissionQuality,
}

impl Default for SpecularTransmission {
    fn default() -> Self {
        Self {
            step: 2,
            quality: ScreenSpaceTransmissionQuality::Medium,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Sensitivity")]
pub enum SensitivityDef {
    Low,
    Medium,
    High,
    Ultra,
    Extreme,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "SmaaPreset")]
pub enum SmaaPresetDef {
    Low,
    Medium,
    High,
    Ultra,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Msaa")]
pub enum MsaaDef {
    Off = 1,
    Sample2 = 2,
    Sample4 = 4,
    Sample8 = 8,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ScreenSpaceAmbientOcclusionQualityLevel")]
pub enum ScreenSpaceAmbientOcclusionQualityLevelDef {
    Low,
    Medium,
    High,
    Ultra,
    Custom {
        slice_count: u32,
        samples_per_slice_side: u32,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ScreenSpaceTransmissionQuality")]
pub enum ScreenSpaceTransmissionQualityDef {
    Low,
    Medium,
    High,
    Ultra,
}
