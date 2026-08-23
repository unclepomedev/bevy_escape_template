use bevy::prelude::*;
use bevy_escape_core::{
    GiveItem as CoreGiveItem, Inventory as CoreInventory,
    InventoryFullMessage as CoreInventoryFullMessage, RemoveItemAt as CoreRemoveItemAt,
};

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
pub type RemoveItemAt = CoreRemoveItemAt<ItemId>;

pub fn remove_item_at(index: usize) -> RemoveItemAt {
    bevy_escape_core::remove_item_at::<ItemId>(index)
}

pub fn log_inventory_full(mut full_messages: MessageReader<InventoryFullMessage>) {
    for message in full_messages.read() {
        info!("inventory is full, could not add {:?}", message.item);
    }
}
