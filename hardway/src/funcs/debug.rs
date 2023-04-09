// A good rule in Rust is that every struct should have std::debug::Debug. But what if that struct contains closures.
// @see https://boydjohnson.dev/blog/impl-debug-for-fn-type/

#[derive(Debug)]
pub struct User {
    name: String,
    age: usize,
    on_grow: Option<GrowFn>,
    
}

// pub type GrowFn = Box<dyn FnOnce(&mut User)>;
// pub type GrowFn = Box<dyn Fn(/*&mut User*/)>;
pub type GrowFn = Box<dyn OnGrowFn>;

impl User {
    pub fn new(name: String, age: usize)-> Self {
        Self { 
            name , 
            age ,
             on_grow:None,
         
         }
    }

    fn grow(&mut self) ->&mut Self  
    {
        self.age += 1 ;

    //    if let Some(on_grow) = self.on_grow{
    //     on_grow(self) ;
    //    }
        if(self.on_grow.is_some()){
            // 
            println!("咋调用回调函数");
            // https://rustwiki.org/en/book/ch17-03-oo-design-patterns.html
            /*
            We call the as_ref method on the Option because we want a reference to the value inside the Option rather than ownership of the value. 
            Because state is an Option<Box<dyn State>>, when we call as_ref, an Option<&Box<dyn State>> is returned. 
            If we didn’t call as_ref, we would get an error because we can’t move state out of the borrowed &self of the function parameter.
            
             */
            self.on_grow.as_ref().unwrap()(/*&mut self*/) ;
        }

        self
    }
}

#[test]
fn it_works() {
    let mut user = User::new("yiqing".to_owned(),18);

    user.on_grow = Some(Box::new(|/*user*/|{
        println!("ai! one year passed!") ;
    }));

    user.grow().grow().grow() ;

    println!("{}", user.age);

}

pub trait OnGrowFn: Fn ( ){}

impl<F> OnGrowFn for F where F: Fn( ) { }

impl std::fmt::Debug for OnGrowFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OnGrowFn")
    }
}

