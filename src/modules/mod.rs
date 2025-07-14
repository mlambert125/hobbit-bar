mod active_window;
mod battery;
mod bluetooth;
mod clock;
mod hypr_workspaces;
mod media;
mod volume;
mod wifi;

pub use active_window::module_active_window;
pub use battery::module_battery;
pub use bluetooth::module_bluetooth;
pub use clock::module_clock;
pub use hypr_workspaces::module_hypr_workspaces;
pub use media::module_media;
pub use volume::module_volume;
pub use wifi::module_wifi;
