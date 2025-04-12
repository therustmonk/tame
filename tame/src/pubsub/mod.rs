mod publisher;
mod registrar;

use crb::core::Unique;
use publisher::PubInner;
use std::ops::Deref;

pub trait PubSub: Sized + Send + 'static {
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
