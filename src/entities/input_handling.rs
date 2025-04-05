use avian2d::prelude::{
	AngularVelocity, ExternalForce, ExternalTorque, LinearVelocity, MaxLinearSpeed,
};
use bevy::prelude::*;
use bevy_enoki::ParticleEffectHandle;
use bevy_enoki::prelude::ParticleEffectInstance;
use leafwing_input_manager::prelude::*;
use std::ops::Neg;

#[derive(Copy, Clone, Debug, Default, Component, Deref, DerefMut, Reflect)]
pub struct MovePower(pub f32);

#[derive(Component)]
pub struct DirectionTracked;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerControl {
	#[actionlike(DualAxis)]
	Movement,
	Burst,
	Interact,
}

pub fn create_default_input_map() -> InputMap<PlayerControl> {
	InputMap::default()
		.with_dual_axis(PlayerControl::Movement, GamepadStick::LEFT)
		.with_dual_axis(PlayerControl::Movement, VirtualDPad::wasd())
		.with(PlayerControl::Interact, KeyCode::KeyE)
		.with(PlayerControl::Interact, GamepadButton::North)
		.with(PlayerControl::Burst, KeyCode::Space)
		.with(PlayerControl::Burst, GamepadButton::South)
}

pub fn process_input(
	mut query: Query<(&ActionState<PlayerControl>, &MovePower, &mut LinearVelocity)>,
) {
	for (action_state, move_speed, mut velocity) in &mut query {
		let move_vector = action_state
			.clamped_axis_pair(&PlayerControl::Movement)
			.normalize_or_zero();

		if move_vector != Vec2::ZERO {
			**velocity += move_vector * (**move_speed / Vec2::splat(15.0));
		}
	}
}

pub fn rotate_velocity_direction(
	parent_query: Query<(&LinearVelocity, &Children), With<DirectionTracked>>,
	mut sprite_query: Query<&mut Sprite>,
) {
	for (velocity, children) in &parent_query {
		for child in children {
			if let Ok(mut sprite) = sprite_query.get_mut(*child) {
				if velocity.x < 0.0 {
					sprite.flip_x = true;
				} else if velocity.x > 0.0 {
					sprite.flip_x = false;
				}
			}
		}
	}
}
pub fn flip_particle_emitter_temp(
	parent_query: Query<(&LinearVelocity, &MaxLinearSpeed, &Children), With<DirectionTracked>>,
	mut emitter_query: Query<(&mut Transform, &mut ParticleEffectInstance)>,
) {
	for (velocity, max_speed, children) in &parent_query {
		for child in children {
			if let Ok((mut emitter, mut particles)) = emitter_query.get_mut(*child) {
				// Medium gross direction hack. Like, damn
				let direction_sign = if velocity.x < 0.0 {
					-1.0
				} else if velocity.x > 0.0 {
					1.0
				} else {
					if let Some(p) = &mut particles.0 {
						p.spawn_amount = 0;
					}
					continue;
				};

				let speed_percent = velocity.length() / max_speed.0;
				let spawn_amount = (4.0 * speed_percent).floor() as u32;

				match particles.0.as_mut() {
					Some(value) => {
						value.spawn_amount = spawn_amount;
						if let Some(direction) = &mut value.direction {
							direction.0.y = direction.0.y.copysign(direction_sign); // Emitter direction is flipped vs sprite
						}
					}
					_ => {}
				}

				emitter.translation.y = emitter.translation.y.copysign(direction_sign);
			}
		}
	}
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			(
				process_input,
				rotate_velocity_direction,
				flip_particle_emitter_temp,
			),
		);
	}
}
