mod publisher;

pub trait PubSub: Send + 'static {
    type Delta;
    type Query;
    type Publisher;
    type Subscriber;
}
