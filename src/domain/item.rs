use bevy::prelude::*;
use bevy_escape_core::{
    GiveItem as CoreGiveItem, Inventory as CoreInventory,
    InventoryFullMessage as CoreInventoryFullMessage, RemoveItemAt as CoreRemoveItemAt,
};
use std::marker::PhantomData;

pub const INVENTORY_CAPACITY: usize = 4;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum ItemId {
    Key1,
    Key2,
    Key3,
    Key4,
}

pub type Inventory = CoreInventory<ItemId>;
pub type GiveItem = CoreGiveItem<ItemId>;
pub type InventoryFullMessage = CoreInventoryFullMessage<ItemId>;

pub fn remove_item_at(index: usize) -> CoreRemoveItemAt<ItemId> {
    CoreRemoveItemAt {
        index,
        item_type: PhantomData,
    }
}

pub fn log_inventory_full(mut full_messages: MessageReader<InventoryFullMessage>) {
    for message in full_messages.read() {
        info!("inventory is full, could not add {:?}", message.item);
    }
}
