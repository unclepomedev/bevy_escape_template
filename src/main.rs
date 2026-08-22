use bevy::prelude::*;

mod domain;
mod layout;
mod presentation;

use presentation::view::draw_hotspots;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, draw_hotspots)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
