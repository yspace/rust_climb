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

// 这个技法叫 blanket implementations
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
    // value 位置可以是 Box<dyn Any>|
    components: HashMap<String, Rc<RefCell<dyn Component>>> 
}
impl Components{
    pub fn new()->Self{
        Self::default()
    }

    pub fn add(&mut self,component :Rc<RefCell<dyn Component>>,name:impl ToString) {
        self.components.insert(name.to_string() ,component);
    }
    // 可以同时提供 get get_mut 版本
    // [live stream: Preparing for a ECS Rust tutorial part 4]()
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
    components: HashMap<String, Rc<RefCell<dyn ComponentAny>>>
}

impl AppContext {
    pub fn new() -> Self {
        Self::default()
    }

    // enum 也可以做到名字跟对象绑定
    pub fn add_component1(&mut self,component :Rc<RefCell<dyn Component>>,name:&'static str) {

    }
    pub fn add_component(&mut self,component :Rc<RefCell<dyn ComponentAny>>,name:impl ToString) {
        self.components.insert(name.to_string() ,component);
    }
    pub fn add_component0(&mut self,component :Box<dyn Component>) {

    }

    // name: impl Into<String> // 这个类型也可以！
    // 此处的返回类型可以有很多选择 Option<Ref<dyn ComponentAny>>
    pub fn get_component(&self,name :impl ToString) ->Option<Rc<RefCell<dyn ComponentAny>>> {
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
        // 因为内部可变性 此处可以根据情况调用 borrow|borrow_mut
        let bowrrowed_db = db_comp.borrow() ;
        // let bowrrowed_db = *bowrrowed_db as dyn Any ;
        // 向下转型 通过unsafe块 将trait object引用的第一个指针（data指针 第二个是vtable 虚表指针）转为指向具体类型的引用
        let db: &MyDb = bowrrowed_db.as_any().downcast_ref().unwrap();

        // rust 中向上转型 upcast是比较繁琐｜难的一个话题
    
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