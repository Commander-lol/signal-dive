use crate::system::{GameState, SystemColours};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use std::collections::VecDeque;
use std::ops::Mul;
use std::time::Duration;

#[derive(Component, Default, Debug, Clone, Copy)]
#[component(storage = "SparseSet")]
pub struct MainCamera;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct CameraFollow;
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct TrackCameraPosition;

pub fn spawn_camera(mut commands: Commands) {
	commands.spawn((
		MainCamera,
		Camera2d::default(),
		Msaa::Off,
		OrthographicProjection {
			far: 4000.0,
			scale: 0.50,
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

#[derive(Default, Resource)]
struct CameraTrackingQueue {
	max_size: usize,
	points: VecDeque<Vec2>,
}

impl CameraTrackingQueue {
	pub fn current(&self) -> Option<&Vec2> {
		self.points.front()
	}

	pub fn add(&mut self, position: Vec2) {
		if self.points.len() >= self.max_size {
			self.points.pop_front();
		}
		self.points.push_back(position);
	}
	pub fn reset(&mut self) {
		self.points.clear();
	}
}

fn reset_camera_tracking(mut tracking_queue: ResMut<CameraTrackingQueue>) {
	tracking_queue.reset();
}

fn lagged_tracking_camera(
	time: Res<Time>,
	trackables: Query<&Transform, With<CameraFollow>>,
	mut positions: ResMut<CameraTrackingQueue>,
	mut timer: Local<Duration>,
) {
	*timer = timer.saturating_add(time.delta());

	if *timer >= Duration::from_secs_f32(0.25) {
		*timer = Duration::ZERO;

		let mut center = Vec2::ZERO;
		for transform in &trackables {
			center += transform.translation.truncate();
		}
		center /= trackables.iter().count() as f32;

		positions.add(center);
	}
}

fn track_last_camera_point(
	time: Res<Time>,
	positions: Res<CameraTrackingQueue>,
	mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
	if let Some(position) = positions.current() {
		for mut camera in &mut camera_query {
			let delta = camera
				.translation
				.truncate()
				.distance(*position)
				.mul(time.delta_secs());

			camera.translation = camera
				.translation
				.move_towards(position.extend(camera.translation.z), delta);
		}
	}
}

fn track_entity_to_camera_position(
	camera_query: Option<Single<&Transform, With<MainCamera>>>,
	mut follower_query: Query<&mut Transform, (With<TrackCameraPosition>, Without<MainCamera>)>,
) {
	let Some(camera_position) = camera_query else {
		return;
	};

	for mut follower in &mut follower_query {
		follower.translation = camera_position
			.translation
			.truncate()
			.extend(follower.translation.z);
	}
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<CameraTrackingQueue>()
			.add_systems(OnEnter(GameState::MainMenu), spawn_camera)
			.add_systems(OnEnter(GameState::InGame), reset_camera_tracking)
			.add_systems(
				Update,
				lagged_tracking_camera.run_if(in_state(GameState::InGame)),
			)
			.add_systems(
				PostUpdate,
				(track_entity_to_camera_position, track_last_camera_point)
					.run_if(in_state(GameState::InGame)),
			);
	}
}
