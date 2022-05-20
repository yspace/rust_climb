use tokio::task::yield_now;
use std::rc::Rc;

pub async fn main() {
    tokio::spawn(async {
        // The scope forces `rc` to drop before `.await`.
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        // `rc` is no longer used. It is **not** persisted when
        // the task yields to the scheduler
        yield_now().await;
    });
}

// async fn main2() {
//     tokio::spawn(async {
//         let rc = Rc::new("hello");

//         // `rc` is used after `.await`. It must be persisted to
//         // the task's state.
//         yield_now().await;

//         println!("{}", rc);
//     });
// }