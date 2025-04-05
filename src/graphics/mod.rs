use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

mod background;

pub struct GraphicsPluginGroup;
impl PluginGroup for GraphicsPluginGroup {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>().add(background::BackgroundNoisePlugin)
	}
}
