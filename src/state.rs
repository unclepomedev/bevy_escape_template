use bevy::prelude::*;

#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Debug, Default)]
pub enum AppState {
    #[default]
    InGame,
}

/// Which screen is showing while in `AppState::InGame`.
#[derive(SubStates, Clone, Copy, Eq, PartialEq, Hash, Debug, Default)]
#[source(AppState = AppState::InGame)]
pub enum ZoomState {
    #[default]
    Overview,
    Zoom1,
    Zoom2,
}
