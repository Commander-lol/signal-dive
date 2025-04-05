use crate::system::{AssetLibrary, GameState};
use avian2d::prelude::{Collider, ExternalForce, ExternalTorque, LinearDamping, LockedAxes, MaxLinearSpeed, RigidBody};
use bevy::prelude::*;
use num_traits::float::FloatCore;
use std::fmt::Debug;
use leafwing_input_manager::InputManagerBundle;
use crate::entities::{create_default_input_map, DirectionTracked, MovePower};

pub fn spawn_test_player(mut commands: Commands, library: Res<AssetLibrary>) {
	commands
		.spawn((
			(
				Transform::from_rotation(Quat::from_rotation_z(90.0.to_radians())),
				Visibility::default(),
				DirectionTracked,
			),
			(
				RigidBody::Dynamic,
				Collider::capsule(14.0, 80.0),
				LinearDamping(2.0),
				ExternalForce::default().with_persistence(false),
				// ExternalTorque::default().with_persistence(false),
				MaxLinearSpeed(150.0),
				MovePower(50.0),
			),
			InputManagerBundle::with_map(create_default_input_map()),
		))
		.with_children(|commands| {
			commands.spawn((
				Transform::from_rotation(Quat::from_rotation_z((-90.0f32).to_radians()))
					.with_translation(Vec3::new(2.0, 2.0, 0.0)),
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
		app.add_systems(OnEnter(GameState::MainMenu), (spawn_test_player,));
	}
}
