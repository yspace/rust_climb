use self::{input_port::{UserInput, UserRepository}, output_port::UserOutput};

use super::entities::User;

pub mod input_port;
pub mod output_port;

pub struct UserUsercase {
    output: Box<dyn UserOutput>,
    repository: Box<dyn UserRepository>,
}

impl UserUsercase {
    pub fn new(
        output: Box<dyn UserOutput>,
        repo: Box<dyn UserRepository>,
    ) -> Self {
        Self { output , repository: repo }
    }
}

impl UserInput for UserUsercase {
    fn get(&self, id: i64) -> Result<(), Box<dyn std::error::Error>> {
        // Get user from database.
        let user: Result<User, std::io::Error> =
            Ok(User::new(1, "yiqing".to_owned(), "18".to_owned())); // Ok处是模拟db查询返回的结果

        let user = user?;

        // Responds to a client through the Output Port.

        self.output.render(&user);
        Ok(())
    }
}
