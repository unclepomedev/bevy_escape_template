use bevy_ecs::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;

use crate::Effect;

#[derive(Resource)]
pub struct Inventory<Item: Eq + Hash + Send + Sync + 'static> {
    items: HashSet<Item>,
}

impl<Item: Eq + Hash + Send + Sync + 'static> Default for Inventory<Item> {
    fn default() -> Self {
        Self {
            items: HashSet::new(),
        }
    }
}

impl<Item: Eq + Hash + Send + Sync + 'static> Inventory<Item> {
    pub fn has(&self, item: &Item) -> bool {
        self.items.contains(item)
    }
}

pub struct GiveItem<Item> {
    pub item: Item,
}

impl<Item: Eq + Hash + Send + Sync + 'static> Effect for GiveItem<Item> {
    fn apply(self: Box<Self>, world: &mut World) {
        let GiveItem { item } = *self;
        world.resource_mut::<Inventory<Item>>().items.insert(item);
    }
}

pub struct RemoveItem<Item> {
    pub item: Item,
}

impl<Item: Eq + Hash + Send + Sync + 'static> Effect for RemoveItem<Item> {
    fn apply(self: Box<Self>, world: &mut World) {
        let RemoveItem { item } = *self;
        world.resource_mut::<Inventory<Item>>().items.remove(&item);
    }
}

// ============================================================================================
// UNIT TESTS
// ============================================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply_effects;

    fn world_with_inventory() -> World {
        let mut world = World::new();
        world.init_resource::<Inventory<&'static str>>();
        world
    }

    #[test]
    fn giving_an_item_makes_has_true() {
        let mut world = world_with_inventory();
        let effects: Vec<Box<dyn Effect>> = vec![Box::new(GiveItem { item: "key" })];
        apply_effects(effects, &mut world);
        assert!(world.resource::<Inventory<&'static str>>().has(&"key"));
    }

    #[test]
    fn removing_an_item_makes_has_false() {
        let mut world = world_with_inventory();
        apply_effects(vec![Box::new(GiveItem { item: "key" })], &mut world);
        apply_effects(vec![Box::new(RemoveItem { item: "key" })], &mut world);
        assert!(!world.resource::<Inventory<&'static str>>().has(&"key"));
    }

    #[test]
    fn an_item_never_given_is_not_present() {
        let world = world_with_inventory();
        assert!(!world.resource::<Inventory<&'static str>>().has(&"key"));
    }
}
