use std::fmt::Debug;

pub trait Service: Debug + Send + Sync {
    fn get(&self);
    fn post(&self);
}
