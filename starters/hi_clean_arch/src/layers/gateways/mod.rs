use super::usecases::input_port::UserRepository;

pub struct User {
    // db: db::Database
}

impl User {
    pub fn new(/*db: db::Database*/) -> Self {
        Self{
            // db
        }
    }
}

impl UserRepository for User {
    fn get_by_id(&self, id: i64) -> Result<super::entities::User, Box<dyn std::error::Error>> {
        // get user data
        // user, err := u.db.User.Find(id)
        // if err != nil {
        //   return nil, err
        // }

        // return user, nil

        Ok(super::entities::User::new(
            2,
            "yiqing2".to_owned(),
            "18".to_owned(),
        ))
    }
}
