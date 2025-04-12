use crate::pubsub::{PubSub, SubEvent, SubId};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, OnEvent};
use crb::core::{Slot, Unique, mpsc};

pub struct SubAgent<T: PubSub> {
    events_tx: mpsc::UnboundedSender<SubEvent<T>>,
    events_rx: Slot<mpsc::UnboundedReceiver<SubEvent<T>>>,
}

impl<T: PubSub> Agent for SubAgent<T> {
    type Context = AgentSession<Self>;
}

impl<T: PubSub> SubAgent<T> {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            events_tx: tx,
            events_rx: Slot::filled(rx),
        }
    }
}
