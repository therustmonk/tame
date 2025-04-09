mod publisher;

use publisher::PubInner;
use std::ops::Deref;

pub trait PubSub: Sized + Send + 'static {
    type Delta;
    type Query;
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
