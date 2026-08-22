use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::{Hotspot1, Hotspot2};

pub fn draw_hotspots(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    let window = windows
        .single()
        .expect("primary window should exist at startup");

    let (hotspot_size, top_row_y, top_left_x, top_center_x) =
        calculate_hotspot_layout(window.width(), window.height());

    let light_blue_color = Color::srgb(0.55, 0.6, 0.9);
    let light_red_color = Color::srgb(0.9, 0.6, 0.55);

    commands.spawn((
        Hotspot1,
        Sprite {
            color: light_blue_color,
            custom_size: Some(hotspot_size),
            ..default()
        },
        Transform::from_xyz(top_left_x, top_row_y, 0.0),
    ));

    commands.spawn((
        Hotspot2,
        Sprite {
            color: light_red_color,
            custom_size: Some(hotspot_size),
            ..default()
        },
        Transform::from_xyz(top_center_x, top_row_y, 0.0),
    ));
}

fn calculate_hotspot_layout(window_width: f32, window_height: f32) -> (Vec2, f32, f32, f32) {
    let cell_width = window_width / 3.0;
    let cell_height = window_height / 3.0;
    let hotspot_size = Vec2::new(cell_width * 0.85, cell_height * 0.85);

    let top_row_y = window_height / 2.0 - cell_height / 2.0;
    let top_left_x = -window_width / 2.0 + cell_width / 2.0;
    let top_center_x = 0.0;

    (hotspot_size, top_row_y, top_left_x, top_center_x)
}
