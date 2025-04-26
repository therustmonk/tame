use crate::pubsub::{PubSub, SubEvent, SubId};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, Context, OnEvent};
use crb::core::{Slot, Unique, mpsc};
use crb::superagent::{Drainer, OnRequest, Request};
use std::marker::PhantomData;

pub struct SubAgent<T: PubSub> {
    events_tx: mpsc::UnboundedSender<SubEvent<T>>,
    events_rx: Slot<mpsc::UnboundedReceiver<SubEvent<T>>>,
}

impl<T: PubSub> Agent for SubAgent<T> {
    type Context = AgentSession<Self>;
    type Link = Address<Self>;
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

pub struct GetEvents<T: PubSub> {
    _type: PhantomData<T>,
}

impl<T: PubSub> GetEvents<T> {
    pub fn new() -> Self {
        Self { _type: PhantomData }
    }
}

impl<T: PubSub> Request for GetEvents<T> {
    type Response = Drainer<SubEvent<T>>;
}

#[async_trait]
impl<T: PubSub> OnRequest<GetEvents<T>> for SubAgent<T> {
    async fn on_request(
        &mut self,
        _: GetEvents<T>,
        _ctx: &mut Context<Self>,
    ) -> Result<Drainer<SubEvent<T>>> {
        let rx = self.events_rx.take()?;
        let drainer = Drainer::from_mpsc(rx);
        Ok(drainer)
    }
}

pub struct Inquire<T: PubSub> {
    query: T::Query,
}

impl<T: PubSub> Inquire<T> {
    pub fn new(query: T::Query) -> Self {
        Self { query }
    }
}

#[async_trait]
impl<T: PubSub> OnEvent<Inquire<T>> for SubAgent<T> {
    async fn handle(&mut self, event: Inquire<T>, _ctx: &mut Context<Self>) -> Result<()> {
        Ok(())
    }
}
