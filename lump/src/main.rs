use dioxus::prelude::*;
use dioxus_desktop::Config;
use futures::StreamExt;
use std::cell::Cell;
use bash_connector::Cache;
use regex::Regex;
use futures_channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
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
    sender: Cell<Option<UnboundedSender<String>>>,
    receiver: Cell<Option<UnboundedReceiver<String>>>,
}

pub fn perform_action() -> String {
    let mut cache = Cache {
        content: String::from(""),
    };

    if cache.collect_data() {
        return cache.content.clone();
    }
    "".to_string()
}

fn app(cx: Scope<AppProps>) -> Element {
    let output = use_state(cx, || "".to_string());

    let _ = use_coroutine(cx, |_: UnboundedReceiver<()>| {
        let receiver = cx.props.receiver.take();
        let output = output.to_owned();
        async move {
            if let Some(mut receiver) = receiver {
                while let Some(msg) = receiver.next().await {
                    output.set(msg);
                }
            }
        }
    });
    cx.render(rsx! {
        div {
            rsx!("{output}")
        }
    })
}
