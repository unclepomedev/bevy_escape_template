use super::{ZoomScreenElement, close_zoom_screen};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_zoom2_screen(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    let window = windows.single().expect("primary window should exist");

    spawn_background(&mut commands, window);
    spawn_input_field(&mut commands);
    spawn_confirm_button(&mut commands);
}

fn spawn_background(commands: &mut Commands, window: &Window) {
    let full_screen_size = Vec2::new(window.width(), window.height());
    let overlay_background_color = Color::srgba(0.05, 0.05, 0.05, 0.95);
    let background_layer = 10.0;

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
}

fn spawn_input_field(commands: &mut Commands) {
    let input_field_color = Color::srgb(0.9, 0.9, 0.9);
    let input_field_size = Vec2::new(240.0, 60.0);
    let input_field_layer = 11.0;
    let input_field_y = 40.0;
    let input_placeholder_text = "";

    commands.spawn((
        ZoomScreenElement,
        Sprite {
            color: input_field_color,
            custom_size: Some(input_field_size),
            ..default()
        },
        Transform::from_xyz(0.0, input_field_y, input_field_layer),
    ));
    commands.spawn((
        ZoomScreenElement,
        Text2d::new(input_placeholder_text),
        TextColor(Color::BLACK),
        Transform::from_xyz(0.0, input_field_y, input_field_layer + 0.1),
    ));
}

fn spawn_confirm_button(commands: &mut Commands) {
    let confirm_button_color = Color::srgb(0.4, 0.7, 0.4);
    let confirm_button_size = Vec2::new(120.0, 48.0);
    let confirm_button_layer = 11.0;
    let confirm_button_y = -60.0;
    let confirm_button_label = "confirm";

    commands
        .spawn((
            ZoomScreenElement,
            Sprite {
                color: confirm_button_color,
                custom_size: Some(confirm_button_size),
                ..default()
            },
            Transform::from_xyz(0.0, confirm_button_y, confirm_button_layer),
            Pickable::default(),
        ))
        .observe(|_click: On<Pointer<Click>>| {
            info!("confirm button clicked (no logic wired up yet)");
        });
    commands.spawn((
        ZoomScreenElement,
        Text2d::new(confirm_button_label),
        Transform::from_xyz(0.0, confirm_button_y, confirm_button_layer + 0.1),
    ));
}
