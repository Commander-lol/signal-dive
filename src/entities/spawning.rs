use bevy::prelude::*;

mod environment;
pub use environment::SpawnKelp;

pub struct SpawnEntity {
	pub position: Vec2,
	pub command: Box<dyn CanSpawn>,
}

pub trait CanSpawn: 'static + Send + Sync {
	fn spawn(&self, position: Vec2, world: &mut World);
}

impl Command for SpawnEntity {
	fn apply(self, world: &mut World) {
		self.command.spawn(self.position, world);
	}
}

pub trait CanSpawnExt {
	fn spawn_entity(&mut self, position: Vec2, command: impl CanSpawn);
}

impl CanSpawnExt for Commands<'_, '_> {
	fn spawn_entity(&mut self, position: Vec2, command: impl CanSpawn) {
		self.queue(SpawnEntity {
			position,
			command: Box::new(command),
		})
	}
}
