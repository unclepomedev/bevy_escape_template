use bevy_ecs::prelude::*;

pub struct WrongActionMessage<Detail> {
    pub detail: Detail,
}

impl<Detail: Send + Sync + 'static> Message for WrongActionMessage<Detail> {}
