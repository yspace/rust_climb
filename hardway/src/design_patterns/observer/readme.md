[How can I implement the observer pattern in Rust?](https://stackoverflow.com/questions/37572734/how-can-i-implement-the-observer-pattern-in-rust)

[Observer Pattern in Rust ](https://dev.to/brookzerker/observer-pattern-in-rust-57hl)
https://github.com/brooks-builds/observer_pattern_in_rust/tree/master/oop/src](https://github.com/brooks-builds/observer_pattern_in_rust/tree/master/oop/src

```rust

use std::sync::{Arc, Mutex};

pub struct EventSystem {
    wrapped_observers: Vec<Arc<Mutex<dyn Observer>>>,
}

impl EventSystem {
    pub fn new() -> EventSystem {
        EventSystem {
            wrapped_observers: vec![],
        }
    }

    pub fn notify(&self, event: GameEvent) {
        for wrapped_observer in self.wrapped_observers.clone() {
            let mut observer = wrapped_observer.lock().unwrap();
            observer.on_notify(&event);
        }
    }

    pub fn add_observer(&mut self, observer: Arc<Mutex<dyn Observer>>) {
        self.wrapped_observers.push(observer);
    }
}

pub trait Observer {
    fn on_notify(&mut self, event: &GameEvent);
}

pub enum GameEvent {
    PlayerScored,
    AiScored,
}

```
