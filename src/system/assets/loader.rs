use crate::system::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::tasks::futures_lite::StreamExt;
use bevy_enoki::Particle2dEffect;
use bevy_enoki::prelude::SpriteParticle2dMaterial;
use micro_games_macros::asset_system;

#[asset_system]
pub struct AssetLibrary {
	image: Image,
	font: Font,
	emitter: Particle2dEffect,

	// Virtual Loading
	sprite_particle: SpriteParticle2dMaterial,
}

impl AssetLibraryLoader<'_> {
	pub fn create_sprite_particle(&mut self, name: &str, particle: SpriteParticle2dMaterial) {
		self.storage
			.sprite_particle
			.insert(name.into(), self.server.add(particle));
	}
}

pub fn register_assets(mut assets: AssetLibraryLoader) {
	assets.load_image_list_static(&[
		("sprites/submarine.png", "submarine"),
		("sprites/noise.png", "background_noise"),
		// Emitter bubbles
		("sprites/environment/bubble_small_1.png", "bubble_small_1"),
		("sprites/environment/bubble_small_2.png", "bubble_small_2"),
		("sprites/environment/bubble_medium_1.png", "bubble_medium_1"),
		("sprites/environment/bubble_large_1.png", "bubble_large_1"),
		// Rock environment
		("sprites/environment/rock_rounded_1.png", "rock_rounded_1"),
		("sprites/environment/rock_spike_1.png", "rock_spike_1"),
	]);

	assets.load_emitter_list_static(&[("particles/bubble_emitter.ron", "bubble_emitter")]);

	assets.create_sprite_particle(
		"bubble_emitter",
		SpriteParticle2dMaterial::from_texture(
			assets.server.load("sprites/environment/bubble_small_1.png"),
		),
	);
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
