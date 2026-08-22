pub mod zoom1;
pub mod zoom2;

use crate::state::ZoomState;
use bevy::prelude::*;

/// Marks any entity spawned as part of a zoom screen.
#[derive(Component)]
pub struct ZoomScreenElement;

pub fn despawn_zoom_screen(
    mut commands: Commands,
    zoom_elements: Query<Entity, With<ZoomScreenElement>>,
) {
    for entity in &zoom_elements {
        commands.entity(entity).despawn();
    }
}

pub fn close_zoom_screen(_click: On<Pointer<Click>>, mut next_zoom: ResMut<NextState<ZoomState>>) {
    next_zoom.set(ZoomState::Overview);
}
