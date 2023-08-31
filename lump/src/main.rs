use bash_connector::Cache;
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use futures::StreamExt;
use futures_channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use regex::Regex;
use std::cell::Cell;

use std::process::Command;

pub mod bash_connector;

fn main() {
    let (sender, receiver) = unbounded();
    let other = sender.clone();

    std::thread::spawn(move || loop {
        let _ = other.unbounded_send(perform_action());
    });

    let config = Config::new().with_window(
        WindowBuilder::default()
            .with_title("Lump - your linux task manager")
            .with_inner_size(dioxus_desktop::LogicalSize::new(600.0, 900.0)),
    );

    dioxus_desktop::launch_with_props(
        app,
        AppProps {
            sender: Cell::new(Some(sender)),
            receiver: Cell::new(Some(receiver)),
        },
        config,
    )
}

struct AppProps {
    sender: Cell<Option<UnboundedSender<Vec<(String, String)>>>>,
    receiver: Cell<Option<UnboundedReceiver<Vec<(String, String)>>>>,
}

pub fn perform_action() -> Vec<(String, String)> {
    let mut cache = Cache {
        content: Vec::new(),
    };

    if cache.collect_data() {
        return cache.content.clone();
    }
    let empty: Vec<(String, String)> = Vec::new();
    return empty;
}

fn app(cx: Scope<AppProps>) -> Element {
    let mut empty: Vec<(String, String)> = Vec::new();
    let output = use_state(cx, || empty);

    let _ = use_coroutine(cx, |_: UnboundedReceiver<()>| {
        let receiver = cx.props.receiver.take();
        let output = output.to_owned();
        async move {
            if let Some(mut receiver) = receiver {
                while let Some(msg) = receiver.next().await {
                    output.set(msg)
                }
            }
        }
    });

    cx.render(rsx! {
        link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/bootstrap@4.3.1/dist/css/bootstrap.min.css" },
        div {
            style: "  float:left; font-size: 0.875em; color:white;background-color:#999999;
            // padding:5px;
            // margin:5px;
            width: 300px;
            height: 920px;
            overflow-y: auto;
            overflow-x: hidden;
            text-align:justify;
            
            ",
            h3 { "List of all process"}
            for (index, (key, value)) in output.iter().enumerate() {
                rsx!("{key} {value} ")
                button {
                    //style: " color:white;background-color:#009900;",
                    onclick: move |event| {
                        Command::new("kill").arg("-9").arg(value).output().expect("Failed to execute command");
                    },
                    "kill me!"
                }
                br {}
            }
        }
        div {
            style: " width:300px;
            float:right;
            color:white;
            background-color:#999999;
            // padding:5px;
            // margin:5px;
            ",
            h3 { "Top consuming"}

        }
    })
}
