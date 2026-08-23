use bevy::prelude::*;
use bevy_escape_core::{Effect, apply_effects};

use crate::domain::hotspot::Hotspot4;
use crate::domain::item::{Inventory, ItemId, remove_item_at};
use crate::domain::progress::UnlockRect5;
use crate::domain::selection::SelectedSlot;
use crate::layout::HotspotLayout;

pub fn draw_hotspot4(commands: &mut Commands, layout: &HotspotLayout) {
    let color = Color::srgb(0.7, 0.7, 0.9);

    commands
        .spawn((
            Hotspot4,
            Sprite {
                color,
                custom_size: Some(layout.size),
                ..default()
            },
            Transform::from_xyz(layout.top_left_x, layout.middle_row_y, 0.0),
            Pickable::default(),
        ))
        .observe(on_hotspot4_click);
}

fn on_hotspot4_click(
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
    if *item != ItemId::Key1 {
        return;
    }

    commands.queue(move |world: &mut World| {
        let effects: Vec<Box<dyn Effect>> =
            vec![Box::new(remove_item_at(index)), Box::new(UnlockRect5)];
        apply_effects(effects, world);
        world.resource_mut::<SelectedSlot>().index = None;
    });
}
