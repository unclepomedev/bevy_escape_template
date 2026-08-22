use bevy_ecs::prelude::*;

/// A type-erased side effect that can be applied to the `World`.
///
/// Any type that implements `Message` automatically implements `Effect` via the blanket impl below.
/// Types that need to mutate a `Resource` directly can implement `Effect` by hand instead.
pub trait Effect: Send + Sync + 'static {
    fn apply(self: Box<Self>, world: &mut World);
}

/// Blanket impl: any Bevy `Message` is automatically usable as an `Effect`.
impl<M: Message> Effect for M {
    fn apply(self: Box<Self>, world: &mut World) {
        world.write_message(*self);
    }
}

/// Applies a list of effects to the world, in order.
pub fn apply_effects(effects: Vec<Box<dyn Effect>>, world: &mut World) {
    for effect in effects {
        effect.apply(world);
    }
}

// ============================================================================================
// UNIT TESTS
// ============================================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Message, Debug, PartialEq, Eq, Clone)]
    struct ShowMessageMsg {
        text: String,
    }

    #[derive(Resource, Default)]
    struct FlagStore {
        door_open: bool,
    }

    struct SetDoorOpen {
        value: bool,
    }

    impl Effect for SetDoorOpen {
        fn apply(self: Box<Self>, world: &mut World) {
            world.resource_mut::<FlagStore>().door_open = self.value;
        }
    }

    #[test]
    fn message_effect_is_written_to_queue() {
        let mut world = World::new();
        world.init_resource::<Messages<ShowMessageMsg>>();

        let effects: Vec<Box<dyn Effect>> = vec![Box::new(ShowMessageMsg {
            text: "1+2=?".to_string(),
        })];
        apply_effects(effects, &mut world);

        let messages = world.resource::<Messages<ShowMessageMsg>>();
        let mut cursor = messages.get_cursor();
        let received: Vec<_> = cursor.read(messages).cloned().collect();
        assert_eq!(
            received,
            vec![ShowMessageMsg {
                text: "1+2=?".to_string()
            }]
        );
    }

    #[test]
    fn custom_effect_mutates_resource_directly() {
        let mut world = World::new();
        world.init_resource::<FlagStore>();

        let effects: Vec<Box<dyn Effect>> = vec![Box::new(SetDoorOpen { value: true })];
        apply_effects(effects, &mut world);

        assert!(world.resource::<FlagStore>().door_open);
    }

    #[test]
    fn mixed_effects_in_one_list() {
        let mut world = World::new();
        world.init_resource::<FlagStore>();
        world.init_resource::<Messages<ShowMessageMsg>>();

        let effects: Vec<Box<dyn Effect>> = vec![
            Box::new(SetDoorOpen { value: true }),
            Box::new(ShowMessageMsg {
                text: "開いた".to_string(),
            }),
        ];
        apply_effects(effects, &mut world);

        assert!(world.resource::<FlagStore>().door_open);
        let messages = world.resource::<Messages<ShowMessageMsg>>();
        let mut cursor = messages.get_cursor();
        assert_eq!(cursor.read(messages).count(), 1);
    }
}
