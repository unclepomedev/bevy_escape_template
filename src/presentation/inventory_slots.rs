use crate::domain::item::{GiveItem, INVENTORY_CAPACITY, Inventory, ItemId, remove_item_at};
use crate::domain::selection::{SelectedSlot, SlotClickOutcome, resolve_slot_click};
use crate::layout::calculate_inventory_slot_layout;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_escape_core::{Effect, apply_effects};

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
                      selected: Res<SelectedSlot>,
                      mut commands: Commands| {
                    handle_slot_click(index, &inventory, &selected, &mut commands);
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

fn handle_slot_click(
    clicked_index: usize,
    inventory: &Inventory,
    selected: &SelectedSlot,
    commands: &mut Commands,
) {
    match resolve_slot_click(clicked_index, inventory, selected) {
        SlotClickOutcome::NoOp => {}
        SlotClickOutcome::Deselect => {
            commands.queue(|world: &mut World| {
                world.resource_mut::<SelectedSlot>().index = None;
            });
        }
        SlotClickOutcome::Select { index } => {
            commands.queue(move |world: &mut World| {
                world.resource_mut::<SelectedSlot>().index = Some(index);
            });
        }
        SlotClickOutcome::Combine {
            selected_index,
            clicked_index,
            result,
        } => {
            commands.queue(move |world: &mut World| {
                let effects: Vec<Box<dyn Effect>> = vec![
                    Box::new(remove_item_at(selected_index)),
                    Box::new(remove_item_at(clicked_index)),
                    Box::new(GiveItem { item: result }),
                ];
                apply_effects(effects, world);
                world.resource_mut::<SelectedSlot>().index = None;
            });
        }
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
        ItemId::Key5 => "key5".to_string(),
    }
}
