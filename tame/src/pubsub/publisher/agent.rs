use super::PubEvent;
use crate::pubsub::PubSub;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context};
use crb::core::{Slot, mpsc};
use crb::superagent::{Drainer, OnRequest, Request};
use std::marker::PhantomData;

pub struct PubAgent<T: PubSub> {
    state: T,
    events_tx: mpsc::UnboundedSender<PubEvent<T>>,
    events_rx: Slot<mpsc::UnboundedReceiver<PubEvent<T>>>,
}

impl<T: PubSub> Agent for PubAgent<T> {
    type Context = AgentSession<Self>;
}

impl<T: PubSub> PubAgent<T> {
    pub fn new(state: T) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            state,
            events_tx: tx,
            events_rx: Slot::filled(rx),
        }
    }
}

pub struct GetEvents<T: PubSub> {
    _type: PhantomData<T>,
}

impl<T: PubSub> GetEvents<T> {
    pub fn new() -> Self {
        Self { _type: PhantomData }
    }
}

impl<T: PubSub> Request for GetEvents<T> {
    type Response = Drainer<PubEvent<T>>;
}

#[async_trait]
impl<T: PubSub> OnRequest<GetEvents<T>> for PubAgent<T> {
    async fn on_request(
        &mut self,
        _: GetEvents<T>,
        _ctx: &mut Context<Self>,
    ) -> Result<Drainer<PubEvent<T>>> {
        let rx = self.events_rx.take()?;
        let drainer = Drainer::from_mpsc(rx);
        Ok(drainer)
    }
}
