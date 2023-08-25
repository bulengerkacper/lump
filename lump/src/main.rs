use bevy::prelude::*;

pub mod bash_connector;
pub mod event_handler;
use bash_connector::collect_data;
use event_handler::EventHandler;
//watch -n 1 pstree
fn main() {
    collect_data();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, EventHandler::handle_mouse_events)
        .add_systems(Update, EventHandler::handle_keyboard_events)
        // .add_systems(Update,collect_data)
        .run();
}
