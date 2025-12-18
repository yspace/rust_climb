use crate::TargetSite;



pub struct SiteChanliu;

#[async_trait::async_trait]
impl  TargetSite   for SiteChanliu{
    fn get_name(&self) -> String {
        String::from("chanliu")
    }
    fn get_title(&self) -> String {
        String::from("长流")
    }
}

#[test]
fn it_works() {
    use futures::executor::block_on;

    async fn dance() {
        println!("Dancing along to the song");
    }
    //
    // let future = hello_world();
    let future = dance();
    block_on(future);


}