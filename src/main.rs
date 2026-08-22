mod domain;
mod layout;
mod presentation;
mod state;

use bevy::prelude::*;
use presentation::overview::draw_hotspots;
use presentation::zoom::despawn_zoom_screen;
use presentation::zoom::zoom1::spawn_zoom1_screen;
use presentation::zoom::zoom2::spawn_zoom2_screen;
use state::{AppState, ZoomState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_sub_state::<ZoomState>()
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, draw_hotspots)
        .add_systems(OnEnter(ZoomState::Zoom1), spawn_zoom1_screen)
        .add_systems(OnExit(ZoomState::Zoom1), despawn_zoom_screen)
        .add_systems(OnEnter(ZoomState::Zoom2), spawn_zoom2_screen)
        .add_systems(OnExit(ZoomState::Zoom2), despawn_zoom_screen)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
