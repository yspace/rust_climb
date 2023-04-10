/*

trait Drop {
    fn drop(&mut self);
}

//  it's useful is if a type holds on to some external resources which needs to be cleaned up when the type is destroyed.

impl<W: Write> Drop for BufWriter<W> {
    fn drop(&mut self) {
        self.flush_buf();
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        unsafe {
            self.lock.inner.raw_unlock();
        }
    }
}

In general, if you're impling an abstraction over some resource that needs to be cleaned up after use then that's a great reason to make use of the Drop trait.

 */

struct MyStruct{
    // hold some resources
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("go out of scope! destroy the holded resources ")
    }
}

#[test]
fn it_works() {
    let ms = MyStruct{} ;
    {
        let ms = ms ;
        // 出了作用域 就调用drop了
    }
    println!("done!")
}