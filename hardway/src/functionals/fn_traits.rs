// Fn     Fn(&self)  可以重复调用
// FnMut     Fn(&mut self)  可以重复调用
// FnOnce     FnOnce(self)  只能调用一次

mod captures{

    fn is_fn<F>(_:F ) where F: Fn(){}
    fn is_fn_once<F>(_:F ) where F: FnOnce()->String{}
    

    #[test]
    fn fn_once(){
        let consumable = String::from("cookie");
        // 根据变量捕获性质 实现不同的trait
        let consumer = move || consumable ;

        // consumer(); // 只能被调用一次
        is_fn_once(consumer);
    }
}