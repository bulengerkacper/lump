use bevy::{prelude::*};

pub mod event_handler;
use event_handler::{handle_mouse_events,handle_keyboard_events};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, handle_mouse_events)
        .add_systems(Update, handle_keyboard_events)
        .run();
}
