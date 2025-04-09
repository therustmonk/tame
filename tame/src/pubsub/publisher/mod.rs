pub mod agent;

use crate::pubsub::PubSub;
use crate::pubsub::registrar::Registrar;
use agent::PubAgent;
use crb::agent::Address;
use derive_more::{Deref, DerefMut};
use std::sync::Arc;

#[derive(Deref, DerefMut)]
pub struct Pub<T: PubSub> {
    inner: Arc<T::Publisher>,
}

impl<T: PubSub> Clone for Pub<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: PubSub> Pub<T> {
    pub fn new(state: T) -> Self {
        let agent = PubAgent::new(state);
        let registrar = Registrar::link();
        let agent = registrar.spawn_pub(agent);
        let pub_inner = PubInner { agent };
        let publisher = T::Publisher::from(pub_inner);
        let inner = Arc::new(publisher);
        Self { inner }
    }
}

#[derive(Deref, DerefMut)]
pub struct PubInner<T: PubSub> {
    agent: Address<PubAgent<T>>,
}
