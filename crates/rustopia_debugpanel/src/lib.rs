mod debugpanel;
mod debugpanel_pages;
mod debugpanel_pages_bundle;
mod debugpanel_plugin;
mod debugpanel_state;
mod page_camera;
mod page_physics;
mod page_system_info;

use debugpanel::DebugPanel;
use debugpanel_pages::DebugPanelPages;
use debugpanel_pages_bundle::DebugPanelPagesBundle;
use debugpanel_state::DebugPanelState;
use page_camera::page_camera;
use page_physics::page_physics;
use page_system_info::page_system_info;

pub use debugpanel_plugin::DebugPanelPlugin;
