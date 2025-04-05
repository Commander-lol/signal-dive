use crate::system::{GameState, SystemColours};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands) {
	commands.spawn((
		Camera2d::default(),
		Msaa::Off,
		OrthographicProjection {
			far: 4000.0,
			scale: 0.80,
			scaling_mode: ScalingMode::AutoMin {
				min_width: 1280.,
				min_height: 720.,
			},
			..OrthographicProjection::default_2d()
		},
		Camera {
			clear_color: ClearColorConfig::Custom(SystemColours::MURKY_BLUE),
			..default()
		},
	));
}

// TODO: Camera tracking system

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::MainMenu), spawn_camera);
	}
}
