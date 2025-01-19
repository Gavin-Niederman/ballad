use std::{
    fmt::Debug,
    sync::{Arc, Weak},
};

use gtk::glib::clone::{Downgrade, Upgrade};
use smol::{
    channel::{Receiver, Sender},
    lock::RwLock,
};

#[derive(Debug, Default)]
struct ReactiveInner<T: Clone> {
    value: T,
    listeners: Vec<Sender<T>>,
}

impl<T: Clone> ReactiveInner<T> {
    pub const fn new(initial: T) -> Self {
        Self {
            value: initial,
            listeners: Vec::new(),
        }
    }

    pub async fn set(&mut self, value: T) {
        self.value = value.clone();
        for listener in self.listeners.iter() {
            listener.send(value.clone()).await.unwrap();
        }
    }

    pub fn subscribe(&mut self, listener: Sender<T>) {
        self.listeners.push(listener);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ListenerControl {
    Remove,
    #[default]
    Keep,
}

#[derive(Debug, Clone, Default)]
pub struct WeakReactive<T: Clone> {
    inner: Weak<RwLock<ReactiveInner<T>>>,
}

impl<T: Clone> Upgrade for WeakReactive<T> {
    type Strong = Reactive<T>;

    fn upgrade(&self) -> Option<Self::Strong> {
        Some(Reactive {
            inner: self.inner.upgrade()?,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Reactive<T: Clone> {
    inner: Arc<RwLock<ReactiveInner<T>>>,
}

impl<T: Clone> Reactive<T> {
    pub fn new(initial: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(ReactiveInner::new(initial))),
        }
    }

    pub async fn set(&self, value: T) {
        self.inner.write().await.set(value).await;
    }
    pub fn set_blocking(&self, value: T) {
        smol::block_on(self.set(value));
    }
    pub async fn get(&self) -> T {
        self.inner.read().await.value.clone()
    }
    pub fn get_blocking(&self) -> T {
        smol::block_on(self.get())
    }

    pub async fn subscribe(&self) -> Receiver<T> {
        let (sender, receiver) = smol::channel::unbounded();
        self.inner.write().await.subscribe(sender);

        receiver
    }

    pub fn connect(&self, connection: impl Fn(Self, T) -> ListenerControl + 'static)
    where
        T: 'static,
    {
        let this = self.clone();
        gtk::glib::spawn_future_local(async move {
            let receiver = this.subscribe().await;
            while let Ok(value) = receiver.recv().await {
                if connection(this.clone(), value) == ListenerControl::Remove {
                    break;
                }
            }
        });
    }
}

impl<T: Clone> Downgrade for Reactive<T> {
    type Weak = WeakReactive<T>;

    fn downgrade(&self) -> WeakReactive<T> {
        WeakReactive {
            inner: self.inner.downgrade(),
        }
    }
}
