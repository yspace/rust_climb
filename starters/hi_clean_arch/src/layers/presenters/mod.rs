use super::usecases::output_port::UserOutput;

// 表示层 可以有很多 比如 json ，xml yaml html ...
struct User{
    // ctx: Context,
}

impl User {
    pub fn new(/*ctx: Context*/) -> Self {
        Self {}
    }
}

impl UserOutput for User {
    fn render(&self,user: &super::entities::User)  {
        // u.ctx.JSON(200, user)
       
        println!("render json :{:?}", user);
    }
}