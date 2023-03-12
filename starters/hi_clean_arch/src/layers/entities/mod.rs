/*
type User struct {
   ID        uint       `json:"id"`
   Name      string     `json:"name"`
   Age       string     `json:"age"`
}
 */

#[derive(Debug, Default)]
pub struct User {
    id: u64,
    name: String,
    age: String,
}

impl User {
    pub fn new(id: u64, name: String, age: String) -> Self {
        Self { id, name, age }
    }
}
