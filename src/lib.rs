#![feature(adt_const_params)]

use avian2d::prelude::Gravity;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowResolution};

mod debug;
mod system;
mod entities;

#[bevy_main]
fn main() {
	launch(build_window());
}

pub fn launch(window: WindowPlugin) {
	let mut app = App::new();
	app.add_plugins((
		DefaultPlugins
			.set(window)
			.set(ImagePlugin::default_nearest()),
		debug::DebugPlugin,
		system::SystemPluginGroup,
		entities::EntityPluginGroup,
		avian2d::PhysicsPlugins::default(),
		// avian2d::debug_render::PhysicsDebugPlugin::default(),
	))
	.insert_resource(Gravity(Vec2::ZERO));

	app.run();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn build_window() -> WindowPlugin {
	WindowPlugin {
		primary_window: Some(Window {
			title: String::from("Signal Dive"),
			present_mode: PresentMode::AutoNoVsync,
			resolution: WindowResolution::new(1280., 720.),
			mode: WindowMode::Windowed,
			..default()
		}),
		..default()
	}
}
