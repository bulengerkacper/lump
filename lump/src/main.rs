use bevy::{input::mouse::MouseButtonInput, prelude::*};

pub mod eventHandler;
use eventHandler::print_mouse_events_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, handle_mouse_events)
        .run();
}
