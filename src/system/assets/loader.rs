use crate::system::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::tasks::futures_lite::StreamExt;
use micro_games_macros::asset_system;

#[asset_system]
pub struct AssetLibrary {
	image: Image,
	font: Font,
}

pub fn register_assets(mut assets: AssetLibraryLoader) {
	assets.load_image_list_static(&[("sprites/submarine.png", "submarine")]);
}

pub fn check_assets(
	library: Res<AssetLibrary>,
	server: Res<AssetServer>,
	mut next_state: ResMut<NextState<GameState>>,
) {
	let states = [(server.as_ref(), library.image.iter())];

	if states.into_iter().all(check_load_state) {
		log::info!("Loaded all assets");
		next_state.set(GameState::MainMenu);
	}
}

fn check_load_state<'a, T: Asset>(
	(server, asset_handles): (
		&AssetServer,
		impl Iterator<Item = (&'a String, &'a Handle<T>)>,
	),
) -> bool {
	asset_handles
		.map(|(_, handle)| server.get_load_state(handle))
		.all(|state| match state {
			Some(LoadState::Loaded) => true,
			Some(LoadState::Failed(err)) => {
				log::error!("Failed to load asset: {err}");
				false
			}
			_ => false,
		})
}
