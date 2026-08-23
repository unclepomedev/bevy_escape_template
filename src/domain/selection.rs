use crate::domain::item::{Inventory, ItemId};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SelectedSlot {
    pub index: Option<usize>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SlotClickOutcome {
    NoOp,
    Deselect,
    Select {
        index: usize,
    },
    Combine {
        selected_index: usize,
        clicked_index: usize,
        result: ItemId,
    },
}

pub fn resolve_slot_click(
    clicked_index: usize,
    inventory: &Inventory,
    selected: &SelectedSlot,
) -> SlotClickOutcome {
    let Some(clicked_item) = inventory.slot(clicked_index).copied() else {
        return SlotClickOutcome::NoOp;
    };

    match selected.index {
        Some(selected_index) if selected_index == clicked_index => SlotClickOutcome::Deselect,
        Some(selected_index) => {
            let selected_item = inventory.slot(selected_index).copied();
            match (selected_item, clicked_item) {
                (Some(ItemId::Key3), ItemId::Key3) => SlotClickOutcome::Combine {
                    selected_index,
                    clicked_index,
                    result: ItemId::Key4,
                },
                _ => SlotClickOutcome::Select {
                    index: clicked_index,
                },
            }
        }
        None => SlotClickOutcome::Select {
            index: clicked_index,
        },
    }
}

// ============================================================================================
// UNIT TESTS
// ============================================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::item::GiveItem;
    use bevy_escape_core::apply_effects;

    fn inventory_with(items: &[ItemId]) -> Inventory {
        let mut world = World::new();
        world.insert_resource(Inventory::new(4));
        for item in items {
            apply_effects(vec![Box::new(GiveItem { item: *item })], &mut world);
        }
        world.remove_resource::<Inventory>().unwrap()
    }

    #[test]
    fn clicking_an_empty_slot_is_a_no_op() {
        let inventory = inventory_with(&[]);
        let selected = SelectedSlot::default();
        assert_eq!(
            resolve_slot_click(0, &inventory, &selected),
            SlotClickOutcome::NoOp
        );
    }

    #[test]
    fn clicking_an_occupied_slot_with_nothing_selected_selects_it() {
        let inventory = inventory_with(&[ItemId::Key1]);
        let selected = SelectedSlot::default();
        assert_eq!(
            resolve_slot_click(0, &inventory, &selected),
            SlotClickOutcome::Select { index: 0 }
        );
    }

    #[test]
    fn clicking_the_selected_slot_again_deselects_it() {
        let inventory = inventory_with(&[ItemId::Key1]);
        let selected = SelectedSlot { index: Some(0) };
        assert_eq!(
            resolve_slot_click(0, &inventory, &selected),
            SlotClickOutcome::Deselect
        );
    }

    #[test]
    fn clicking_a_non_matching_item_switches_selection() {
        let inventory = inventory_with(&[ItemId::Key1, ItemId::Key2]);
        let selected = SelectedSlot { index: Some(0) };
        assert_eq!(
            resolve_slot_click(1, &inventory, &selected),
            SlotClickOutcome::Select { index: 1 }
        );
    }

    #[test]
    fn clicking_another_key3_while_a_key3_is_selected_combines_them() {
        let inventory = inventory_with(&[ItemId::Key3, ItemId::Key3]);
        let selected = SelectedSlot { index: Some(0) };
        assert_eq!(
            resolve_slot_click(1, &inventory, &selected),
            SlotClickOutcome::Combine {
                selected_index: 0,
                clicked_index: 1,
                result: ItemId::Key4,
            }
        );
    }
}
