use bevy::prelude::*;

use crate::domain::hotspot::Hotspot2;
use crate::layout::HotspotLayout;
use crate::state::ZoomState;

pub fn draw_hotspot2(commands: &mut Commands, layout: &HotspotLayout) {
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

fn open_zoom2(_click: On<Pointer<Click>>, mut next_zoom: ResMut<NextState<ZoomState>>) {
    next_zoom.set(ZoomState::Zoom2);
}
