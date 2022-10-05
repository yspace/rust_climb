https://stackoverflow.com/questions/37572734/how-can-i-implement-the-observer-pattern-in-rust

https://github.com/lpxxn/rust-design-pattern

~~~rust

use std::rc::Weak;

struct Event;

trait Observable {
    fn register(&mut self, observer: Weak<dyn Observer>);
}

trait Observer {
    fn notify(&self, event: &Event);
}

~~~
The key is to allocate the Observer into a Rc and then hand over Weak (weak references) to the Observable.

If the Observer needs be modified on the Event, then either it needs internal mutability or it needs to be wrapped into a RefCell (passing Weak<RefCell<dyn Observer>> to the Observable).

When notifying, the Observable will regularly realize that there are dead weak-references (the Observer has disappeared), it can remove those then, lazily.


~~~rust

pub struct Notifier<E> {
    subscribers: Vec<Box<dyn Fn(&E)>>,
}

impl<E> Notifier<E> {
    pub fn new() -> Notifier<E> {
        Notifier {
            subscribers: Vec::new(),
        }
    }

    pub fn register<F>(&mut self, callback: F)
    where
        F: 'static + Fn(&E),
    {
        self.subscribers.push(Box::new(callback));
    }

    pub fn notify(&self, event: E) {
        for callback in &self.subscribers {
            callback(&event);
        }
    }
}

// =================================================================================

pub enum Event {} // You make Event hold anything you want to fire 

pub trait Listener {
    fn notify(&mut self, event: &Event);
}

pub trait Dispatchable<T>
    where T: Listener
{
    fn register_listener(&mut self, listener: Arc<RefCell<T>>);
}

pub struct Dispatcher<T>
    where T: Listener
{
    /// A list of synchronous weak refs to listeners
    listeners: Vec<Weak<RefCell<T>>>,
}

impl<T> Dispatchable<T> for Dispatcher<T>
    where T: Listener
{
    /// Registers a new listener
    fn register_listener(&mut self, listener: Arc<RefCell<T>>) {
        self.listeners.push(Arc::downgrade(&listener));
    }
}

impl<T> Dispatcher<T>
    where T: Listener
{
    pub fn new() -> Dispatcher<T> {
        Dispatcher { listeners: Vec::new() }
    }

    pub fn num_listeners(&self) -> usize {
        self.listeners.len()
    }

    pub fn dispatch(&mut self, event: Event) {
        let mut cleanup = false;
        // Call the listeners
        for l in self.listeners.iter() {
            if let Some(mut listener_rc) = l.upgrade() {
                let mut listener = listener_rc.borrow_mut();
                listener.notify(&event);
            } else {
                println!("Cannot get listener, cleanup necessary");
                cleanup = true;
            }
        }
        // If there were invalid weak refs, clean up the list
        if cleanup {
            println!("Dispatcher is cleaning up weak refs");
            self.listeners.retain(|ref l| {
                // Only retain valid weak refs
                let got_ref = l.clone().upgrade();
                match got_ref {
                    None => false,
                    _ => true,
                }
            });
        }
    }
}

~~~