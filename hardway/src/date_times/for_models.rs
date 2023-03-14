// use chrono::Local;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct User {
    name: String,
    pub created_at: u64,
}

impl User {
    pub fn new(name: String) -> Self {
        let now = SystemTime::now();
        let unix_time = now.duration_since(UNIX_EPOCH).expect("back to future");
        Self {
            name,
            // 这个时间戳可以转化成不同类型
            created_at: unix_time.as_secs(),
        }
    }
}
