use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowResolution};

#[bevy_main]
fn main() {
    launch(build_window());
}

pub fn launch(window: WindowPlugin) {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(window),
    ));

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
