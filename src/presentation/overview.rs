use crate::domain::hotspot::{Hotspot1, Hotspot2};
use crate::layout::calculate_hotspot_layout;
use crate::state::ZoomState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn draw_hotspots(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    let window = windows
        .single()
        .expect("primary window should exist at startup");

    let layout = calculate_hotspot_layout(window.width(), window.height());

    let light_blue_color = Color::srgb(0.55, 0.6, 0.9);
    let light_red_color = Color::srgb(0.9, 0.6, 0.55);

    commands
        .spawn((
            Hotspot1,
            Sprite {
                color: light_blue_color,
                custom_size: Some(layout.size),
                ..default()
            },
            Transform::from_xyz(layout.top_left_x, layout.top_row_y, 0.0),
            Pickable::default(),
        ))
        .observe(open_zoom1);

    commands
        .spawn((
            Hotspot2,
            Sprite {
                color: light_red_color,
                custom_size: Some(layout.size),
                ..default()
            },
            Transform::from_xyz(layout.top_center_x, layout.top_row_y, 0.0),
            Pickable::default(),
        ))
        .observe(open_zoom2);
}

fn open_zoom1(_click: On<Pointer<Click>>, mut next_zoom: ResMut<NextState<ZoomState>>) {
    next_zoom.set(ZoomState::Zoom1);
}

fn open_zoom2(_click: On<Pointer<Click>>, mut next_zoom: ResMut<NextState<ZoomState>>) {
    next_zoom.set(ZoomState::Zoom2);
}
