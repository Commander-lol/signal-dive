use crate::entities::{
	CanSpawnExt, DirectionTracked, MovePower, SpawnKelp, create_default_input_map,
};
use crate::graphics::{
	LAYER_BEHIND, LAYER_ENTITIES, LAYER_FRONT, LayerStyle, SwayingObjectSpawner,
};
use crate::system::{AssetLibrary, CameraFollow, GameState, const_transition_state};
use avian2d::prelude::{
	Collider, ExternalForce, ExternalTorque, LinearDamping, LockedAxes, MaxLinearSpeed, RigidBody,
};
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_enoki::{ParticleEffectHandle, ParticleSpawner};
use leafwing_input_manager::InputManagerBundle;
use num_traits::float::FloatCore;
use std::fmt::Debug;

pub fn spawn_test_player(mut commands: Commands, library: Res<AssetLibrary>) {
	commands
		.spawn((
			(
				Transform::from_rotation(Quat::from_rotation_z(90.0.to_radians()))
					.with_translation(Vec3::new(0.0, 0.0, LAYER_ENTITIES)),
				Visibility::default(),
				DirectionTracked,
				CameraFollow,
			),
			(
				RigidBody::Dynamic,
				Collider::capsule(14.0, 80.0),
				LinearDamping(2.0),
				ExternalForce::default().with_persistence(false),
				MaxLinearSpeed(175.0),
				MovePower(40.0),
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

			commands.spawn((
				DirectionTracked,
				Transform::from_translation(Vec3::new(10.0, 24.0, -5.0)),
				ParticleEffectHandle(library.emitter("bubble_emitter")),
				ParticleSpawner(library.sprite_particle("bubble_emitter")),
			));
		});
}

pub fn spawn_test_environment(mut commands: Commands) {
	commands.spawn_entity(vec2(0.0, -50.0), SpawnKelp::new(1, LayerStyle::Behind));
	commands.spawn_entity(vec2(20.0, -50.0), SpawnKelp::new(2, LayerStyle::Behind));
	commands.spawn_entity(vec2(15.0, -50.0), SpawnKelp::new(3, LayerStyle::Front));
}

pub fn spawn_buildings(mut commands: Commands, library: Res<AssetLibrary>) {
	commands.spawn(
		(Sprite {
			anchor: Anchor::BottomCenter,
			image: library.image("building_1"),
			..default()
		}),
	);
}

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			OnEnter(GameState::MainMenu),
			const_transition_state::<{ GameState::InGame }>,
		)
		.add_systems(
			OnEnter(GameState::InGame),
			(spawn_test_player, spawn_test_environment),
		);
	}
}
