use bevy::prelude::*;
use bevy_escape_core::{Effect, apply_effects};

use crate::domain::clear::{ClearLabel, SetGameClear};
use crate::domain::hotspot::Hotspot1;
use crate::domain::item::{Inventory, ItemId};
use crate::domain::selection::SelectedSlot;
use crate::layout::HotspotLayout;
use crate::state::ZoomState;

pub fn draw_hotspot1(commands: &mut Commands, layout: &HotspotLayout) {
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
        .observe(on_hotspot1_click);
}

fn on_hotspot1_click(
    _click: On<Pointer<Click>>,
    selected: Res<SelectedSlot>,
    inventory: Res<Inventory>,
    mut next_zoom: ResMut<NextState<ZoomState>>,
    mut commands: Commands,
) {
    let selected_item = selected
        .index
        .and_then(|index| inventory.slot(index).copied());

    match selected_item {
        Some(ItemId::Key2) => set_game_clear(&mut commands, ClearLabel::Clear2),
        Some(ItemId::Key4) => set_game_clear(&mut commands, ClearLabel::Clear1),
        _ => {
            next_zoom.set(ZoomState::Zoom1);
        }
    }
}

fn set_game_clear(commands: &mut Commands, label: ClearLabel) {
    commands.queue(move |world: &mut World| {
        let effects: Vec<Box<dyn Effect>> = vec![Box::new(SetGameClear { label })];
        apply_effects(effects, world);
    });
}
