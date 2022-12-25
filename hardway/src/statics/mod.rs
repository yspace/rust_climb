//  静态变量经常配合laze_static使用

// 对于 static ，Rust以静态量的方式提供了类似“全局变量”的功能。它们与常量类似，不过静态量在使用时并不内联。这意味着对每一个值只有一个实例，并且位于内存中的固定位置。

// 参考：https://www.codercto.com/a/89622.html

static N: i32 = 5; 
static NAME: &'static str = "Steve"; //静态量贯穿于整个程序的生命周期，因此任何存储在常量中的引用有一个'static生命周期 //因为这是可变的，一个线程可能在更新N同时另一个在读取它，导致内存不安全。 //因此访问和改变一个static mut是不安全（unsafe）的，因此必须在unsafe块中操作 static mut NUM: i32 = 10; unsafe { NUM=NUM+1; }

//因为这是可变的，一个线程可能在更新N同时另一个在读取它，导致内存不安全。
 //因此访问和改变一个static mut是不安全（unsafe）的，因此必须在unsafe块中操作 
 static mut NUM: i32 = 10;
 


static V: Vec<u8> = Vec::new();
pub fn main() {
    println!("hello static");

    unsafe { NUM=NUM+1; }
    unsafe{ 

        println!("{}", NUM);
    } 
}