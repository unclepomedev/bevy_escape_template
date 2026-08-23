use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::domain::clear::{ClearLabel, GameClear};

#[derive(Component)]
pub struct GameClearOverlay;

#[derive(Component)]
pub struct GameClearText;

pub fn spawn_game_clear_screen(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows
        .single()
        .expect("primary window should exist at startup");
    let size = Vec2::new(window.width(), window.height());

    commands.spawn((
        GameClearOverlay,
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 0.9),
            custom_size: Some(size),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 20.0),
        Visibility::Hidden,
    ));

    commands.spawn((
        GameClearText,
        Text2d::new(""),
        Transform::from_xyz(0.0, 0.0, 21.0),
    ));
}

pub fn sync_game_clear_screen(
    game_clear: Res<GameClear>,
    mut overlays: Query<&mut Visibility, With<GameClearOverlay>>,
    mut texts: Query<&mut Text2d, With<GameClearText>>,
) {
    if !game_clear.is_changed() {
        return;
    }

    let Some(label) = game_clear.label else {
        return;
    };

    if let Ok(mut visibility) = overlays.single_mut() {
        *visibility = Visibility::Visible;
    }

    if let Ok(mut text) = texts.single_mut() {
        text.0 = match label {
            ClearLabel::Clear1 => "clear1".to_string(),
            ClearLabel::Clear2 => "clear2".to_string(),
        };
    }
}
