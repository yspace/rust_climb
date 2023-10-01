
use chrono::{DateTime, Utc,Duration};
use futures::Future;

#[derive(Debug)]
struct WaitForIt {
    message: String,
    until: DateTime<Utc>,
    polls: u64,
}
impl WaitForIt {
    pub fn new(message: String, delay: Duration) -> WaitForIt {
        WaitForIt {
            polls: 0,
            message: message,
            until: Utc::now() + delay,
        }
    }
}


// impl Future for WaitForIt {
//     type Item = String;
//     type Error = Box<Error>;

//     fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
//         let now = Utc::now();
//         if self.until < now {
//             Ok(Async::Ready(
//                 format!("{} after {} polls!", self.message, self.polls),
//             ))
//         } else {
//             self.polls += 1;

//             println!("not ready yet --> {:?}", self);
//             Ok(Async::NotReady)
//         }
//     }
// }
