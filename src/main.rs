mod domain;
mod input;
mod layout;
mod presentation;
mod state;

use crate::domain::input_buffer::{InputBuffer, reset_input_buffer};
use crate::input::typing::append_typed_digits;
use crate::presentation::{
    overview::draw_hotspots,
    zoom::despawn_zoom_screen,
    zoom::zoom1::spawn_zoom1_screen,
    zoom::zoom2::{spawn_zoom2_screen, sync_input_field_display},
};
use crate::state::{AppState, ZoomState};
use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.init_state::<AppState>().add_sub_state::<ZoomState>();
    app.init_resource::<InputBuffer>();

    app.add_systems(Startup, (setup_camera, draw_hotspots))
        .add_systems(OnEnter(ZoomState::Zoom1), spawn_zoom1_screen)
        .add_systems(OnExit(ZoomState::Zoom1), despawn_zoom_screen)
        .add_systems(
            OnEnter(ZoomState::Zoom2),
            (reset_input_buffer, spawn_zoom2_screen).chain(),
        )
        .add_systems(OnExit(ZoomState::Zoom2), despawn_zoom_screen)
        .add_systems(
            Update,
            (append_typed_digits, sync_input_field_display)
                .chain()
                .run_if(in_state(ZoomState::Zoom2)),
        );
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
