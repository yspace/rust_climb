use std::{cell::{RefCell, Ref}, rc::Rc, collections::HashMap, any::Any};

use strum_macros::ToString ;

// 这里Component只是一个marker的角色 用Any的话会更宽泛些 这里是试验性的 以后用Any吧
pub trait Component: std::fmt::Debug + Any{}
// pub trait Component: std::fmt::Debug + Any{}
// pub trait Component: std::fmt::Debug + Any{}
// pub trait Component0<T: Debug = Self>:Debug<T>{}

// @see https://stackoverflow.com/questions/41604107/rust-trait-object-conversion
pub trait ComponentAny: Component + Any {
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
}

impl<T> ComponentAny for T
    where T: Component + Any
{
    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}


#[derive(Debug,Default)]
struct Components{
    components: HashMap<String, Rc<RefCell<dyn Component>>> 
}
impl Components{
    pub fn new()->Self{
        Self::default()
    }

    pub fn add(&mut self,component :Rc<RefCell<dyn Component>>,name:impl ToString) {
        self.components.insert(name.to_string() ,component);
    }
    pub fn get(&self,name:impl ToString)->Option<Rc<RefCell<dyn Component>>> {
        if let Some(comp) = self.components.get(&name.to_string()) {
            Some(comp.clone())
        }else {
            None
        }
    }
}

#[derive(Debug,ToString)]
enum ComponentName{
    DB
}

#[derive(Debug,Default)]
pub struct AppContext{
    /*
    the following other types implement trait `Debug`:
            (dyn Any + 'static)
            (dyn Any + Send + 'static)
            (dyn Any + Send + Sync + 'static)
    */
    // 类型可以是Components 
    components: HashMap<String, Rc<RefCell<dyn Component>>>
}

impl AppContext {
    pub fn new() -> Self {
        Self::default()
    }

    // enum 也可以做到名字跟对象绑定
    pub fn add_component1(&mut self,component :Rc<RefCell<dyn Component>>,name:&'static str) {

    }
    pub fn add_component(&mut self,component :Rc<RefCell<dyn Component>>,name:impl ToString) {
        self.components.insert(name.to_string() ,component);
    }
    pub fn add_component0(&mut self,component :Box<dyn Component>) {

    }

    // name: impl Into<String> // 这个类型也可以！
    pub fn get_component(&self,name :impl ToString) ->Option<Rc<RefCell<dyn Component>>> {
        if let Some(comp) = self.components.get(&name.to_string()) {
            Some(comp.clone())
        }else {
            None
        }
    }
}

#[derive(Debug)]
struct MyDb{}

impl Component for MyDb {}

//
#[derive(Debug)]
pub struct MainState{
    ctx: AppContext,
}

impl MainState {
    pub fn new(mut ctx:AppContext/* ,some-other-component... */) -> Self {
        
        let db = MyDb{} ;
        let rc_db = Rc::new(RefCell::new(db)) ;
        ctx.add_component(rc_db, ComponentName::DB) ;
        
        Self {
            ctx,
        }
    }

    pub fn do_something(&self){
        let db_comp = self.ctx.get_component(ComponentName::DB).expect("cann't found component db"); ;
        // let bowrrowed_db = db_comp.borrow() ;
        // let bowrrowed_db = *bowrrowed_db as dyn Any ;
        // let db = bowrrowed_db.downcast_ref().unwrap();
    
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod test{
    use super::*;

    #[test]
    fn it_works(){
        let speed = 42.0_f32 ;
        let s2 = 42.000000000001_f32;
        assert_eq!(speed, s2);
    }
}