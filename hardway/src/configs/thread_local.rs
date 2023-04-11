use std::sync::{Arc, RwLock};

// @see https://blog.sentry.io/2018/04/05/you-cant-rust-that/
// @see http://oostens.me/posts/singletons-in-rust/

#[derive(Default)]
struct ConfigInner {
    debug_mode: bool,
}

struct Config {
    inner: RwLock<ConfigInner>,
}

impl Config {
    pub fn new() -> Arc<Config> {
        Arc::new(Config {
            inner: RwLock::new(Default::default()),
        })
    }
    pub fn current() -> Arc<Config> {
        CURRENT_CONFIG.with(|c| c.clone())
    }
    pub fn debug_mode(&self) -> bool {
        self.inner.read().unwrap().debug_mode
    }
    pub fn set_debug_mode(&self, value: bool) {
        self.inner.write().unwrap().debug_mode = value;
    }
}
/*

If you do not need this type to work with threads you can also replace Arc with Rc and RwLock with RefCell.

To recap: when you need to borrow data that outlives the lifetime of something you need refcounting.
Don’t be afraid of using Arc but be aware that this locks you to immutable data.
Combine with interior mutability (like RwLock) to make the object mutable.

 */
thread_local! {
    static CURRENT_CONFIG: Arc<Config> = Config::new();
}

fn main() {
    let config = Config::current();
    config.set_debug_mode(true);
    if config.debug_mode() {
        // do something
    }
}

mod _0 {
    use std::sync::Arc;

    #[derive(Default)]
    struct Config {
        pub debug_mode: bool,
    }

    impl Config {
        pub fn current() -> Arc<Config> {
            CURRENT_CONFIG.with(|c| c.clone())
        }
    }

    thread_local! {
        static CURRENT_CONFIG: Arc<Config> = Arc::new(Default::default());
    }

    fn main() {
        let config = Config::current();
        // here we can *immutably* work with config
        if config.debug_mode {
            // do something
        }
    }
}

mod interior_mutability {
    /*
    There are two options for this. One is to wrap the Config in something like an RwLock.
     The second is to have the Config use locking internally.

     单线程时：
     If you do not need this type to work with threads you can also replace Arc with Rc and RwLock with RefCell.
     */
    use std::sync::{Arc, RwLock};

    #[derive(Default)]
    struct ConfigInner {
        debug_mode: bool,
    }

    struct Config {
        inner: RwLock<ConfigInner>,
    }

    impl Config {
        pub fn new() -> Arc<Config> {
            Arc::new(Config {
                inner: RwLock::new(Default::default()),
            })
        }
        pub fn current() -> Arc<Config> {
            CURRENT_CONFIG.with(|c| c.clone())
        }
        pub fn debug_mode(&self) -> bool {
            self.inner.read().unwrap().debug_mode
        }
        pub fn set_debug_mode(&self, value: bool) {
            self.inner.write().unwrap().debug_mode = value;
        }
    }

    thread_local! {
        static CURRENT_CONFIG: Arc<Config> = Config::new();
    }

    fn main() {
        let config = Config::current();
        config.set_debug_mode(true);
        if config.debug_mode() {
            // do something
        }
    }
}

mod _2 {
    use std::sync::{Arc, RwLock};

    #[derive(Default)]
    struct Config {
        pub debug_mode: bool,
    }

    impl Config {
        pub fn current() -> Arc<Config> {
            CURRENT_CONFIG.with(|c| c.read().unwrap().clone())
        }
        pub fn make_current(self) {
            CURRENT_CONFIG.with(|c| *c.write().unwrap() = Arc::new(self))
        }
    }
    /*
    But the above pattern of effectively having Arc<RwLock<Config>> can be a bit problematic and swapping it for RwLock<Arc<Config>> can be significantly better.
     */
    thread_local! {
        static CURRENT_CONFIG: RwLock<Arc<Config>> = RwLock::new(Default::default());
    }

    fn main() {
        Config { debug_mode: true }.make_current();
        if Config::current().debug_mode {
            // do something
        }
    }
    /*

        Likewise, you can again switch Arc for Rc and RwLock for RefCell if you do not need this to work with threads.
         If you are just working with thread locals you can also combine RefCell with Arc.

    To recap: instead of using interior mutability where an object changes its internal state,
     consider using a pattern where you promote new state to be current and current consumers of the old state will continue to hold on to it by putting an Arc into an RwLock.

       总结
        following mantra is now what I want to print out and hang somewhere:

    Handles, not self referential pointers
    Reference count your way out of lifetime / borrow checker hell
    Consider promoting new state instead of interior mutability

         */
}
