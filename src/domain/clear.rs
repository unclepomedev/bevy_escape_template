use bevy::prelude::*;
use bevy_escape_core::Effect;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClearLabel {
    Clear1,
    Clear2,
}

#[derive(Resource, Default)]
pub struct GameClear {
    pub label: Option<ClearLabel>,
}

pub struct SetGameClear {
    pub label: ClearLabel,
}

impl Effect for SetGameClear {
    fn apply(self: Box<Self>, world: &mut World) {
        world.resource_mut::<GameClear>().label = Some(self.label);
    }
}
