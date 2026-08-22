use crate::domain::hotspot::{Hotspot1, Hotspot2};
use crate::domain::solved::{Solution1, Solved};
use crate::layout::{HotspotLayout, calculate_hotspot_layout};
use crate::state::ZoomState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct SolutionIndicator;

pub fn draw_hotspots(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    let window = windows
        .single()
        .expect("primary window should exist at startup");

    let layout = calculate_hotspot_layout(window.width(), window.height());

    draw_hotspot1(&mut commands, &layout);
    draw_hotspot2(&mut commands, &layout);
    draw_solution_indicator(&mut commands, &layout);
}

fn draw_hotspot1(commands: &mut Commands, layout: &HotspotLayout) {
    let light_blue_color = Color::srgb(0.55, 0.6, 0.9);

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
}

fn draw_hotspot2(commands: &mut Commands, layout: &HotspotLayout) {
    let light_red_color = Color::srgb(0.9, 0.6, 0.55);

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

fn draw_solution_indicator(commands: &mut Commands, layout: &HotspotLayout) {
    commands.spawn((
        SolutionIndicator,
        Sprite {
            color: Color::NONE,
            custom_size: Some(layout.size),
            ..default()
        },
        Transform::from_xyz(layout.top_right_x, layout.top_row_y, 0.0),
        Visibility::Hidden,
    ));
}

fn open_zoom1(_click: On<Pointer<Click>>, mut next_zoom: ResMut<NextState<ZoomState>>) {
    next_zoom.set(ZoomState::Zoom1);
}

fn open_zoom2(_click: On<Pointer<Click>>, mut next_zoom: ResMut<NextState<ZoomState>>) {
    next_zoom.set(ZoomState::Zoom2);
}

pub fn sync_solution_indicator(
    solved: Res<Solved>,
    mut indicators: Query<(&mut Sprite, &mut Visibility), With<SolutionIndicator>>,
) {
    if !solved.is_changed() {
        return;
    }

    let Ok((mut sprite, mut visibility)) = indicators.single_mut() else {
        return;
    };

    match solved.quiz1 {
        Solution1::Unsolved => {
            *visibility = Visibility::Hidden;
        }
        Solution1::Answer3 => {
            sprite.color = Color::srgb(0.2, 0.8, 0.2);
            *visibility = Visibility::Visible;
        }
        Solution1::Answer12 => {
            sprite.color = Color::srgb(0.6, 0.2, 0.8);
            *visibility = Visibility::Visible;
        }
    }
}
