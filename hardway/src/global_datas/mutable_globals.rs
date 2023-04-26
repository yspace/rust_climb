use std::sync::atomic::{AtomicU8, Ordering};

// The global variable is not mut, so no unsafe is needed.
static LOG_LEVEL: AtomicU8 = AtomicU8::new(0);
// The code is thread-safe and as performant as an unsafe version 

pub fn get_log_level() -> u8 {
    LOG_LEVEL.load(Ordering::Relaxed)
}

pub fn set_log_level(level: u8) {
    // If you need stricter ordering guarantees between LOG_LEVEL and other data in the program, you can use Ordering::SeqCst instead.
    LOG_LEVEL.store(level, Ordering::Relaxed);
}