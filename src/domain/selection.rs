use crate::domain::item::Inventory;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SelectedSlot {
    pub index: Option<usize>,
}

pub fn toggle_slot_selection(index: usize, inventory: &Inventory, selected: &mut SelectedSlot) {
    if inventory.slot(index).is_none() {
        return;
    }

    selected.index = if selected.index == Some(index) {
        None
    } else {
        Some(index)
    };
}

// ============================================================================================
// UNIT TESTS
// ============================================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::item::{GiveItem, ItemId};
    use bevy_escape_core::apply_effects;

    fn inventory_with_one_item() -> Inventory {
        let mut world = World::new();
        world.insert_resource(Inventory::new(4));
        apply_effects(vec![Box::new(GiveItem { item: ItemId::Key1 })], &mut world);
        world.remove_resource::<Inventory>().unwrap()
    }

    #[test]
    fn clicking_an_occupied_slot_selects_it() {
        let inventory = inventory_with_one_item();
        let mut selected = SelectedSlot::default();
        toggle_slot_selection(0, &inventory, &mut selected);
        assert_eq!(selected.index, Some(0));
    }

    #[test]
    fn clicking_the_selected_slot_again_deselects_it() {
        let inventory = inventory_with_one_item();
        let mut selected = SelectedSlot { index: Some(0) };
        toggle_slot_selection(0, &inventory, &mut selected);
        assert_eq!(selected.index, None);
    }

    #[test]
    fn clicking_an_empty_slot_does_nothing() {
        let inventory = inventory_with_one_item();
        let mut selected = SelectedSlot::default();
        toggle_slot_selection(1, &inventory, &mut selected);
        assert_eq!(selected.index, None);
    }
}
