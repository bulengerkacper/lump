use bevy::{input::keyboard::KeyboardInput, input::mouse::MouseButtonInput, prelude::*};

pub struct EventHandler {}

impl EventHandler {
    pub fn handle_mouse_events(
        mut mouse_button_input_events: EventReader<MouseButtonInput>,
        mut cursor_moved_events: EventReader<CursorMoved>,
    ) {
        for event in mouse_button_input_events.iter() {
            info!("{:?}", event);
        }

        for event in cursor_moved_events.iter() {
            info!("{:?}", event);
        }
    }

    pub fn handle_keyboard_events(mut keyboard_input_events: EventReader<KeyboardInput>) {
        for event in keyboard_input_events.iter() {
            info!("{:?}", event);
        }
    }
}
