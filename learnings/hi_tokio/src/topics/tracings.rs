use tracing::{debug, error, info, span, warn, Level};
use tracing_subscriber::prelude::*;

mod custom_layer;
use custom_layer::CustomLayer;

pub async fn main() -> mini_redis::Result<()> {
    part2::main() ; return Ok(());

    
    // construct a subscriber that prints formatted traces to stdout
    // let subscriber = tracing_subscriber::FmtSubscriber::new();

    // Start configuring a `fmt` subscriber
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .finish();

    // Set up how `tracing-subscriber` will deal with tracing data.
    // tracing_subscriber::registry().with(custom_layer::CustomLayer).init();
    tracing_subscriber::registry()
        .with(custom_layer::CustomLayer2)
        .init();

    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)?;

    // 调用有tracing 功能的函数
    let number = trace_me(1, 2);
    println!("{}", number);

    Ok(())
}

#[tracing::instrument]
fn trace_me(a: u32, b: u32) -> u32 {
    info!("trace_me: a={}, b={}", a, b);
    a + b
}

mod part2 {
    use tracing::{debug_span, info, info_span};
    use tracing_subscriber::prelude::*;

    // mod custom_layer;
    // use custom_layer::CustomLayer;
    use super::* ;

    pub fn main() {
        tracing_subscriber::registry().with(CustomLayer).init();

        let outer_span = info_span!("outer", level = 0);
        let _outer_entered = outer_span.enter();

        let inner_span = debug_span!("inner", level = 1);
        let _inner_entered = inner_span.enter();

        info!(a_bool = true, answer = 42, message = "first example");
    }
}
