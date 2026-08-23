use bevy::prelude::*;
use bevy_escape_core::Effect;

#[derive(Resource, Default)]
pub struct Progress {
    pub rect5_unlocked: bool,
}

pub struct UnlockRect5;

impl Effect for UnlockRect5 {
    fn apply(self: Box<Self>, world: &mut World) {
        world.resource_mut::<Progress>().rect5_unlocked = true;
    }
}
