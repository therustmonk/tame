mod publisher;
mod registrar;
mod subscriber;

use crb::core::{Unique, watch};
use publisher::PubInner;
use std::ops::Deref;

pub trait PubSub: Sized + Sync + Send + 'static {
    type Delta: Send;
    type Query: Send;
    type Publisher: Publisher<Self>;
    type Subscriber: Subscriber<Self>;
}

pub trait Publisher<T: PubSub>
where
    Self: From<PubInner<T>>,
    Self: Deref<Target = PubInner<T>>,
{
}

pub trait Subscriber<T: PubSub> {}

pub type PubId = Unique;

pub type SubId = Unique;

pub struct PubEvent<T: PubSub> {
    pub sub_id: SubId,
    pub value: PubValue<T>,
}

pub enum PubValue<T: PubSub> {
    Connected,
    Query(T::Query),
    Disconnected,
}

pub enum SubEvent<T: PubSub> {
    Loaded(State<T>),
    Delta(T::Delta),
    Lost,
}

pub struct State<T> {
    state_rx: watch::Receiver<T>,
}

impl<T> State<T> {
    pub fn new(state: T) -> (Self, watch::Sender<T>) {
        let (state_tx, state_rx) = watch::channel(state);
        (Self { state_rx }, state_tx)
    }

    pub fn borrow(&self) -> watch::Ref<T> {
        self.state_rx.borrow()
    }
}
