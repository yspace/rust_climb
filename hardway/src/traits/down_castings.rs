use std::{any::Any, cell::{RefCell, Ref, RefMut}, rc::Rc, collections::HashMap};

// [Rust 反射，反射库 bevy_reflect，实现运行时检测是否实现了 trait](https://zhuanlan.zhihu.com/p/615577638)
// 看起来Bevey库的反射功能有点强 闲了看看

// [Rust源码阅读： Cell/RefCell与内部可变性](https://zhuanlan.zhihu.com/p/384388192)
// 上面这个也不错

// trait AsAny {
//     fn as_any (&self) ->Box<dyn Any > {
//       Box::new( self)
//     }
// }

trait Component /*: AsAny */ {
    fn name(&self) -> String;
    // 在这里不能实现呀 相当于php中后期静态绑定static的能力
    // fn as_any<'a>(&'a self) ->Box<dyn Any + 'a>{
    //     Box::new(self)
    // }
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

#[derive(Debug)]
struct MyDb {
    url: String,
}
impl MyDb {
    fn do_something(&self) {
        println!("connecting to {}", self.url);
    }

    fn do_something2(&mut self) {
        println!("do something2 ... {}", self.url);
    }
}
impl Default for MyDb {
    fn default() -> Self {
        Self {
            url: String::from("localhost:3306"),
        }
    }
}

impl Component for MyDb {
    fn name(&self) -> String {
        //   "mysql-instance".into()
        "mysql-instance".to_owned()
    }
    
    // 无法在Component上做默认实现！？？
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

// #[derive(Debug)]
struct App {
    // components: Vec::<Box<dyn Component>>,
    components: Vec<RefCell<Box<dyn Component>>>,
}
impl Default for App {
    fn default() -> Self {
        Self {
            // components: Default::default()
            // components: Vec::<Box<dyn Component>>::new(),
            // components: Vec::<Rc<RefCell<dyn Component>>>::new(),
            components: Vec::<RefCell<Box<dyn Component>>>::new(),
        }
    }
}

#[test]
fn test_components() {
    let mut app = App::default();
    // let db = Box::new(MyDb::default());
    let mut db = RefCell::new(Box::new(MyDb::default()));
    app.components.push(db);

    app.components.into_iter().for_each(|component| {
        let mut c = component.borrow_mut();
        println!("{:?}", c.name());
        if let Some(db) = c.as_any().downcast_ref::<MyDb>() {
            db.do_something();
        }
        if let Some(db) = c.as_mut_any().downcast_mut::<MyDb>() {
            db.do_something2();
        }

        // 向下转型
        // if let Some(db) = component.as_any().downcast_mut::<MyDb>() {
        //     db.do_something() ;
        // }
    })
}

#[test]
fn test_refcell() {
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    // Create a new block to limit the scope of the dynamic borrow
    {
        let mut map: RefMut<_> = shared_map.borrow_mut();
        map.insert("africa", 92388);
        map.insert("kyoto", 11837);
        map.insert("piccadilly", 11826);
        map.insert("marbles", 38);
    }
    // Note that if we had not let the previous borrow of the cache fall out
    // of scope then the subsequent borrow would cause a dynamic thread panic.
    // This is the major hazard of using `RefCell`.
    let total: i32 = shared_map.borrow().values().sum();
    println!("{}", total);
}


// @see https://www.coder.work/article/7509333 将Ref<Box<dyn Any>> 向下转换为 Ref<Box<T>> 时处理向下转换错误 
// https://stackoverflow.com/questions/64310019/handle-downcast-error-when-downcasting-refboxdyn-any-into-refboxt
pub fn foo<T: 'static>(cell: &RefCell<Box<dyn Any>>) -> Option<Ref<T>> {
    let borrowed = cell.borrow();
    
    if borrowed.is::<T>() {
        Some(Ref::map(borrowed, |x| x.downcast_ref::<T>().unwrap()))
    } else {
        None
    }
}
