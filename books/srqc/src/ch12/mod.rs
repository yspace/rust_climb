mod lifetimes ;

pub fn main(){

 borrows::run() ;
 borrows::run2() ;
 borrows::run3() ;

 lifetimes::main() ;
}

mod borrows{
    /**
    对于&mut型指针，请大家注意不要混淆它与变量绑定之间的语
法。如果mut修饰的是变量名，那么它代表这个变量可以被重新绑定；
如果mut修饰的是“借用指针&”，那么它代表的是被指向的对象可以被
修改

借用指针在编译后，实际上就是一个普通的指针，它的意义只能在
编译阶段的静态检查中体现。
    */

    pub fn run(){
        let mut var = 0_i32 ;
        {
            let p1 = &mut var; // p1 指针本身不能被重新绑定,我们可以通过p1改变变量var的值
            *p1 = 1;
        }
        {
            let temp = 2_i32 ;
            let mut p2 = &var ; // 我们不能通过p2改变变量var的值,但p2指针本身指向的位置可以被改变
            p2 = &temp ;
        }
        {
            let mut temp = 3_i32 ;
            let mut p3 = &mut var ; //我们既可以通过p3改变变量var的值,而且p3指针本身指向的位置也可以改变

            *p3 = 3 ;
            p3 = &mut temp ;
        }

    }

    // 这里的参数采用的“引用传递”,意味着实参本身并未丢失对内存的管理权
fn borrow_semantics(v : &Vec<i32>) {
    // 打印参数占用空间的大小,在64位系统上,结果为8,表明该指针与普通裸指针的内部表示方法相同
    println!("size of param: {}", std::mem::size_of::<&Vec<i32>>());
    for item in v {
        print!("{} ", item);
    }
    println!("");
}

// 这里的参数采用的“值传递”,而Vec没有实现Copy trait,意味着它将执行move语义
fn move_semantics(v : Vec<i32>) {
    // 打印参数占用空间的大小,结果为24,表明实参中栈上分配的内存空间复制到了函数的形参中
    println!("size of param: {}", std::mem::size_of::<Vec<i32>>());
    for item in v {
        print!("{} ", item);
    }
    println!("");
}

pub fn run2(){
    let array = vec![1, 2, 3];
    // 需要注意的是,如果使用引用传递,不仅在函数声明的地方需要使用&标记
    // 函数调用的地方同样需要使用&标记,否则会出现语法错误
    // 这样设计主要是为了显眼,不用去阅读该函数的签名就知道这个函数调用的时候发生了什么
    // 而小数点方式的成员函数调用,对于self参数,会“自动转换”,不必显式借用,这里有个区别
    borrow_semantics(&array);
    // 在使用引用传递给上面的函数后,array本身依然有效,我们还能在下面的函数中使用
    move_semantics(array);
    // 在使用move语义传递后,array在这个函数调用后,它的生命周期已经完结
}

pub fn run3(){
    // 创建了一个可变的 String 类型实例
    let mut x : String = "hello".into();
    // 调用 len(&self) -> usize 函数。 self的类型是 &Self
    // x.len() 等同于 String::len(&x)
    println!("length of String {}", x.len());
    // 调用fn push(&mut self, ch: char) 函数。self的类型是 &mut Self,因此它有权对字符串做
// 修改
    // x.push('!') 等同于 String::push(&mut x, '!')
 x.push('!');
    println!("length of String {}", x.len());
    // 调用 fn into_bytes(self) -> Vec<u8> 函数。注意self的类型,此处发生了所有权转移
    // x.into_bytes() 等同于 String::into_bytes(x)
    let v = x.into_bytes();
    // 再次调用len(),编译失败,因为此处已经超过了 x 的生命周期
    //println!("length of String {}", x.len());
}

}