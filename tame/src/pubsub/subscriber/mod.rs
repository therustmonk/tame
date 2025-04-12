pub mod agent;

use crate::pubsub::registrar::Registrar;
use crate::pubsub::{PubSub, SubEvent, SubId};
use agent::SubAgent;
use anyhow::{Error, Result};
use crb::agent::Address;
use crb::superagent::{Drainer, InteractExt};
use derive_more::{Deref, DerefMut};
use std::sync::Arc;

#[derive(Deref, DerefMut)]
pub struct Sub<T: PubSub> {
    inner: Arc<T::Subscriber>,
}

impl<T: PubSub> Clone for Sub<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: PubSub> Sub<T> {
    pub fn new() -> Self {
        let agent = SubAgent::new();
        let registrar = Registrar::link();
        let agent = registrar.spawn_sub(agent);
        let sub_inner = SubInner { agent };
        let subscriber = T::Subscriber::from(sub_inner);
        let inner = Arc::new(subscriber);
        Self { inner }
    }

    pub async fn events(&mut self) -> Result<Drainer<SubEvent<T>>> {
        let request = agent::GetEvents::new();
        self.interact(request).await.map_err(Error::from)
    }

    pub fn query(&self, query: T::Query) -> Result<()> {
        let query = agent::Inquire::new(query);
        self.event(query)
    }
}

#[derive(Deref, DerefMut)]
pub struct SubInner<T: PubSub> {
    agent: Address<SubAgent<T>>,
}
