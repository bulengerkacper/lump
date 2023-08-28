use dioxus::prelude::*;
use std::{thread, time};

pub mod bash_connector;
pub mod gui;
use gui::Gui;
//pub mod event_handler;
use bash_connector::Cache;
//use event_handler::EventHandler;

fn main() {
    let mut cache = Cache {
        content: String::from(""),
    };

    let _data_collection_thread = thread::spawn(move || {
        cache.collect_data();
        thread::sleep(time::Duration::from_millis(1000));
    });

    dioxus_desktop::launch(Gui::app);
}
