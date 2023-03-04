
use serde::{Deserialize, Serialize};

pub fn run(){
    let msg = Message::new(
        "yiqing".to_string(),
        "hi redis".to_string());

        let serialized = serde_json::to_string(&msg).unwrap();
        println!("{}", serialized);

        let msg2: Message = serde_json::from_str(&serialized).unwrap();
        println!("{:?}", msg2);
        
}

#[test]
fn test_run() {
    run();
}

// @see https://blog.logrocket.com/json-and-rust-why-serde_json-is-the-top-choice/
#[derive(Debug,Deserialize,Serialize)]
struct Message{
    user: String,
    msg: String,
}

impl Message {
    pub fn new(user: String,msg: String) -> Self {
        Self {
            user,
            msg,
        }
    }
}