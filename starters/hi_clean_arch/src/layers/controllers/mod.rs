use super::{usecases::{output_port, self, input_port::UserInput}, presenters, gateways};


pub struct User{

}

// 真实场景中上下文是从main入口文件中传递过来的
struct Context{}
impl Context {
    pub fn get_user_id(self) -> i64 {
        1
    }
}

impl User {
   fn get_user(ctx: Context) -> Result<(),Box<dyn std::error::Error>> {
    let user_id = ctx.get_user_id();
    
    let output_port = presenters::User::new();
    let repository_port = gateways::User::new();
    
    let usercase = usecases::UserUsercase::new(
       Box::new( output_port), Box::new(repository_port)
    );
    
    let result = usercase.get(user_id) ;
    match result {
        Ok(_) => {println!("done!");},
        Err(err) => {
            println!("{}", err);
        },
    }

    Ok(())
   }
}

#[test]
fn test_user(){
    // let user = User{};
    User::get_user(Context {  });
}