// 创建包含不同类型的集合

trait UsbModule{
    fn name(&self) -> String{
        String::from("usb module")
    }
}

struct UsbCamera{}

impl UsbModule for UsbCamera{}
impl UsbCamera{
    pub fn new() -> Self{
        Self{}
    }
}

struct UsbDisk{}
impl UsbModule for UsbDisk{}
impl UsbDisk{
    pub fn new() -> Self{
        Self{}
    }
}

struct UsbMicrophone{}
impl UsbModule for UsbMicrophone{}
impl UsbMicrophone{
    pub fn new() -> Self{
        Self{}
    }
}

// ## Use generic parameters when you need absolute performance and trait objects when you need more flexibility.

// 动态运行期分发
fn print_usb_module(module: &dyn UsbModule){
    let name = module.name();
    println!("{}", name);
}
// 这个是编译期静态分发 涉及到单态化的概念 。好比在编译器针对每种使用到的类型都产生一个版本
fn print_usb_module0<M: UsbModule>(module: &M){
    let name = module.name();
    println!("{}", name);
}

#[test]
fn test_usb_module_name(){
    let mut modules:Vec<Box<dyn UsbModule>> = vec![
       Box::new( UsbMicrophone::new()),
       Box::new( UsbCamera::new()),
       Box::new( UsbDisk::new()),
    ];
    for module in modules{
        print_usb_module(&*module);
    }
}