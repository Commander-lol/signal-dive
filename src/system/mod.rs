use bevy::app::PluginGroupBuilder;
use bevy::prelude::PluginGroup;

mod assets;
mod cameras;
mod colours;
mod state;

pub use assets::AssetLibrary;
pub use cameras::{CameraFollow, MainCamera, TrackCameraPosition};
pub use colours::SystemColours;
pub use state::{GameState, const_transition_state, is_window_focused};

pub struct SystemPluginGroup;
impl PluginGroup for SystemPluginGroup {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(state::GameStatePlugin)
			.add(assets::AssetsPlugin)
			.add(cameras::CameraPlugin)
	}
}
