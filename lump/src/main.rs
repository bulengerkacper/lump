use bevy::prelude::*;
use std::{thread,time};

pub mod bash_connector;
pub mod event_handler;
use bash_connector::Cache;
use event_handler::EventHandler;

fn main() {
    let mut cache = Cache {
        content: String::from(""),
    };
    
    let _data_collection_thread = thread::spawn(move || {
        cache.collect_data();
        thread::sleep(time::Duration::from_millis(1000));
    });

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, EventHandler::handle_mouse_events)
        .add_systems(Update, EventHandler::handle_keyboard_events)
        .run();
}