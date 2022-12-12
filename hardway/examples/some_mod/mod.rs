use std::{cell::RefCell, rc::Rc, collections::HashMap};

use strum_macros::ToString ;

pub trait Component: std::fmt::Debug{}
// pub trait Component0<T: Debug = Self>:Debug<T>{}

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
        let db = self.ctx.get_component(ComponentName::DB).expect("cann't found component db"); ;
    }
}