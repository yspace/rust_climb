
use std::collections::HashMap;
#[derive(Default)]
pub struct Runner {
    // 参考：https://github.com/redox-os/orbtk/.../crates/api/src/widget_base/registry.rs
    funcs: HashMap<  String ,  Box<dyn Fn()> >
}

impl Runner{
    /// Creates a service registry with an empty Registry map.
    pub fn new() -> Self {
        Self::default()
    }
    pub fn init(){

    }
    pub fn run(){

    }

    pub fn add_case(&mut self, key: impl Into<String>, func:   Box<dyn Fn()> ){
        self.funcs.insert(key.into(), func) ;
    }
    pub fn get_case(&self, key: &str) -> Option< &Box<dyn Fn() > > {
        let func = self.funcs
            .get(&key.to_string()) ;
//            .unwrap_or_else(|| panic!("Registry.get(): key: {} could not be found.", key))

        match func {
            Some(f) =>  Some(f),
            None => None ,
        }

    }
}


// cargo test -p whale
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn runner_get() {
        let mut runner = Runner::new();
        fn one(){
            println!("one called") ;
        }
        runner.add_case("one", Box::new(one)) ;

        assert!(runner.get_case ("one").is_some());
        //  runner.get_case("one").unwrap()() ;
    }

}