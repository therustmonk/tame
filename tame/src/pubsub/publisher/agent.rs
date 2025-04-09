use crate::pubsub::PubSub;
use crb::agent::{Agent, AgentSession};

pub struct PubAgent<T: PubSub> {
    state: T,
}

impl<T: PubSub> Agent for PubAgent<T> {
    type Context = AgentSession<Self>;
}

impl<T: PubSub> PubAgent<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}
