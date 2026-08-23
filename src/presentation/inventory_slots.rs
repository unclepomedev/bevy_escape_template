use crate::domain::item::{INVENTORY_CAPACITY, Inventory, ItemId};
use crate::domain::selection::{SelectedSlot, toggle_slot_selection};
use crate::layout::calculate_inventory_slot_layout;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct InventorySlotDisplay {
    pub index: usize,
}

#[derive(Component)]
pub struct InventorySlotLabel {
    index: usize,
}

pub fn spawn_inventory_slots(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    let window = windows
        .single()
        .expect("primary window should exist at startup");
    let layout =
        calculate_inventory_slot_layout(window.width(), window.height(), INVENTORY_CAPACITY);

    let empty_slot_color = Color::srgb(0.3, 0.3, 0.3);

    for (index, position) in layout.positions.iter().enumerate() {
        commands
            .spawn((
                InventorySlotDisplay { index },
                Sprite {
                    color: empty_slot_color,
                    custom_size: Some(layout.slot_size),
                    ..default()
                },
                Transform::from_xyz(position.x, position.y, 1.0),
                Pickable::default(),
            ))
            .observe(
                move |_click: On<Pointer<Click>>,
                      inventory: Res<Inventory>,
                      mut selected: ResMut<SelectedSlot>| {
                    toggle_slot_selection(index, &inventory, &mut selected);
                },
            );

        commands.spawn((
            InventorySlotLabel { index },
            Text2d::new(""),
            TextColor(Color::WHITE),
            TextFont {
                font_size: FontSize::Px(14.0),
                ..default()
            },
            Transform::from_xyz(position.x, position.y, 1.1),
        ));
    }
}

pub fn sync_inventory_slots(
    inventory: Res<Inventory>,
    selected: Res<SelectedSlot>,
    mut slot_sprites: Query<(&InventorySlotDisplay, &mut Sprite)>,
    mut slot_labels: Query<(&InventorySlotLabel, &mut Text2d)>,
) {
    if !inventory.is_changed() && !selected.is_changed() {
        return;
    }

    let empty_slot_color = Color::srgb(0.3, 0.3, 0.3);
    let filled_slot_color = Color::srgb(0.7, 0.7, 0.4);
    let selected_slot_color = Color::srgb(0.9, 0.9, 0.2);

    for (slot, mut sprite) in &mut slot_sprites {
        sprite.color = if selected.index == Some(slot.index) {
            selected_slot_color
        } else if inventory.slot(slot.index).is_some() {
            filled_slot_color
        } else {
            empty_slot_color
        };
    }

    for (label, mut text) in &mut slot_labels {
        text.0 = match inventory.slot(label.index) {
            Some(item) => item_label(*item),
            None => String::new(),
        };
    }
}

fn item_label(item: ItemId) -> String {
    match item {
        ItemId::Key1 => "key1".to_string(),
        ItemId::Key2 => "key2".to_string(),
        ItemId::Key3 => "key3".to_string(),
        ItemId::Key4 => "key4".to_string(),
    }
}
