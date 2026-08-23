mod hotspot1;
mod hotspot2;
mod hotspot4;
mod hotspot5;
mod hotspot6;
mod solution_indicator;

pub use hotspot4::log_wrong_item_on_hotspot4;
pub use hotspot5::sync_hotspot5_visibility;
pub use solution_indicator::sync_solution_indicator;

use self::{
    hotspot1::draw_hotspot1, hotspot2::draw_hotspot2, hotspot4::draw_hotspot4,
    hotspot5::draw_hotspot5, hotspot6::draw_hotspot6, solution_indicator::draw_solution_indicator,
};
use crate::layout::calculate_hotspot_layout;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn draw_hotspots(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    let window = windows
        .single()
        .expect("primary window should exist at startup");

    let layout = calculate_hotspot_layout(window.width(), window.height());

    draw_hotspot1(&mut commands, &layout);
    draw_hotspot2(&mut commands, &layout);
    draw_solution_indicator(&mut commands, &layout);
    draw_hotspot4(&mut commands, &layout);
    draw_hotspot5(&mut commands, &layout);
    draw_hotspot6(&mut commands, &layout);
}
