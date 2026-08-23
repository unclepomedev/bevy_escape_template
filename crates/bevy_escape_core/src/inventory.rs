use crate::Effect;
use bevy_ecs::prelude::*;
use std::marker::PhantomData;

#[derive(Resource)]
pub struct Inventory<Item: Send + Sync + 'static> {
    slots: Vec<Option<Item>>,
}

impl<Item: Send + Sync + 'static> Inventory<Item> {
    pub fn new(capacity: usize) -> Self {
        let mut slots = Vec::with_capacity(capacity);
        slots.resize_with(capacity, || None);
        Self { slots }
    }

    pub fn capacity(&self) -> usize {
        self.slots.len()
    }

    pub fn slot(&self, index: usize) -> Option<&Item> {
        self.slots.get(index).and_then(|slot| slot.as_ref())
    }

    fn first_empty_slot(&self) -> Option<usize> {
        self.slots.iter().position(Option::is_none)
    }
}

impl<Item: PartialEq + Send + Sync + 'static> Inventory<Item> {
    pub fn has(&self, item: &Item) -> bool {
        self.slots.iter().any(|slot| slot.as_ref() == Some(item))
    }
}

pub struct GiveItem<Item> {
    pub item: Item,
}

impl<Item: Send + Sync + 'static> Effect for GiveItem<Item> {
    fn apply(self: Box<Self>, world: &mut World) {
        let GiveItem { item } = *self;

        let empty_slot = {
            let inventory = world.resource_mut::<Inventory<Item>>();
            inventory.first_empty_slot()
        };

        match empty_slot {
            Some(index) => {
                world.resource_mut::<Inventory<Item>>().slots[index] = Some(item);
            }
            None => {
                let message: Box<dyn Effect> = Box::new(InventoryFullMessage { item });
                message.apply(world);
            }
        }
    }
}

pub struct RemoveItemAt<Item> {
    pub index: usize,
    pub item_type: PhantomData<Item>,
}

impl<Item: Send + Sync + 'static> Effect for RemoveItemAt<Item> {
    fn apply(self: Box<Self>, world: &mut World) {
        world.resource_mut::<Inventory<Item>>().slots[self.index] = None;
    }
}

pub struct InventoryFullMessage<Item> {
    pub item: Item,
}

impl<Item: Send + Sync + 'static> Message for InventoryFullMessage<Item> {}

// ============================================================================================
// UNIT TESTS
// ============================================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply_effects;

    fn world_with_inventory(capacity: usize) -> World {
        let mut world = World::new();
        world.insert_resource(Inventory::<&'static str>::new(capacity));
        world.init_resource::<Messages<InventoryFullMessage<&'static str>>>();
        world
    }

    #[test]
    fn giving_an_item_fills_the_first_empty_slot() {
        let mut world = world_with_inventory(2);
        apply_effects(vec![Box::new(GiveItem { item: "key1" })], &mut world);

        let inventory = world.resource::<Inventory<&'static str>>();
        assert_eq!(inventory.slot(0), Some(&"key1"));
        assert_eq!(inventory.slot(1), None);
    }

    #[test]
    fn items_fill_slots_in_acquisition_order() {
        let mut world = world_with_inventory(2);
        apply_effects(vec![Box::new(GiveItem { item: "key1" })], &mut world);
        apply_effects(vec![Box::new(GiveItem { item: "key2" })], &mut world);

        let inventory = world.resource::<Inventory<&'static str>>();
        assert_eq!(inventory.slot(0), Some(&"key1"));
        assert_eq!(inventory.slot(1), Some(&"key2"));
    }

    #[test]
    fn giving_an_item_when_full_emits_inventory_full_message() {
        let mut world = world_with_inventory(1);
        apply_effects(vec![Box::new(GiveItem { item: "key1" })], &mut world);
        apply_effects(vec![Box::new(GiveItem { item: "key2" })], &mut world);

        let inventory = world.resource::<Inventory<&'static str>>();
        assert_eq!(inventory.slot(0), Some(&"key1"));

        let messages = world.resource::<Messages<InventoryFullMessage<&'static str>>>();
        let mut cursor = messages.get_cursor();
        let received: Vec<_> = cursor.read(messages).collect();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].item, "key2");
    }

    #[test]
    fn removing_an_item_leaves_a_gap_at_that_slot() {
        let mut world = world_with_inventory(2);
        apply_effects(vec![Box::new(GiveItem { item: "key1" })], &mut world);
        apply_effects(vec![Box::new(GiveItem { item: "key2" })], &mut world);
        apply_effects(
            vec![Box::new(RemoveItemAt::<&'static str> {
                index: 0,
                item_type: PhantomData,
            })],
            &mut world,
        );

        let inventory = world.resource::<Inventory<&'static str>>();
        assert_eq!(inventory.slot(0), None);
        assert_eq!(inventory.slot(1), Some(&"key2"));
    }

    #[test]
    fn removing_a_slot_frees_it_up_for_a_new_item() {
        let mut world = world_with_inventory(2);
        apply_effects(vec![Box::new(GiveItem { item: "key1" })], &mut world);
        apply_effects(vec![Box::new(GiveItem { item: "key2" })], &mut world);
        apply_effects(
            vec![Box::new(RemoveItemAt::<&'static str> {
                index: 0,
                item_type: PhantomData,
            })],
            &mut world,
        );
        apply_effects(vec![Box::new(GiveItem { item: "key3" })], &mut world);

        let inventory = world.resource::<Inventory<&'static str>>();
        assert_eq!(inventory.slot(0), Some(&"key3"));
        assert_eq!(inventory.slot(1), Some(&"key2"));
    }

    #[test]
    fn has_finds_an_item_regardless_of_slot() {
        let mut world = world_with_inventory(2);
        apply_effects(vec![Box::new(GiveItem { item: "key1" })], &mut world);

        let inventory = world.resource::<Inventory<&'static str>>();
        assert!(inventory.has(&"key1"));
        assert!(!inventory.has(&"key2"));
    }
}
