mod agent;

use super::PubSub;
use agent::PubAgent;
use crb::agent::Address;
use std::sync::Arc;

pub struct Pub<T: PubSub> {
    inner: Arc<PubInner<T>>,
}

struct PubInner<T: PubSub> {
    agent: Address<PubAgent<T>>,
}
