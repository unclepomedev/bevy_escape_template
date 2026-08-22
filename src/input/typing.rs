use crate::domain::input_buffer::InputBuffer;
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

pub fn append_typed_digits(
    mut keyboard_events: MessageReader<KeyboardInput>,
    mut input_buffer: ResMut<InputBuffer>,
) {
    for event in keyboard_events.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }

        match &event.logical_key {
            Key::Backspace => {
                input_buffer.text.pop();
            }
            Key::Character(characters) => {
                for character in characters.chars() {
                    if character.is_ascii_digit() {
                        input_buffer.text.push(character);
                    }
                }
            }
            _ => {}
        }
    }
}
