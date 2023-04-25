// The panic hook is a global resource.
use std::panic;

// #[test]
fn main() {
    /*
    The panic hook is invoked when a thread panics,
    but before the panic runtime is invoked.
    As such, the hook will run with both the aborting and unwinding runtimes.

     The default hook prints a message to standard error and generates a backtrace if requested,
      but this behavior can be customized with the set_hook and take_hook functions.



     */
    panic::set_hook(Box::new(|info| {
        // The hook is provided with a PanicInfo struct which contains information about the origin of the panic,
        //  including the payload passed to panic! and the source code location from which the panic originated.
        println!("Custom panic hook: {:?}", info);
    }));

    panic!("Normal panic");
}

mod _2 {
    use std::panic;

    fn main() {
        panic::set_hook(Box::new(|_| {
            println!("Custom panic hook");
        }));

        // Unregisters the current panic hook, returning it. 注销之前注册的自定义hook.
        // If no custom hook is registered, the default hook will be returned.
        let _ = panic::take_hook();

        panic!("Normal panic");
    }
}
