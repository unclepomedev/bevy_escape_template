use super::{ZoomScreenElement, close_zoom_screen};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_zoom1_screen(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    let window = windows.single().expect("primary window should exist");

    let full_screen_size = Vec2::new(window.width(), window.height());
    let overlay_background_color = Color::srgba(0.05, 0.05, 0.05, 0.95);
    let background_layer = 10.0;
    let text_layer = 11.0;
    let hint_text = "1+2=?";

    // Full-window background. Clicking anywhere on it closes the zoom screen.
    commands
        .spawn((
            ZoomScreenElement,
            Sprite {
                color: overlay_background_color,
                custom_size: Some(full_screen_size),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, background_layer),
            Pickable::default(),
        ))
        .observe(close_zoom_screen);

    commands.spawn((
        ZoomScreenElement,
        Text2d::new(hint_text),
        Transform::from_xyz(0.0, 0.0, text_layer),
    ));
}
