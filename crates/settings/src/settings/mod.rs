mod settings;
mod settings_control;
mod settings_graphic;
mod settings_keybind;

use settings_control::ControlSettings;
use settings_graphic::GraphicsSettings;
use settings_keybind::KeybindSettings;

pub use settings::Settings;
pub use settings_graphic::AntiAliasing;
