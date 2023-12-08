
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct MyAppData {
    pub counter: Arc<RwLock<usize>>,
}