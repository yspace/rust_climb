
pub fn run() {
//     use notify_rust::{Notification, Hint};
// Notification::new()
//     .summary("Category:email")
//     .body("This has nothing to do with emails.\nIt should not go away until you acknowledge it.")
//     .icon("thunderbird")
//     .appname("thunderbird")
//     // .hint(Hint::Category("email".to_owned()))
//     // .hint(Hint::Resident(true)) // this is not supported by all implementations
//     .timeout(0) // this however is
//     .show()?;

use notify_rust::Notification;
Notification::new()
    .summary("Firefox News")
    .body("This will almost look like a real firefox notification.")
    .icon("firefox")
    .show().unwrap();
}