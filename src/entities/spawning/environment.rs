use crate::entities::spawning::CanSpawn;
use crate::graphics::{LayerStyle, SwayingObjectSpawner};
use bevy::ecs::system::SystemState;
use bevy::math::Vec2;
use bevy::prelude::{Entity, World};
use num_traits::AsPrimitive;

pub struct SpawnKelp {
	pub variant: usize,
	pub layer: LayerStyle,
}

impl SpawnKelp {
	pub fn new(variant: usize, layer: LayerStyle) -> Self {
		Self {
			variant: variant.clamp(0, 6),
			layer,
		}
	}
}

impl CanSpawn for SpawnKelp {
	fn spawn(&self, position: Vec2, world: &mut World) {
		let mut state = SystemState::<SwayingObjectSpawner>::new(world);
		let mut spawner: SwayingObjectSpawner = state.get_mut(world);

		spawner.spawn_kelp(
			format!("kelp_{}", self.variant),
			position.extend(self.layer.as_()),
		);

		state.apply(world);
	}
}
