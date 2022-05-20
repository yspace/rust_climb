#[tokio::main]
async fn main() {
    // spawn相当于 分化出多个要账的人 async异步代码块相当于账单
    let handle = tokio::spawn(async {
        // Do some async work
        "return value"
    });

    // Do some other work

    let out = handle.await.unwrap();
    println!("GOT {}", out);
}