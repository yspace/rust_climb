pub fn run() {
    println!("clokwerk...");
    // Scheduler, and trait for .seconds(), .minutes(), etc.
    use clokwerk::{Scheduler, TimeUnits};
    // Import week days and WeekDay
    use clokwerk::Interval::*;
    use std::thread;
    use std::time::Duration;

    // Create a new scheduler
    let mut scheduler = Scheduler::new();
    // or a scheduler with a given timezone
    let mut scheduler = Scheduler::with_tz(chrono::Utc);
    // Add some tasks to it
    scheduler
        .every(10.minutes())
        .plus(30.seconds())
        .run(|| println!("Periodic task"));
    scheduler
        .every(1.day())
        .at("3:20 pm")
        .run(|| println!("Daily task"));
    scheduler
        .every(Tuesday)
        .at("14:20:17")
        .and_every(Thursday)
        .at("15:00")
        .run(|| println!("Biweekly task"));

    // Manually run the scheduler in an event loop
    for _ in 1..10 {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(10));
    }
    // Or run it in a background thread
    let thread_handle = scheduler.watch_thread(Duration::from_millis(100));
    // The scheduler stops when `thread_handle` is dropped, or `stop` is called
    // thread_handle.stop();
}
