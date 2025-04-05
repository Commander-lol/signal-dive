use bevy::app::PluginGroupBuilder;
use bevy::prelude::PluginGroup;

mod assets;
mod state;
mod cameras;
mod colours;

pub use assets::AssetLibrary;
pub use state::{GameState, const_transition_state, is_window_focused};
pub use colours::SystemColours;

pub struct SystemPluginGroup;
impl PluginGroup for SystemPluginGroup {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(state::GameStatePlugin)
			.add(assets::AssetsPlugin)
			.add(cameras::CameraPlugin)
	}
}
