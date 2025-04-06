use crate::graphics::LAYER_BACKGROUND;
use crate::system::{AssetLibrary, GameState, MainCamera};
use bevy::prelude::*;
use bevy::window::WindowResized;

#[derive(Component)]
struct Background;

fn spawn_background(mut commands: Commands, library: Res<AssetLibrary>, window: Single<&Window>) {
	commands.spawn((
		Background,
		Transform::from_translation(Vec3::new(0.0, 0.0, LAYER_BACKGROUND)),
		Sprite {
			image: library.image("background_noise"),
			image_mode: SpriteImageMode::Tiled {
				tile_y: true,
				tile_x: true,
				stretch_value: 1.0,
			},
			custom_size: Some(window.size()),
			..default()
		},
	));
}

fn update_background_position(
	mut query: Query<&mut Transform, With<Background>>,
	camera_query: Option<
		Single<&Transform, (Changed<Transform>, Without<Background>, With<MainCamera>)>,
	>,
) {
	let Some(camera) = camera_query else {
		return;
	};

	for mut transform in &mut query {
		transform.translation = camera.translation;
	}
}

fn update_background_size(
	mut resize_events: EventReader<WindowResized>,
	mut background: Option<Single<&mut Sprite, With<Background>>>,
) {
	let mut event = resize_events.read().last();

	match (&mut event, &mut background) {
		(Some(event), Some(background)) => {
			background.custom_size = Some(Vec2::new(event.width, event.height));
		}
		_ => {}
	}
}

pub struct BackgroundNoisePlugin;
impl Plugin for BackgroundNoisePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::InGame), spawn_background)
			.add_systems(
				Update,
				(update_background_position, update_background_size)
					.run_if(in_state(GameState::InGame)),
			);
	}
}
