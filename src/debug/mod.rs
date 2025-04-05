use crate::system::{AssetLibrary, GameState};
use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
use std::fmt::Debug;
use num_traits::float::FloatCore;

pub fn spawn_test_camera(mut commands: Commands) {
	commands.spawn((Camera2d::default()));
}

pub fn spawn_test_player(mut commands: Commands, library: Res<AssetLibrary>) {
	commands.spawn((
		(Transform::from_rotation(Quat::from_rotation_z(90.0.to_radians())), Visibility::default()),
		(RigidBody::Dynamic, Collider::capsule(14.0, 90.0)),
	)).with_children(|commands| {
		commands.spawn((
			Transform::from_rotation(Quat::from_rotation_z((-90.0f32).to_radians())),
			Sprite {
				image: library.image("submarine"),
				..default()
			},
		));
	});
}

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			OnEnter(GameState::MainMenu),
			(spawn_test_camera, spawn_test_player),
		);
	}
}
