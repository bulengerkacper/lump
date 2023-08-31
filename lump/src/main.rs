use bash_connector::Cache;
use dioxus::prelude::*;
use dioxus_desktop::Config;
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

    dioxus_desktop::launch_with_props(
        app,
        AppProps {
            sender: Cell::new(Some(sender)),
            receiver: Cell::new(Some(receiver)),
        },
        Config::default(),
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
//linie 56 i 62 do przepisania
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
        for (index, (key, value)) in output.iter().enumerate() {
            rsx!("{key} {value} ")
            button {
                onclick: move |event| 
                {
                    Command::new("kill").arg("-9").arg(value).output().expect("Failed to execute command");

                },
                "kill me!"
            }
            // button {
            //     style: " font-size: 9px;",
            //     onclick: move |_| Command::new("kill").arg("-9").arg(value).output().expect("Failed to execute command"),
            // }
            br {}
        }
    })
}
