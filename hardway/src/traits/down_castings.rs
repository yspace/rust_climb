use std::any::Any;



// trait AsAny {
//     fn as_any (&self) ->Box<dyn Any > {
//       Box::new( self) 
//     }
// }

trait Component /*: AsAny */ {
    fn name(&self)->String;
    // fn as_any<'a>(&'a self) ->Box<dyn Any + 'a>{
    //     Box::new(self) 
    // }
    fn as_any(&self) -> &dyn Any ;
}

#[derive(Debug)]
struct MyDb{
    url: String,
}
impl  MyDb {
    
    fn do_something(&self){
        println!("connecting to {}", self.url);
    }
}
impl Default for MyDb
{
    fn default() -> Self {
        Self { url: String::from("localhost:3306") }
    }

}

impl Component for MyDb{
    fn name(&self)->String {
    //   "mysql-instance".into()
      "mysql-instance".to_owned()
    }

    fn as_any(&self) -> &dyn Any  {
        self
    }
}

// #[derive(Debug)]
struct App{
    components: Vec::<Box<dyn Component>>,
}
impl Default for App {
    fn default() -> Self {
        Self { 
            // components: Default::default() 
            components: Vec::<Box<dyn Component>>::new(),
        }
    }
}

#[test]
fn test_components(){
    let mut app = App::default() ;
    let db = Box::new(MyDb::default());
    app.components.push(db);

    app.components.into_iter().for_each(|component|{
        println!("{:?}",  component.name());

        // 向下转型
        if let Some(db) = component.as_any().downcast_ref::<MyDb>() {
            db.do_something() ;
        }
    })
}