use serde::{Serialize, de::DeserializeOwned};

pub trait Moveable
where
    Self: DeserializeOwned + Serialize + Clone + Sync + Send + 'static,
{
}

impl<T> Moveable for T where T: DeserializeOwned + Serialize + Clone + Sync + Send + 'static {}
