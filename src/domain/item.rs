use bevy_escape_core::{
    GiveItem as CoreGiveItem, Inventory as CoreInventory, RemoveItem as CoreRemoveItem,
};

#[expect(dead_code)]
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum ItemId {
    Key,
}

#[expect(dead_code)]
pub type Inventory = CoreInventory<ItemId>;
#[expect(dead_code)]
pub type GiveItem = CoreGiveItem<ItemId>;
#[expect(dead_code)]
pub type RemoveItem = CoreRemoveItem<ItemId>;
