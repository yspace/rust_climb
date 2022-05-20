use tokio::task;

pub async fn main() {
    let v = vec![1, 2, 3];

    task::spawn(async move {
        println!("Here's a vec: {:?}", v);
    });
}