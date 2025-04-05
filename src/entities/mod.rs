use bevy::app::{PluginGroup, PluginGroupBuilder};
use leafwing_input_manager::prelude::InputManagerPlugin;

mod input_handling;

pub use input_handling::{DirectionTracked, MovePower, PlayerControl, create_default_input_map};

pub struct EntityPluginGroup;
impl PluginGroup for EntityPluginGroup {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(input_handling::InputPlugin)
			.add(InputManagerPlugin::<PlayerControl>::default())
	}
}
