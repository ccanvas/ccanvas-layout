use std::collections::{HashSet, LinkedList};

use ccanvas_layout::{Layout, LayoutRequest};
use libccanvas::{
    bindings::{Event, EventVariant, Subscription},
    client::{Client, ClientConfig},
    features::common::Dimension,
};
use tokio::{sync::OnceCell, task::JoinSet};

const ALLOCATED: &str = "!layout-allocated-rect";
const CONFIRM: &str = "!layout-render-confirm";

#[tokio::main]
async fn main() {
    static CLIENT: OnceCell<Client> = OnceCell::const_new();
    let _ = CLIENT.set(Client::new(ClientConfig::default()).await);
    let subscriptions = CLIENT.get().unwrap().subscribe_multiple(vec![
        Subscription::specific_message_tag("!layout-add".to_string()).into(),
        Subscription::specific_message_tag("!layout-set".to_string()).into(),
        Subscription::specific_message_tag("!layout-remove".to_string()).into(),
        Subscription::ScreenResize.with_priority(100),
        Subscription::Focused.with_priority(50),
    ]);

    let mut term_size = {
        let ((term_width, term_height), _) =
            tokio::join!(CLIENT.get().unwrap().term_size(), subscriptions);
        Dimension::new(term_width, term_height)
    };

    let mut state = Layout::None;

    let mut events_delayed: LinkedList<Event> = LinkedList::new();

    CLIENT
        .get()
        .unwrap()
        .broadcast(serde_json::Value::Null, "!layout-ready".to_string())
        .await;

    loop {
        let event = if let Some(event) = events_delayed.pop_front() {
            event
        } else {
            CLIENT.get().unwrap().recv().await
        };

        match event.get() {
            EventVariant::Message { content, .. } => {
                let content: LayoutRequest =
                    if let Ok(req) = serde_json::from_value(content.clone()) {
                        req
                    } else {
                        continue;
                    };

                match content {
                    LayoutRequest::Add {
                        at,
                        split,
                        constraint_1,
                        constraint_2,
                        component,
                        border,
                    } => {
                        if !state.add(
                            &at,
                            &split,
                            constraint_1,
                            constraint_2,
                            component.clone(),
                            border,
                        ) {
                            continue;
                        }

                        if let Some(component) = component {
                            CLIENT
                                .get()
                                .unwrap()
                                .watch(CONFIRM.to_string(), component)
                                .await;
                        }
                    }
                    LayoutRequest::Remove { at } => {
                        if let Some(state) = state.get(&at) {
                            let mut set = JoinSet::new();

                            state.components().into_iter().for_each(|discrim| {
                                set.spawn(
                                    CLIENT.get().unwrap().watch(CONFIRM.to_string(), discrim),
                                );
                            });

                            while set.join_next().await.is_some() {}
                        } else {
                            continue;
                        }

                        if !state.remove(&at) {
                            continue;
                        }
                    }
                    LayoutRequest::SetLayout { at, layout } => {
                        if !state.set(&at, layout) {
                            continue;
                        }

                        let mut set = JoinSet::new();

                        state.components().into_iter().for_each(|discrim| {
                            set.spawn(CLIENT.get().unwrap().watch(CONFIRM.to_string(), discrim));
                        });

                        while set.join_next().await.is_some() {}
                    }
                }
            }
            EventVariant::Focused => {
                term_size = {
                    let (term_width, term_height) = CLIENT.get().unwrap().term_size().await;
                    Dimension::new(term_width, term_height)
                }
            }
            EventVariant::Resize { width, height } => term_size = Dimension::new(*width, *height),
            _ => continue,
        }

        CLIENT.get().unwrap().clear_all();

        let areas = state.areas(term_size.into(), CLIENT.get().unwrap());

        CLIENT.get().unwrap().renderall().await;
        let mut set = JoinSet::new();
        let mut unconfirmed = HashSet::new();

        areas.into_iter().for_each(|(rect, discrim)| {
            set.spawn(CLIENT.get().unwrap().set(
                ALLOCATED.to_string(),
                discrim.clone(),
                serde_json::to_value(rect).unwrap(),
            ));
            unconfirmed.insert(discrim);
        });

        while set.join_next().await.is_some() {}

        if !unconfirmed.is_empty() {
            loop {
                let event = CLIENT.get().unwrap().recv().await;
                if let EventVariant::ValueUpdated { label, discrim, .. } = event.get() {
                    if label == CONFIRM {
                        if unconfirmed.remove(discrim) && unconfirmed.is_empty() {
                            break;
                        }
                    } else {
                        events_delayed.push_back(event);
                    }
                } else {
                    events_delayed.push_back(event);
                }
            }
        }

        event.done(true);
    }
}
