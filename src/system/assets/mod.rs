use bevy::prelude::*;

mod loader;

use crate::system::{GameState, const_transition_state};
pub use loader::{AssetLibrary, AssetLibraryLoader};

pub struct AssetsPlugin;
impl Plugin for AssetsPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<loader::AssetLibrary>()
			.add_systems(OnEnter(GameState::Preload), loader::register_assets)
			.add_systems(
				OnEnter(GameState::Preload),
				const_transition_state::<{ GameState::Splash }>,
			)
			.add_systems(
				OnEnter(GameState::Splash),
				const_transition_state::<{ GameState::Loading }>,
			)
			.add_systems(
				Update,
				loader::check_assets.run_if(in_state(GameState::Loading)),
			);
	}
}
