mod future_internal ;
mod using_tokio ;
mod tokio_actors ;
mod async_futures ;

mod async_raw_sample;

mod async_std_book;

pub fn main() {
    println!("in mod ayncs");
    async_std_book::main();

    async_raw_sample::main();
    //
    // basic::run();
    // using_await::run();
    // sing_and_song::main();
    // sing_song_v2::run() ;

    // using_tokio::run();
}

mod basic {
    pub fn run() {
        use futures::executor::block_on;

        let f = do_something();
        block_on(f);
    }

    async fn do_something() {
        println!("go go go ");
    }
}

mod using_await {
    pub fn run() {
        println!("using await ");

        use futures::executor::block_on;

        let f = hello_world();
        block_on(f);
    }

    async fn hello_world() {
        hello_cat().await;
        println!("hello world");
    }
    async fn hello_cat() {
        println!("hello kitty");
    }
}

mod sing_and_song {
    struct Song;

    async fn learn_song() -> Song {

        Song {} /* ... */
    }
    async fn sing_song(song: Song) { /* ... */
    }
    async fn dance() { /* ... */
    }

    use futures::executor::block_on;
    pub fn main() {
        let song = block_on(learn_song());
        block_on(sing_song(song));
        block_on(dance());
    }
}

mod sing_song_v2{
    use futures::executor::block_on;
    pub fn run() {
        block_on(async_main());
    }
    struct Song ;
    async fn learn_song() -> Song {

        Song {} /* ... */
    }
    async fn sing_song(song: Song) {  /* ... */ }
async fn learn_and_sing() {
    // 这里使用`.await`来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务`.await`后，完全可以去执行跳舞的任务
    let song = learn_song().await;
    
    // 唱歌必须要在学歌之后
    sing_song(song).await;
}

async fn dance(){}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!`可以并发的处理和等待多个`Future`，若`learn_and_sing Future`被阻塞，那`dance Future`可以拿过线程的所有权继续执行。若`dance`也变成阻塞状态，那`learn_and_sing`又可以再次拿回线程所有权，继续执行。
    // 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，并将其交给`main`函数中的`block_on`执行器
    futures::join!(f1, f2);
}


}