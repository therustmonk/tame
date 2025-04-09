mod agent;

use super::PubSub;
use agent::PubAgent;
use crb::agent::Address;
use std::sync::Arc;

pub struct Pub<T: PubSub> {
    inner: Arc<T::Publisher>,
}

impl<T: PubSub> Pub<T> {
    pub fn new(state: T) -> Self {
        todo!()
    }
}

pub struct PubInner<T: PubSub> {
    agent: Address<PubAgent<T>>,
}
