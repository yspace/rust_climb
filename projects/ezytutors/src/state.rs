// use ezytutors::models::Course;

use super::models::Course ;
// use crate::models::Course;
use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub courses: Mutex<Vec<Course>>,
}
