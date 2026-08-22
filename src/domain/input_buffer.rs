use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct InputBuffer {
    pub text: String,
}

impl InputBuffer {
    pub fn clear(&mut self) {
        self.text.clear();
    }
}

pub fn reset_input_buffer(mut input_buffer: ResMut<InputBuffer>) {
    input_buffer.clear();
}
