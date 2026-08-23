use bevy::prelude::*;
use bevy_escape_core::{Effect, apply_effects};

use crate::domain::hotspot::Hotspot6;
use crate::domain::item::{Inventory, ItemId, remove_item_at};
use crate::domain::selection::SelectedSlot;
use crate::layout::HotspotLayout;

pub fn draw_hotspot6(commands: &mut Commands, layout: &HotspotLayout) {
    let color = Color::srgb(0.5, 0.8, 0.8);

    commands
        .spawn((
            Hotspot6,
            Sprite {
                color,
                custom_size: Some(layout.size),
                ..default()
            },
            Transform::from_xyz(layout.top_right_x, layout.middle_row_y, 0.0),
            Pickable::default(),
        ))
        .observe(on_hotspot6_click);
}

fn on_hotspot6_click(
    _click: On<Pointer<Click>>,
    selected: Res<SelectedSlot>,
    inventory: Res<Inventory>,
    mut commands: Commands,
) {
    let Some(index) = selected.index else {
        return;
    };
    let Some(item) = inventory.slot(index) else {
        return;
    };
    if *item != ItemId::Key3 {
        return;
    }

    commands.queue(move |world: &mut World| {
        let effects: Vec<Box<dyn Effect>> = vec![Box::new(remove_item_at(index))];
        apply_effects(effects, world);
        world.resource_mut::<SelectedSlot>().index = None;
        info!("key3 was used on hotspot6");
    });
}
