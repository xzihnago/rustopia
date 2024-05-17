use bevy::{
    core_pipeline::{core_3d::ScreenSpaceTransmissionQuality, fxaa::Sensitivity},
    pbr::ScreenSpaceAmbientOcclusionQualityLevel,
    render::view::Msaa,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GraphicsSettings {
    pub aa: AntiAliasing,
    #[serde(with = "ScreenSpaceAmbientOcclusionQualityLevelDef")]
    pub ssao: ScreenSpaceAmbientOcclusionQualityLevel,
    pub specular_transmission: SpecularTransmission,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub enum AntiAliasing {
    #[default]
    Off,
    #[serde(with = "SensitivityDef")]
    FXAA(Sensitivity),
    #[serde(with = "MsaaDef")]
    MSAA(Msaa),
    TAA,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

// TODO: waiting for bevy 0.14 update, official Debug implementation for Sensitivity
impl std::fmt::Debug for AntiAliasing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AntiAliasing::Off => write!(f, "Off"),
            AntiAliasing::FXAA(sensitivity) => write!(f, "FXAA({})", sensitivity.get_str()),
            AntiAliasing::MSAA(samples) => write!(f, "MSAA({:?})", samples),
            AntiAliasing::TAA => write!(f, "TAA"),
        }
    }
}

// TODO: waiting for bevy 0.14 update, official Debug implementation for ScreenSpaceAmbientOcclusionQualityLevel (#13387)
impl std::fmt::Debug for GraphicsSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ssao = match &self.ssao {
            ScreenSpaceAmbientOcclusionQualityLevel::Low => format!("Low"),
            ScreenSpaceAmbientOcclusionQualityLevel::Medium => format!("Medium"),
            ScreenSpaceAmbientOcclusionQualityLevel::High => format!("High"),
            ScreenSpaceAmbientOcclusionQualityLevel::Ultra => format!("Ultra"),
            ScreenSpaceAmbientOcclusionQualityLevel::Custom {
                slice_count,
                samples_per_slice_side,
            } => format!(
                "Custom {{ slice_count: {}, samples_per_slice_side: {} }}",
                slice_count, samples_per_slice_side
            ),
        };

        write!(
            f,
            "GraphicsSettings {{ anti_aliasing: {:?}, ssao: {}, specular_transmission: {:?} }}",
            self.aa, ssao, self.specular_transmission
        )
    }
}
