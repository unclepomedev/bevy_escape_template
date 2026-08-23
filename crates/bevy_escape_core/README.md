# bevy_escape_core

Type-erased effect application and a capacity-bounded inventory for Bevy escape games.

## How to use

### Applying effects

1. Implement `Effect` for anything that mutates the `World`, or use any `Message` (it gets `Effect` for free.)
2. Collect effects into `Vec<Box<dyn Effect>>`.
3. Call `apply_effects`.

```rust
use bevy::prelude::*;
use bevy_escape_core::{Effect, apply_effects};

#[derive(Resource, Default)]
struct DoorOpen(bool);

struct SetDoorOpen(bool);
impl Effect for SetDoorOpen {
    fn apply(self: Box<Self>, world: &mut World) {
        world.resource_mut::<DoorOpen>().0 = self.0;
    }
}

#[derive(Message)]
struct ShowMessage(String);

fn on_click(mut commands: Commands) {
    commands.queue(|world: &mut World| {
        let effects: Vec<Box<dyn Effect>> = vec![
            Box::new(SetDoorOpen(true)),
            Box::new(ShowMessage("opened".into())),
        ];
        apply_effects(effects, world);
    });
}
```

Register `Message` types with `app.add_message::<T>()` first — an unregistered message panics instead of silently dropping.

### Using the inventory

```rust
use bevy::prelude::*;
use bevy_escape_core::{Effect, GiveItem, Inventory, apply_effects};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum ItemId { Key }

fn setup(mut app: App) {
    app.insert_resource(Inventory::<ItemId>::new(4));
}

fn give_key(mut commands: Commands) {
    commands.queue(|world: &mut World| {
        let effects: Vec<Box<dyn Effect>> = vec![Box::new(GiveItem { item: ItemId::Key })];
        apply_effects(effects, world);
    });
}
```

Items fill the first empty slot in acquisition order. Removed slots stay empty (no compaction). Register `InventoryFullMessage<Item>` for full-inventory feedback.

## Examples

See `src/` in this repository for a real game using it.

## License

MIT or Apache-2.0
