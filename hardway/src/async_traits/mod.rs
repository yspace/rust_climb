use std::{cell::RefCell, sync::Mutex};

use async_std::{task};
use async_trait::async_trait;
// @see https://blog.csdn.net/DAOSHUXINDAN/article/details/108311793
#[async_trait]
trait Foo {
    async fn foo(&self);
}

struct A;

#[async_trait]
impl Foo for A {
    async fn foo(&self) {
        println!("foo A")
    }
}

fn main() {
    task::block_on(async {
        task::spawn(async {
            let a = A;
            // 直接通过 A 调用 foo 没有问题
            a.foo().await;
            // let o = &a as &dyn Foo;
            // 通过 trait object 编译错误如下

            let o = &a as &(dyn Foo + Sync);

            o.foo().await;
        }).await
    });
}

// 问题的根本原因在于有 !Send 的值跨越 await 导致生成的匿名 Future 也是 !Send 继而无法在线程之间传递
// 如果某结构体内部存在 !Sync 的字段，那么跨越 await 依然有问题，如下所示
struct C(Mutex<RefCell<i32>>);
impl C {
    async fn bar(&self) {
        println!("bar C");
    }
}
#[async_trait]
impl Foo for C {
    async fn foo(&self) {
        // 这里await没事！
        self.bar().await;

        let mut mg = self.0.lock().expect("lock");
        let c = mg.get_mut();
        // 跨越 await 则无法编译通过
        // self.bar().await;


       
        *c = 10;
    }
}


#[test]
fn it_works(){}