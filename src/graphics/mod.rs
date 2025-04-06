use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use num_traits::AsPrimitive;

mod background;
mod swaying_material;

pub use swaying_material::{SwayingMaterial, SwayingObjectSpawner};

pub const LAYER_BACKGROUND: f32 = 1.0;
pub const LAYER_ENTITIES: f32 = 200.0;

pub const LAYER_BEHIND: f32 = LAYER_ENTITIES - 100.0;
pub const LAYER_FRONT: f32 = LAYER_ENTITIES + 100.0;

#[derive(Copy, Clone)]
pub enum LayerStyle {
	Behind,
	Entity,
	Front,
}

impl AsPrimitive<f32> for LayerStyle {
	fn as_(self) -> f32 {
		match self {
			LayerStyle::Behind => LAYER_BEHIND,
			LayerStyle::Entity => LAYER_ENTITIES,
			LayerStyle::Front => LAYER_FRONT,
		}
	}
}

pub struct GraphicsPluginGroup;
impl PluginGroup for GraphicsPluginGroup {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(background::BackgroundNoisePlugin)
			.add(swaying_material::SwayingMaterialPlugin)
	}
}
