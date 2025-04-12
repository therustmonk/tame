use crate::pubsub::{PubEvent, PubSub, SubId};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, OnEvent};
use crb::core::{Slot, Unique, mpsc};
use crb::superagent::{Drainer, OnRequest, Request};
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct PubAgent<T: PubSub> {
    state: T,
    events_tx: mpsc::UnboundedSender<PubEvent<T>>,
    events_rx: Slot<mpsc::UnboundedReceiver<PubEvent<T>>>,
    subscribers: HashMap<SubId, Unique<Flow>>,
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
            subscribers: HashMap::new(),
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

pub struct Dispatch<T: PubSub> {
    sub_id: Option<SubId>,
    delta: T::Delta,
}

impl<T: PubSub> Dispatch<T> {
    pub fn new(sub_id: Option<SubId>, delta: T::Delta) -> Self {
        Self { sub_id, delta }
    }
}

#[async_trait]
impl<T: PubSub> OnEvent<Dispatch<T>> for PubAgent<T> {
    async fn handle(&mut self, event: Dispatch<T>, _ctx: &mut Context<Self>) -> Result<()> {
        if let Some(sub_id) = event.sub_id {
        } else {
            // Distribute to all subscribers
        }
        Ok(())
    }
}

pub struct Flow {
    sub_id: SubId,
    // TODO: Address of a `SubAgent`
}
