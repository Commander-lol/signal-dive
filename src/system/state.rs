use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::marker::ConstParamTy;
use std::ops::Not;

#[derive(ConstParamTy, Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
#[allow(unused)]
pub enum GameState {
	#[default]
	Preload,
	Loading,
	Splash,
	MainMenu,
	LevelTransition,
	InGame,
}

pub fn is_window_focused(window: Single<&Window, With<PrimaryWindow>>) -> bool {
	window.focused
}

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
	fn build(&self, app: &mut App) {
		app.init_state::<GameState>()
			.enable_state_scoped_entities::<GameState>();
	}
}

pub fn const_transition_state<const NEXT_STATE: GameState>(
	mut state: ResMut<NextState<GameState>>,
) {
	log::debug!("const state transition: {NEXT_STATE:?}");
	state.set(NEXT_STATE)
}
