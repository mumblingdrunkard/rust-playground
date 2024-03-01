use std::marker::PhantomData;

use crate::observer::Observer;

pub struct Event<'a, T1> {
    subscribers: Vec<Box<dyn Observer<T1>>>,
    _pd: PhantomData<&'a ()>,
}

impl<'a, T1> Event<'a, T1>
where
    T1: Clone,
{
    pub fn new() -> Self {
        return Self {
            subscribers: Vec::new(),
            _pd: PhantomData::default(),
        };
    }

    pub fn subscribe(&mut self, arg: Box<dyn Observer<T1> + 'a>) {
        self.subscribers.push(arg);
    }

    pub fn invoke(&self, arg1: T1) {
        for subscriber in &self.subscribers {
            subscriber.invoke(arg1.clone());
        }
    }
}
